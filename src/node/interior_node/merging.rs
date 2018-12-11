use super::InteriorNode;
use btree::BTree;
use node::{LeafNode, Node};

impl InteriorNode {
  pub fn merge_or_rotate_children(
    &mut self,
    btree: &BTree,
    node1: &mut Node,
    node2: &mut Node,
  ) {
    let idx1 = self
      .child_identifiers
      .iter()
      .position(|identifier| identifier == node1.identifier())
      .expect("node1 is supposed to be a child...");

    let idx2 = self
      .child_identifiers
      .iter()
      .position(|identifier| identifier == node2.identifier())
      .expect("node2 is supposed to be a child...");

    let (left_idx, left_node, right_idx, right_node) = if idx1 < idx2 {
      (idx1, node1, idx2, node2)
    } else {
      (idx2, node2, idx1, node1)
    };

    // Merges two nodes immutably, creating a new node.
    let new_node_identifier = match (left_node, right_node) {
      (Node::LeafNode(left_node), Node::LeafNode(right_node)) => {
        LeafNode::merge_or_rotate_sibblings(
          btree, left_node, right_node,
        )
      }

      (
        Node::InteriorNode(left_node),
        Node::InteriorNode(right_node),
      ) => InteriorNode::merge_or_rotate_sibblings(
        btree, left_node, right_node,
      ),

      _ => panic!("sibblings can't ever be different node types..."),
    };

    // left_idx is what split the merged nodes from each other.
    self.splits.remove(left_idx);
    // Think of "merging in" from right to left. We remove the
    // right identifier.
    self.child_identifiers.remove(right_idx);
    // And update the left one.
    self.child_identifiers[left_idx] = new_node_identifier;
  }

  pub fn merge_or_rotate_sibblings(
    btree: &BTree,
    left_node: &InteriorNode,
    right_node: &InteriorNode,
  ) -> String {
    // TODO: Must write logic for rotation!!

    // Merge splits
    let mut splits = left_node.splits.clone();
    splits.extend(right_node.splits.iter().cloned());

    // Merge child_identifiers
    let mut child_identifiers = left_node.child_identifiers.clone();
    child_identifiers
      .extend(right_node.child_identifiers.iter().cloned());

    // Create the new node.
    InteriorNode::store(btree, splits, child_identifiers)
  }
}
