use super::{DeletionPath, DeletionPathEntry};
use btree::deletion::acquire_parent_of_deepest_stable_node;
use locking::{LockSet, LockSetNodeWriteGuard};

pub fn acquire_deletion_path(
  lock_set: &mut LockSet,
  key_to_delete: &str,
) -> DeletionPath {
  // Acquire a write lock on the deepest stable node (or root identifier
  // if there is no stable node).
  let mut deletion_path = loop {
    let deletion_path = begin_deletion_path(lock_set, key_to_delete);

    if deletion_path.is_some() {
      // Hopefully the deepest stable node stayed stable! Then we can
      // continue.
      break deletion_path.unwrap();
    }

    // The deepest stable node may go unstable due to simultaneous
    // delete, which means we must try everything again and loop back
    // around.
  };

  loop {
    // We descend until we hit a leaf, acquiring nodes along the way.
    if deletion_path.last_node_ref().is_leaf_node() {
      break;
    }

    extend_deletion_path(lock_set, &mut deletion_path, key_to_delete);
  }

  deletion_path
}

// Starts the deletion path by acquiring the topmost write guard.
fn begin_deletion_path(
  lock_set: &mut LockSet,
  key_to_delete: &str,
) -> Option<DeletionPath> {
  // The first step is to acquire a read guard of the parent above our
  // target stable node.
  let parent_guard = match acquire_parent_of_deepest_stable_node(
    lock_set,
    key_to_delete,
  ) {
    None => {
      // However, the root itself may be unstable for deletion. In that
      // case, we may be merging a new root!
      return Some(DeletionPath::new_from_unstable_root(lock_set));
    }

    // Normally, there is *some* stable node. We unwrap its parent.
    Some(parent_guard) => parent_guard,
  };

  // We write lock the stable node.
  let deletion_path = match parent_guard.downcast() {
    // If the deepest stable node is the root, then parent_guard will be
    // a read guard on the root_identifier.
    (Some(root_identifier), None) => {
      DeletionPath::new_from_stable_parent(lock_set, &root_identifier)
    }

    // Typically, the deepest stable node is not the root. In which
    // case, we have read locked its parent. Here we take the write lock
    // on the child, releasing the read lock on the parent.
    (None, Some(parent_node)) => {
      let child_identifier = parent_node
        .unwrap_interior_node_ref("a parent must be an interior node")
        .child_identifier_by_key(key_to_delete);

      DeletionPath::new_from_stable_parent(lock_set, child_identifier)
    }

    _ => {
      panic!("Every guard must be for a RootIdentifier or a Node...")
    }
  };

  // We must check: did the target node go unstable on us? If that
  // happened, we will have to start everything again...
  let is_still_stable = {
    let last_node_guard_of_path = deletion_path.last_node_guard_ref();
    last_node_guard_of_path
      .unwrap_node_ref()
      .can_delete_without_becoming_deficient()
  };

  if is_still_stable {
    Some(deletion_path)
  } else {
    None
  }
}

// We extend our path by locking a new path node, and also a sibbling to
// merge with (or rotate with).
fn extend_deletion_path(
  lock_set: &mut LockSet,
  path: &mut DeletionPath,
  key_to_delete: &str,
) {
  // Who is the parent? We clone this guard because we will store it
  // separately in the new PathEntry we will create.
  let parent_node_guard = path.last_node_guard_ref().clone();

  // Who is the child? Acquire it.
  let (child_idx, child_node_guard) = {
    let parent_node = parent_node_guard
      .unwrap_interior_node_ref("must not descend through leaves");

    let child_idx = parent_node.child_idx_by_key(key_to_delete);
    let child_node_identifier =
      parent_node.child_identifier_by_idx(child_idx);
    let child_node_guard =
      lock_set.node_write_guard_for_hold(&child_node_identifier);

    (child_idx, child_node_guard)
  };

  // Who are the sibblings? Which one should we merge with?
  let sibbling_node_guard = {
    let parent_node = parent_node_guard
      .unwrap_interior_node_ref("must not descend through leaves");
    let sibbling_node_identifiers = parent_node
      .sibbling_identifiers_for_idx(child_idx);

    // Which is the sibbling to merge with? Acquire it.
    acquire_sibbling_node(lock_set, sibbling_node_identifiers)
  };

  // Create a DeletionPathEntry saying to do a merge.
  let path_entry = DeletionPathEntry::new_merge_with_sibbling_entry(
    parent_node_guard,
    child_node_guard,
    sibbling_node_guard,
  );

  // And store it in on the path.
  path.push(path_entry);
}

fn acquire_sibbling_node(
  lock_set: &mut LockSet,
  sibbling_node_identifiers: (Option<&str>, Option<&str>),
) -> LockSetNodeWriteGuard {
  match sibbling_node_identifiers {
    (None, None) => {
      panic!("non-root node should never have no sibblings")
    }

    // If there is only one sibbling, we don't get a choice.
    (Some(sibbling_node_identifier), None)
    | (None, Some(sibbling_node_identifier)) => {
      lock_set.node_write_guard_for_hold(&sibbling_node_identifier)
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
        let left_sibbling_guard = lock_set
          .node_write_guard_for_hold(&left_sibbling_node_identifier);

        if left_sibbling_guard
          .unwrap_node_ref()
          .can_delete_without_becoming_deficient()
        {
          return left_sibbling_guard;
        }
      }

      // Since you can't rotate from the left sibbling, try using the
      // right sibbling. Prefer merging from the right.
      lock_set
        .node_write_guard_for_hold(&right_sibbling_node_identifier)
    }
  }
}
