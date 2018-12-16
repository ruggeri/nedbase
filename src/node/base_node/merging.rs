use super::Node;
use btree::BTree;
use node::{InteriorNode, LeafNode, MergeOrRotateResult};

// These are methods common to InteriorNode and LeafNode for merging and
// rotation.
impl Node {
  pub fn merge_or_rotate_sibblings(
    btree: &BTree,
    parent_node: &mut InteriorNode,
    left_node: &mut Node,
    right_node: &mut Node,
    left_idx: usize,
  ) -> MergeOrRotateResult {
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
  ) -> MergeOrRotateResult {
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
  ) -> MergeOrRotateResult {
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
  ) -> MergeOrRotateResult {
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
}
