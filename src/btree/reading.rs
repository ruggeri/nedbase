use btree::BTree;
use locking::{NodeReadGuard, RootIdentifierReadGuard};
use std::sync::Arc;

impl BTree {
  pub fn find_leaf_for_key(
    btree: &Arc<BTree>,
    key: &str,
  ) -> NodeReadGuard {
    let mut current_node_guard = {
      let identifier_guard = RootIdentifierReadGuard::acquire(btree);
      NodeReadGuard::acquire(btree, &(*identifier_guard))
    };

    loop {
      if current_node_guard.is_leaf_node() {
        break
      }

      current_node_guard = {
        let child_identifier = current_node_guard
          .unwrap_interior_node_ref("must not try to descend through leaf node")
          .child_identifier_by_key(key);
        NodeReadGuard::acquire(btree, child_identifier)
      };
    }

    current_node_guard
  }

  pub fn contains_key(btree: &Arc<BTree>, key: &str) -> bool {
    BTree::find_leaf_for_key(btree, key)
      .unwrap_leaf_node_ref("find_leaf_for_key must return leaf node")
      .contains_key(key)
  }
}
