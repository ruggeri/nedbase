use super::{
  acquire_parent_of_deepest_stable_node, DeletionPath,
  DeletionPathEntry, UnderflowAction, WriteSet,
};
use btree::BTree;
use locking::ReadGuard;
use std::sync::Arc;

pub fn acquire_deletion_path(
  btree: &Arc<BTree>,
  key_to_delete: &str,
) -> (DeletionPath, WriteSet) {
  // Acquire a write lock on the deepest stable node (or root identifier
  // if there is no stable node).
  let (mut deletion_path, mut write_set) = loop {
    let mut write_set = WriteSet::new();
    let deletion_path =
      begin_deletion_path(btree, &mut write_set, key_to_delete);

    if deletion_path.is_some() {
      // Hopefully the deepest stable node stayed stable! Then we can
      // continue.
      break (deletion_path.unwrap(), write_set);
    }

    // The deepest stable node may go unstable due to simultaneous
    // delete, which means we must try everything again and loop back
    // around.
  };

  loop {
    // We descend until we hit a leaf, acquiring nodes along the way.
    if deletion_path.last_node_ref(&write_set).is_leaf_node() {
      break;
    }

    extend_deletion_path(
      btree,
      &mut deletion_path,
      &mut write_set,
      key_to_delete,
    );
  }

  (deletion_path, write_set)
}

// Starts the deletion path by acquiring the topmost write guard.
fn begin_deletion_path(
  btree: &Arc<BTree>,
  write_set: &mut WriteSet,
  key_to_delete: &str,
) -> Option<DeletionPath> {
  // The first step is to acquire a read guard of the parent above our
  // target stable node.
  let parent_guard =
    match acquire_parent_of_deepest_stable_node(btree, key_to_delete) {
      None => {
        // However, the root itself may be unstable for deletion. In that
        // case, we may be merging a new root!
        return Some(DeletionPath::new_from_unstable_root(
          btree, write_set,
        ));
      }

      // Normally, there is *some* stable node. We unwrap its parent.
      Some(parent_guard) => parent_guard,
    };

  // We write lock the stable node.
  let deletion_path = match parent_guard {
    // If the deepest stable node is the root, then parent_guard will be
    // a read guard on the root_identifier.
    ReadGuard::RootIdentifierReadGuard(root_identifier_guard) => {
      DeletionPath::new_from_stable_parent(
        btree,
        write_set,
        root_identifier_guard.as_str_ref(),
      )
    }

    // Typically, the deepest stable node is not the root. In which
    // case, we have read locked its parent. Here we take the write lock
    // on the child, releasing the read lock on the parent.
    ReadGuard::NodeReadGuard(parent_node_guard) => {
      let child_identifier = parent_node_guard
        .unwrap_interior_node_ref("a parent must be an interior node")
        .child_identifier_by_key(key_to_delete);

      DeletionPath::new_from_stable_parent(
        btree,
        write_set,
        child_identifier,
      )
    }
  };

  // We must check: did the target node go unstable on us? If that
  // happened, we will have to start everything again...
  let last_node_of_path = deletion_path.last_node_ref(write_set);

  if !last_node_of_path.can_delete_without_becoming_deficient() {
    None
  } else {
    Some(deletion_path)
  }
}

// We extend our path by locking a new path node, and also a sibbling to
// merge with (or rotate with).
fn extend_deletion_path(
  btree: &Arc<BTree>,
  path: &mut DeletionPath,
  write_set: &mut WriteSet,
  key_to_delete: &str,
) {
  // Get identifiers of parent/child/sibbling.
  let (
    parent_node_identifier,
    child_node_identifier,
    sibbling_node_identifiers,
  ) = {
    // Who is the parent?
    let parent_node = path
      .last_node_ref(write_set)
      .unwrap_interior_node_ref("must not descend through leaves");
    let parent_node_identifier = String::from(parent_node.identifier());

    // Who is the child? Acquire it.
    let child_idx = parent_node.child_idx_by_key(key_to_delete);
    let child_node_identifier =
      String::from(parent_node.child_identifier_by_idx(child_idx));

    // Who are the sibblings?
    let sibbling_node_identifiers =
      parent_node.sibbling_identifiers_for_idx(child_idx);
    let sibbling_node_identifiers = (
      sibbling_node_identifiers.0.map(String::from),
      sibbling_node_identifiers.1.map(String::from),
    );

    (
      parent_node_identifier,
      child_node_identifier,
      sibbling_node_identifiers,
    )
  };

  // Acquire the child node.
  write_set.acquire_node_guard(btree, &child_node_identifier);

  // Which is the sibbling to merge with? Acquire it.
  let merge_sibbling_identifier =
    acquire_sibbling_node(btree, write_set, sibbling_node_identifiers);

  // Create a DeletionPathEntry saying to do a merge.
  let path_entry = DeletionPathEntry::UnstableNode {
    underflow_action: UnderflowAction::MergeWithSibbling {
      parent_node_identifier,
      sibbling_node_identifier: merge_sibbling_identifier,
    },

    path_node_identifier: child_node_identifier,
  };

  // And store it in on the path.
  path.push(path_entry);
}

fn acquire_sibbling_node(
  btree: &Arc<BTree>,
  write_set: &mut WriteSet,
  sibbling_node_identifiers: (Option<String>, Option<String>),
) -> String {
  match sibbling_node_identifiers {
    (None, None) => {
      panic!("non-root node should never have no sibblings")
    }

    // If there is only one sibbling, we don't get a choice.
    (Some(sibbling_node_identifier), None)
    | (None, Some(sibbling_node_identifier)) => {
      write_set.acquire_node_guard(btree, &sibbling_node_identifier);
      sibbling_node_identifier
    }

    (
      Some(left_sibbling_node_identifier),
      Some(right_sibbling_node_identifier),
    ) => {
      // I put this in a scope because I don't want to retain the
      // reference to the guard when I do operations to acquire the
      // right sibbling below.
      //
      // Prefer rotating from the left node. Probably fine.
      {
        let left_sibbling_guard = write_set
          .acquire_node_guard(btree, &left_sibbling_node_identifier);

        if left_sibbling_guard.can_delete_without_becoming_deficient() {
          return left_sibbling_node_identifier;
        }
      }

      // Don't forget to release the guard in the write set.
      //
      // TODO: This is the first place I think I can't rely on RAII.
      // Hmm...
      write_set.drop_node_guard(&left_sibbling_node_identifier);

      // If you can't rotate from the left sibbling, try using the right
      // sibbling. Prefer merging from the right.
      write_set.drop_node_guard(&left_sibbling_node_identifier);
      write_set
        .acquire_node_guard(btree, &right_sibbling_node_identifier);
      right_sibbling_node_identifier
    }
  }
}
