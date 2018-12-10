use super::{InteriorNode, LeafNode};

pub enum Node {
  LeafNode(LeafNode),
  InteriorNode(InteriorNode),
}

impl Node {
  pub fn can_delete_without_becoming_deficient(&self) -> bool {
    match self {
      Node::LeafNode(leaf_node) => {
        leaf_node.can_delete_without_becoming_deficient()
      }
      Node::InteriorNode(interior_node) => {
        interior_node.can_delete_without_becoming_deficient()
      }
    }
  }

  pub fn can_grow_without_split(&self) -> bool {
    match self {
      Node::LeafNode(leaf_node) => leaf_node.can_grow_without_split(),
      Node::InteriorNode(interior_node) => {
        interior_node.can_grow_without_split()
      }
    }
  }

  pub fn identifier(&self) -> &str {
    match self {
      Node::LeafNode(leaf_node) => &leaf_node.identifier(),
      Node::InteriorNode(interior_node) => &interior_node.identifier(),
    }
  }

  pub fn is_deficient(&self) -> bool {
    match self {
      Node::LeafNode(leaf_node) => leaf_node.is_deficient(),
      Node::InteriorNode(interior_node) => interior_node.is_deficient(),
    }
  }

  // Helper to determine whether a node is deficient in size.
  pub fn is_deficient_size(
    num_keys: usize,
    max_key_capacity: usize,
  ) -> bool {
    // The largest deficient node should be able to merge with the
    // smallest sufficient node and still obey max_key_capacity.
    num_keys + (num_keys + 1) <= max_key_capacity
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
