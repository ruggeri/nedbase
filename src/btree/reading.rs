use locking::{
  NodeReadGuard,
  RootIdentifierReadGuard,
};
use node::Node;
use std::sync::Arc;
use super::BTree;

impl BTree {
  pub fn find_leaf_for_key(btree: &Arc<BTree>, key: &str) -> NodeReadGuard {
    ::util::log_method_entry("find_leaf_for_key starting");
    let mut current_node_guard = {
      let identifier_guard = RootIdentifierReadGuard::acquire(btree);
      NodeReadGuard::acquire(btree, &(*identifier_guard))
    };

    loop {
      current_node_guard = match &(*current_node_guard) {
        Node::LeafNode(_) => break,
        Node::InteriorNode(interior_node) => {
          let child_identifier = interior_node.child_identifier_by_key(key);
          NodeReadGuard::acquire(btree, child_identifier)
        }
      }
    };

    ::util::log_method_entry("find_leaf_for_key completed");
    current_node_guard
  }

  pub fn contains_key(btree: &Arc<BTree>, key: &str) -> bool {
    match &(*(BTree::find_leaf_for_key(btree, key))) {
      Node::LeafNode(ln) => ln.contains_key(key),
      Node::InteriorNode(..) => panic!("Unexpected interior node!"),
    }
  }
}

