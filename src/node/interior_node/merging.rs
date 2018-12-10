use super::InteriorNode;
use btree::BTree;
use node::{LeafNode, Node};

impl InteriorNode {
  pub fn merge_children(
    &mut self,
    btree: &BTree,
    node1: &Node,
    node2: &Node,
  ) -> Node {
    let (mut idx1, mut idx2) = (0, 0);

    let mut idx1 = self
      .child_identifiers
      .iter()
      .position(|identifier| identifier == node1.identifier())
      .expect("node1 is supposed to be a child...");

    let mut idx2 = self
      .child_identifiers
      .iter()
      .position(|identifier| identifier == node2.identifier())
      .expect("node2 is supposed to be a child...");

    let (left_node, right_node) = if idx1 < idx2 {
      (node1, node2)
    } else {
      (node2, node1)
    };

    match (left_node, right_node) {
      (Node::LeafNode(left_node), Node::LeafNode(right_node)) => {
        LeafNode::merge_sibblings(btree, left_node, right_node).upcast()
      }

      (
        Node::InteriorNode(left_node),
        Node::InteriorNode(right_node),
      ) => InteriorNode::merge_sibblings(btree, left_node, right_node)
        .upcast(),

      _ => panic!("sibblings can't ever be different node types..."),
    }
  }

  pub fn merge_sibblings(
    btree: &BTree,
    left_node: &InteriorNode,
    right_node: &InteriorNode,
  ) -> InteriorNode {
    // TODO: Write me!
    unimplemented!()
  }
}
