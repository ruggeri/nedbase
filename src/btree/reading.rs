use locking::{
  NodeReadGuard,
  RootIdentifierReadGuard,
};
use node::Node;
use super::BTree;

impl BTree {
  pub fn find_leaf_for_key(&self, key: &str) -> NodeReadGuard {
    let mut current_node_guard = {
      let identifier_guard = RootIdentifierReadGuard::acquire(self);
      NodeReadGuard::acquire(self, &(*identifier_guard.identifier))
    };

    loop {
      current_node_guard = match &(*current_node_guard.node) {
        Node::LeafNode(_) => break,
        Node::InteriorNode(interior_node) => {
          let child_identifier = interior_node.child_identifier_by_key(key);
          NodeReadGuard::acquire(self, child_identifier)
        }
      }
    };

    current_node_guard
  }

  pub fn contains_key(&self, key: &str) -> bool {
    match &(*(self.find_leaf_for_key(key).node)) {
      Node::LeafNode(ln) => ln.contains_key(key),
      Node::InteriorNode(..) => panic!("Unexpected interior node!"),
    }
  }
}

