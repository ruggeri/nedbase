use super::acquire_parent_of_deepest_stable_node;
use locking::{LockSet, LockSetReadGuard, WriteGuardPath};

enum WriteGuardPathAcquisitionResult {
  Success(WriteGuardPath),
  TopNodeWentUnstable,
}

// Acquire a path of write guards starting from deepest stable node all
// the way down to a LeafNode. Because this can fail if the target top
// node becomes unstable, we loop until success.
pub fn acquire_write_guard_path(
  lock_set: &mut LockSet,
  insert_key: &str,
) -> WriteGuardPath {
  loop {
    // Note: parent_of_stable_node might be None if we are splitting the
    // root.
    let parent_of_stable_node =
      acquire_parent_of_deepest_stable_node(lock_set, insert_key);

    // Now try to acquire the WriteGuardPath. Note that this will
    // release the read lock on the parent (if any).
    let write_guard_acquisition_result =
      try_to_acquire_write_guard_path(
        lock_set,
        parent_of_stable_node,
        insert_key,
      );

    match write_guard_acquisition_result {
      WriteGuardPathAcquisitionResult::TopNodeWentUnstable => {
        // The deepest stable node may go unstable due to simultaneous
        // insert, which means we must try everything again.
        continue;
      }

      WriteGuardPathAcquisitionResult::Success(write_guard_path) => {
        // Hopefully the deepest stable node stayed stable! Then we can
        // continue.
        return write_guard_path;
      }
    }
  }
}

// Acquire a path of write guards starting from deepest stable node all
// the way down to a LeafNode. This may fail.
fn try_to_acquire_write_guard_path(
  lock_set: &mut LockSet,
  parent_read_guard: Option<LockSetReadGuard>,
  insert_key: &str,
) -> WriteGuardPathAcquisitionResult {
  // First, try to acquire that top write guard.
  let write_guards = try_to_acquire_top_write_guard(
    lock_set,
    parent_read_guard,
    insert_key,
  );

  // Next, check if we were able to acquire the top write guard.
  let mut write_guards = match write_guards {
    WriteGuardPathAcquisitionResult::Success(write_guards) => {
      write_guards
    }
    // Propagate any error back to the parent.
    error => return error,
  };

  // Descend acquiring write guards.
  loop {
    let child_guard = {
      let last_node = write_guards
        .peek_deepest_lock(
          "write_guards starts with a Node and we only add more",
        )
        .unwrap_node_ref(
          "last guard in lock path should always be for a Node",
        );

      if last_node.is_leaf_node() {
        break;
      }

      let child_identifier = last_node
        .unwrap_interior_node_ref(
          "should be descending through InteriorNode",
        )
        .child_identifier_by_key(insert_key);

      lock_set.node_write_guard_for_hold(child_identifier)
    };

    // TODO: If the node we just locked is stable we can release the
    // prior locks. In that case we were just slightly overly
    // pessimistic.
    write_guards.push(child_guard.upcast());
  }

  WriteGuardPathAcquisitionResult::Success(write_guards)
}

// Attemps to acquire a write guard on the deepest stable ancestor.
fn try_to_acquire_top_write_guard(
  lock_set: &mut LockSet,
  parent_read_guard: Option<LockSetReadGuard>,
  insert_key: &str,
) -> WriteGuardPathAcquisitionResult {
  let mut write_guards = WriteGuardPath::new();

  match parent_read_guard {
    None => {
      // There may be no parent_read_guard because we may have to split
      // all the way through the root.
      let root_identifier_guard =
        lock_set.root_identifier_write_guard_for_hold();
      let root_node_guard = lock_set
        .node_write_guard_for_hold(&root_identifier_guard.identifier());
      write_guards.push(root_identifier_guard.upcast());
      write_guards.push(root_node_guard.upcast());
    }

    Some(parent_read_guard) => {
      // If there is a stable node, acquire a write lock on it. We have
      // a read lock on its "parent": either a parent node in the tree,
      // or if the lowest stable node is the root, then at least the
      // root identifier.
      //
      // Note that we take ownership of the parent read guard here, so
      // it will be unlocked after write guard is acquired.
      let deepest_stable_parent = match parent_read_guard.downcast() {
        (Some(root_identifier), None) => {
          lock_set.node_write_guard_for_hold(&root_identifier)
        }

        (None, Some(parent_node)) => {
          let child_identifier = parent_node
            .unwrap_interior_node_ref(
              "a parent node must be an interior node",
            )
            .child_identifier_by_key(insert_key);

          lock_set.node_write_guard_for_hold(child_identifier)
        }

        _ => panic!("Guards are always for a RootIdentifier or a Node"),
      };

      if !deepest_stable_parent.node().can_grow_without_split() {
        // It is possible that because of concurrent insertions, this
        // node is no longer stable. Then we must start all over again!
        return WriteGuardPathAcquisitionResult::TopNodeWentUnstable;
      }

      write_guards.push(deepest_stable_parent.upcast());
    }
  }

  WriteGuardPathAcquisitionResult::Success(write_guards)
}
