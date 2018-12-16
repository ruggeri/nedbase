use super::{DeletionAction, DeletionPath};
use locking::{LockSet, LockSetNodeWriteGuard};

// This is a helper that builds a DeletionPath. When building a
// DeletionPath, it helps to remember who your parent node was.

pub struct DeletionPathBuilder {
  // `current_node` is the last node on the DeletionPath to have been
  // pushed.
  current_node: LockSetNodeWriteGuard,
  path: DeletionPath,
}

impl DeletionPathBuilder {
  // Build a DeletionPathBuilder. When we are unstable all the way to
  // the root, the top action will be to do an update on the root
  // identifier.
  pub fn new_from_unstable_root(
    lock_set: &mut LockSet,
  ) -> DeletionPathBuilder {
    // Acquire locks for the root identifier and the current root node.
    let root_identifier_guard = lock_set.root_identifier_write_guard();
    let root_node_guard = lock_set
      .node_write_guard(&root_identifier_guard.identifier());

    // We start out with a DeletionAction for updating the root
    // identifier.
    let path = DeletionPath {
      actions: vec![DeletionAction::update_root_identifier(
        root_identifier_guard,
        root_node_guard.clone(),
      )],
    };

    DeletionPathBuilder {
      current_node: root_node_guard,
      path,
    }
  }

  // Build a DeletionPathBuilder when there is a top stable node.
  pub fn new_from_stable_parent(
    lock_set: &mut LockSet,
    stable_ancestor_identifier: &str,
  ) -> Option<DeletionPathBuilder> {
    // First, acquire a guard on the stable ancestor.
    let stable_node_guard =
      lock_set.node_write_guard(stable_ancestor_identifier);

    // We must check: did the target node go unstable on us? If that
    // happened, we will have to start everything again...
    let is_still_stable = {
      stable_node_guard
        .unwrap_node_ref()
        .can_delete_without_becoming_deficient()
    };

    if !is_still_stable {
      return None;
    }

    // If the top stable node is still stable, then it is the
    // "current_node." At first there is not action in the path. But
    // when we do push the first action, it can refer to `current_node`
    // as its parent.
    let builder = DeletionPathBuilder {
      current_node: stable_node_guard,
      path: DeletionPath { actions: vec![] },
    };

    Some(builder)
  }

  // We extend our path by locking a new path node, and also a sibbling
  // to merge with (or rotate with).
  pub fn extend_deletion_path(
    &mut self,
    lock_set: &mut LockSet,
    key_to_delete: &str,
  ) {
    // Who is the parent? It's the current node. Note: we'll replace the
    // current_node again by the end.
    //
    // TODO: This is a little inefficient, because we actually only need
    // to *move* the parent_node_guard into the new DeletionAction we
    // create...
    let parent_node_guard = self.current_node.clone();

    // Who is the child? Acquire it.
    let (child_idx, child_node_guard) = {
      let parent_node = self
        .current_node
        .unwrap_interior_node_ref("must not descend through leaves");

      let child_idx = parent_node.child_idx_by_key(key_to_delete);
      let child_node_identifier =
        parent_node.child_identifier_by_idx(child_idx);
      let child_node_guard =
        lock_set.node_write_guard(&child_node_identifier);

      (child_idx, child_node_guard)
    };

    // Who are the sibblings? Which one should we merge with?
    let sibbling_node_guard = {
      let parent_node = parent_node_guard
        .unwrap_interior_node_ref("must not descend through leaves");
      let sibbling_node_identifiers =
        parent_node.sibbling_identifiers_for_idx(child_idx);

      // Which is the sibbling to merge with? Acquire it.
      self.acquire_sibbling_node(lock_set, sibbling_node_identifiers)
    };

    // Create a DeletionAction saying to do a merge.
    let action = DeletionAction::merge_with_sibbling(
      parent_node_guard,
      child_node_guard.clone(),
      sibbling_node_guard,
    );

    // And store it in on the path.
    self.path.push_action(action);

    // Last, update the current_node.
    self.current_node = child_node_guard;
  }

  // This acquires a lock on the sibbling node to merge with.
  fn acquire_sibbling_node(
    &mut self,
    lock_set: &mut LockSet,
    sibbling_node_identifiers: (Option<&str>, Option<&str>),
  ) -> LockSetNodeWriteGuard {
    match sibbling_node_identifiers {
      (None, None) => {
        panic!("non-root node should never have no sibblings")
      }

      // If there is only one sibbling, we don't get a choice. We must
      // merge/rotate with that sibbling.
      (Some(sibbling_node_identifier), None)
      | (None, Some(sibbling_node_identifier)) => {
        lock_set.node_write_guard(&sibbling_node_identifier)
      }

      (
        Some(left_sibbling_node_identifier),
        Some(right_sibbling_node_identifier),
      ) => {
        // I put this in a scope because I don't want to retain the
        // reference to the left sibbling's guard when I do operations
        // to acquire the right sibbling below.
        //
        // Prefer rotating from the left node. Probably fine to be
        // biased like this.
        {
          let left_sibbling_guard = lock_set
            .node_write_guard(&left_sibbling_node_identifier);

          // If the left sibbling has spare keys, rotate them to us.
          if left_sibbling_guard
            .unwrap_node_ref()
            .can_delete_without_becoming_deficient()
          {
            return left_sibbling_guard;
          }
        }

        // If you can't rotate from the left sibbling, try using the
        // right sibbling. Note: while we prefer the left sibbling for
        // rotation, we prefer the right sibbling for merging.
        lock_set
          .node_write_guard(&right_sibbling_node_identifier)
      }
    }
  }

  // When done, we discard the builder and keep the path it has built.
  pub fn finish(mut self, key_to_delete: &str) -> DeletionPath {
    // One last thing to do! Add a final action for deleting the key at
    // the LeafNode!
    self.path.push_action(DeletionAction::delete_key_from_node(
      self.current_node,
      key_to_delete,
    ));

    self.path
  }

  // This lets us know when we are done!
  pub fn hit_leaf_node(&self) -> bool {
    self.current_node.unwrap_node_ref().is_leaf_node()
  }
}
