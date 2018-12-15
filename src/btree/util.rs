use locking::{
  LockSet, LockSetReadGuard, ReadGuardPath,
};
use node::Node;

// Finds highest lock target that may need to be mutated by an
// operation.
pub fn acquire_parent_of_deepest_node_meeting_test<F>(
  lock_set: &mut LockSet,
  key: &str,
  stability_check: F,
) -> Option<LockSetReadGuard>
where
  F: Fn(&Node) -> bool,
{
  let mut read_guards = ReadGuardPath::new();

  // Acquire read lock on root identifier, and then on the root node.
  {
    let identifier_guard = lock_set.root_identifier_read_guard_for_temp();
    let current_node_guard =
      lock_set.node_read_guard_for_temp(&identifier_guard.identifier());

    read_guards.push(identifier_guard.upcast());
    read_guards.push(current_node_guard.upcast());
  }

  // Now descend, taking read locks hand-over-hand.
  loop {
    let current_node_guard = {
      // Look at the end of our path. Is it a leaf node? Then we can
      // break and wrap things up.
      let node_read_guard = read_guards
        .peek_deepest_lock(
          "since we break at LeafNode, should not run out of locks",
        )
        .unwrap_node_ref(
          "final read guard in path should always be for a node",
        );

      if node_read_guard.is_leaf_node() {
        break;
      }

      // Otherwise we must continue to descend.
      let child_identifier = node_read_guard
        .unwrap_interior_node_ref(
          "should be descending through InteriorNode",
        )
        .child_identifier_by_key(key);
      lock_set.node_read_guard_for_hold(child_identifier)
    };

    // Whenever we encounter a stable node, we can clear all but the
    // last two locks.
    //
    // Why all but the last two? Eventually we will temporarily release
    // the read lock on the target stable node, so that we can try to
    // reacquire a write lock.
    //
    // Holding a read lock on its parent means that the target of the
    // write lock will still be where the value should live.
    if stability_check(&(*current_node_guard.node())) {
      // See how I shuffle the parent guard? Ugh.
      let last_guard =
        read_guards.pop("should never run out of read locks");
      read_guards.clear();
      read_guards.push(last_guard);
    }

    // Regardless whether we have encountered a stable node, we will
    // continue to hold this lock.
    read_guards.push(current_node_guard.upcast());
  }

  // By the end, we have >= than two locks. We need at most one lock at
  // the end.
  //
  // * If there is any stable node encountered, we want to hold the
  //   parent (so the stable node doesn't move on us), and reacquire the
  //   write lock on the stable node. Note: the "parent" may be the root
  //   identifier lock.
  //
  // * If there are no stable nodes, we'll have to acquire the root
  //   identifier lock *for writing*. There is no need to hold any read
  //   lock anymore.

  // First, drop all but the top two locks. Unpack those.
  read_guards.truncate(2);
  let node_guard = read_guards
    .pop("expected at least root node guard here...");
  let node = node_guard
    .unwrap_node_ref(
      "second lock should never be a root identifier lock",
    );
  let parent_guard =
    read_guards.pop("should always have had at least two guards");

  if stability_check(&node) {
    Some(parent_guard)
  } else {
    None
  }
}
