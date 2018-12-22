use super::LeafNode;
use btree::BTree;
use node::StringComparisonValue;

// These methods all pertain to storing an InteriorNode.
impl LeafNode {
  // This is for public use. It's intended to be used to create an empty
  // starting root node.
  pub fn empty(btree: &BTree) -> String {
    LeafNode::store(
      btree,
      vec![],
      StringComparisonValue::Infinity,
      None,
    )
  }

  // This is used internally when splitting.
  pub(super) fn store(
    btree: &BTree,
    keys: Vec<String>,
    max_value: StringComparisonValue<String>,
    next_node_identifier: Option<String>,
  ) -> String {
    let identifier = btree.get_new_identifier();

    let node = LeafNode {
      identifier: identifier.clone(),
      keys,
      max_value,
      next_node_identifier,
      max_key_capacity: btree.max_key_capacity(),
    };

    btree.store_node(node.upcast());

    identifier
  }
}
