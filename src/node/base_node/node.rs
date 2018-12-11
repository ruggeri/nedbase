use node::{InteriorNode, LeafNode};

pub enum Node {
  LeafNode(LeafNode),
  InteriorNode(InteriorNode),
}

impl Node {
  pub fn identifier(&self) -> &str {
    match self {
      Node::LeafNode(leaf_node) => &leaf_node.identifier(),
      Node::InteriorNode(interior_node) => &interior_node.identifier(),
    }
  }

  pub fn is_interior_node(&self) -> bool {
    match self {
      Node::InteriorNode(..) => true,
      _ => false,
    }
  }

  pub fn is_leaf_node(&self) -> bool {
    match self {
      Node::LeafNode(..) => true,
      _ => false,
    }
  }
}
