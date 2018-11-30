use super::{InteriorNode, LeafNode};

pub enum Node {
  LeafNode(LeafNode),
  InteriorNode(InteriorNode),
}

impl Node {
  pub fn can_grow_without_split(&self) -> bool {
    match self {
      Node::LeafNode(leaf_node) => leaf_node.can_grow_without_split(),
      Node::InteriorNode(interior_node) => interior_node.can_grow_without_split(),
    }
  }

  pub fn identifier(&self) -> &str {
    match self {
      Node::LeafNode(leaf_node) => &leaf_node.identifier(),
      Node::InteriorNode(interior_node) => &interior_node.identifier(),
    }
  }

  pub fn unwrap_interior_node_ref(&self, message: &'static str) -> &InteriorNode {
    match self {
      Node::InteriorNode(interior_node) => interior_node,
      Node::LeafNode(..) => panic!(message),
    }
  }

  pub fn unwrap_interior_node_mut_ref(&mut self, message: &'static str) -> &mut InteriorNode {
    match self {
      Node::InteriorNode(interior_node) => interior_node,
      Node::LeafNode(..) => panic!(message),
    }
  }

  pub fn unwrap_leaf_node_ref(&self, message: &'static str) -> &LeafNode {
    match self {
      Node::InteriorNode(..) => panic!(message),
      Node::LeafNode(leaf_node) => leaf_node,
    }
  }

  pub fn unwrap_leaf_node_mut_ref(&mut self, message: &'static str) -> &mut LeafNode {
    match self {
      Node::InteriorNode(..) => panic!(message),
      Node::LeafNode(leaf_node) => leaf_node,
    }
  }
}
