use super::InteriorNode;
use btree::BTree;
use node::SplitInfo;

// These methods all pertain to storing an InteriorNode.
impl InteriorNode {
  // This method is used internally when splitting an InteriorNode.
  pub(super) fn store(
    btree: &BTree,
    splits: Vec<String>,
    child_identifiers: Vec<String>,
  ) -> String {
    let identifier = btree.get_new_identifier();

    let node = InteriorNode {
      identifier: identifier.clone(),
      splits,
      child_identifiers,
      max_key_capacity: btree.max_key_capacity(),
    };

    btree.store_node(node.upcast());

    identifier
  }

  // This method is used *externally* when the root node is split.
  pub fn store_new_root(
    btree: &BTree,
    split_info: SplitInfo,
  ) -> String {
    InteriorNode::store(
      btree,
      vec![split_info.new_median],
      vec![
        split_info.new_left_identifier,
        split_info.new_right_identifier,
      ],
    )
  }
}
