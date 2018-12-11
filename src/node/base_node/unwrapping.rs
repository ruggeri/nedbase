use super::Node;
use node::{InteriorNode, LeafNode};

impl Node {
  pub fn unwrap_interior_node_ref(
    &self,
    message: &'static str,
  ) -> &InteriorNode {
    match self {
      Node::InteriorNode(interior_node) => interior_node,
      Node::LeafNode(..) => panic!(message),
    }
  }

  pub fn unwrap_interior_node_mut_ref(
    &mut self,
    message: &'static str,
  ) -> &mut InteriorNode {
    match self {
      Node::InteriorNode(interior_node) => interior_node,
      Node::LeafNode(..) => panic!(message),
    }
  }

  pub fn unwrap_leaf_node_ref(
    &self,
    message: &'static str,
  ) -> &LeafNode {
    match self {
      Node::InteriorNode(..) => panic!(message),
      Node::LeafNode(leaf_node) => leaf_node,
    }
  }

  pub fn unwrap_leaf_node_mut_ref(
    &mut self,
    message: &'static str,
  ) -> &mut LeafNode {
    match self {
      Node::InteriorNode(..) => panic!(message),
      Node::LeafNode(leaf_node) => leaf_node,
    }
  }
}
