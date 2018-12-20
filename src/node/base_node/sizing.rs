use super::Node;

// These are methods common to InteriorNode and LeafNode about sizing.
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

  pub fn is_deficient(&self) -> bool {
    match self {
      Node::LeafNode(leaf_node) => leaf_node.is_deficient(),
      Node::InteriorNode(interior_node) => interior_node.is_deficient(),
    }
  }

  // Helper to determine whether a node is deficient in size.
  pub(in node) fn _is_deficient(
    num_keys: usize,
    max_key_capacity: usize,
  ) -> bool {
    // The largest deficient node should be able to merge with the
    // smallest sufficient node and still obey max_key_capacity.
    num_keys + (num_keys + 1) <= max_key_capacity
  }
}
