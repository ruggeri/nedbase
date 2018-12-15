use btree::BTree;
use locking::{LockSet, LockSetNodeReadGuard};

impl BTree {
  pub fn contains_key(lock_set: &mut LockSet, key: &str) -> bool {
    let guard = BTree::find_leaf_for_key(lock_set, key);
    let node = guard
      .unwrap_leaf_node_ref("find_leaf_for_key must return leaf node");

    node.contains_key(key)
  }

  pub fn find_leaf_for_key(
    lock_set: &mut LockSet,
    key: &str,
  ) -> LockSetNodeReadGuard {
    // We will only assign to _parent_guard, but it is important to hold
    // onto.
    //
    // We descend down the tree, hand-over-hand, taking temporary read
    // locks.
    let (mut _parent_guard, mut current_node_guard) = {
      let root_identifier_guard =
        lock_set.root_identifier_read_guard_for_temp();
      let mut current_node_guard = lock_set
        .node_read_guard_for_temp(&root_identifier_guard.identifier());

      (root_identifier_guard.upcast(), current_node_guard)
    };

    loop {
      // If we hit a LeafNode we are done descending.
      if current_node_guard.unwrap_node_ref().is_leaf_node() {
        break;
      }

      // Drop the parent guard before trying to acquiring the next child
      // on the path.
      _parent_guard.release();

      let child_guard = {
        let current_node = current_node_guard.unwrap_interior_node_ref(
          "must not try to descend through leaf node",
        );
        let child_identifier =
          current_node.child_identifier_by_key(key);

        lock_set.node_read_guard_for_temp(child_identifier)
      };

      _parent_guard = current_node_guard.upcast();
      current_node_guard = child_guard;
    }

    // Now that we are all finished descending, we will release the
    // temporary read guard on the leaf, and reacquire. This is
    // important if we are doing a lookup in a ReadWrite transaction.
    //
    // We could probably search again in the parent for this identifier
    // without doing a String::from. But this seems okay.
    let target_identifier =
      String::from(current_node_guard.unwrap_node_ref().identifier());
    current_node_guard.release();
    lock_set.node_read_guard_for_hold(&target_identifier)
  }
}
