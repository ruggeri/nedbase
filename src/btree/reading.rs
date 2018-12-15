use btree::BTree;
use locking::{LockSet, LockSetNodeReadGuard};

impl BTree {
  pub fn contains_key(lock_set: &mut LockSet, key: &str) -> bool {
    let guard = BTree::find_leaf_for_key(lock_set, key);
    let node = guard.node();

    node
      .unwrap_leaf_node_ref("find_leaf_for_key must return leaf node")
      .contains_key(key)
  }

  pub fn find_leaf_for_key(
    lock_set: &mut LockSet,
    key: &str,
  ) -> LockSetNodeReadGuard {
    let (mut _parent_guard, mut current_node_guard) = {
      let root_identifier_guard = lock_set.root_identifier_read_guard_for_temp();
      let mut current_node_guard = lock_set.node_read_guard_for_temp(&root_identifier_guard.identifier());

      (root_identifier_guard.upcast(), current_node_guard)
    };

    loop {
      if current_node_guard.node().is_leaf_node() {
        break;
      }

      // Notice how I do the hand-over-hand locking here. This happens
      // because of reassignment to current_node_guard
      let child_guard = {
        let current_node = current_node_guard.node();
        let child_identifier = current_node
          .unwrap_interior_node_ref(
            "must not try to descend through leaf node",
          )
          .child_identifier_by_key(key);

        lock_set.node_read_guard_for_temp(child_identifier)
      };

      _parent_guard = current_node_guard.upcast();
      current_node_guard = child_guard;
    }

    let target_identifier = String::from(current_node_guard.node().identifier());
    current_node_guard.release();

    lock_set.node_read_guard_for_hold(&target_identifier)
  }
}
