use node::{InteriorNode, LeafNode, TraversalDirection};

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

  pub fn next_node_identifier(&self) -> Option<&String> {
    match self {
      Node::LeafNode(leaf_node) => leaf_node.next_node_identifier(),
      Node::InteriorNode(interior_node) => {
        interior_node.next_node_identifier()
      }
    }
  }

  pub fn traverse_toward(&self, key: &str) -> TraversalDirection<&str> {
    match self {
      Node::LeafNode(leaf_node) => leaf_node.traverse_toward(key),
      Node::InteriorNode(interior_node) => {
        interior_node.traverse_toward(key)
      }
    }
  }
}
