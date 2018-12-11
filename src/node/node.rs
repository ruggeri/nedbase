use super::{InteriorNode, LeafNode};
use btree::BTree;

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

  pub fn merge_or_rotate_sibblings(
    btree: &BTree,
    parent_node: &mut InteriorNode,
    left_node: &mut Node,
    right_node: &mut Node,
    left_idx: usize,
  ) {
    match (
      left_node.can_delete_without_becoming_deficient(),
      right_node.can_delete_without_becoming_deficient(),
    ) {
      (true, true) => panic!("wait, merge is not needed??"),

      (false, true) => Node::rotate_left_from_sibbling(
        parent_node,
        left_node,
        right_node,
        left_idx,
      ),

      (true, false) => Node::rotate_right_from_sibbling(
        parent_node,
        left_node,
        right_node,
        left_idx,
      ),

      (false, false) => Node::merge_sibblings(
        btree,
        parent_node,
        left_node,
        right_node,
        left_idx,
      ),
    }
  }

  pub fn merge_sibblings(
    btree: &BTree,
    parent_node: &mut InteriorNode,
    left_node: &mut Node,
    right_node: &mut Node,
    left_idx: usize,
  ) {
    match (left_node, right_node) {
      (Node::LeafNode(left_node), Node::LeafNode(right_node)) => {
        LeafNode::merge_sibblings(
          btree,
          parent_node,
          left_node,
          right_node,
          left_idx,
        )
      }

      (
        Node::InteriorNode(left_node),
        Node::InteriorNode(right_node),
      ) => InteriorNode::merge_sibblings(
        btree,
        parent_node,
        left_node,
        right_node,
        left_idx,
      ),

      _ => panic!("sibblings can't ever be different node types..."),
    }
  }

  pub fn rotate_right_from_sibbling(
    parent_node: &mut InteriorNode,
    left_node: &mut Node,
    right_node: &mut Node,
    left_idx: usize,
  ) {
    match (left_node, right_node) {
      (Node::LeafNode(left_node), Node::LeafNode(right_node)) => {
        LeafNode::rotate_right_from_sibbling(
          parent_node,
          left_node,
          right_node,
          left_idx,
        )
      }

      (
        Node::InteriorNode(left_node),
        Node::InteriorNode(right_node),
      ) => InteriorNode::rotate_right_from_sibbling(
        parent_node,
        left_node,
        right_node,
        left_idx,
      ),

      _ => panic!("sibblings can't ever be different node types..."),
    }
  }

  pub fn rotate_left_from_sibbling(
    parent_node: &mut InteriorNode,
    left_node: &mut Node,
    right_node: &mut Node,
    left_idx: usize,
  ) {
    match (left_node, right_node) {
      (Node::LeafNode(left_node), Node::LeafNode(right_node)) => {
        LeafNode::rotate_left_from_sibbling(
          parent_node,
          left_node,
          right_node,
          left_idx,
        )
      }

      (
        Node::InteriorNode(left_node),
        Node::InteriorNode(right_node),
      ) => InteriorNode::rotate_left_from_sibbling(
        parent_node,
        left_node,
        right_node,
        left_idx,
      ),

      _ => panic!("sibblings can't ever be different node types..."),
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
