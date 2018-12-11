use super::LeafNode;
use btree::BTree;
use node::util::search_sorted_strings_for_str;
use node::DeletionResult;

impl LeafNode {
  // Deletion will not perform any rebalancing; that function must be
  // handled by the caller.
  //
  // TODO: This feels weird because of the asymetry with
  // insertion/splitting. Even then, the caller had to handle a split,
  // though. The difference is that insertion gave an opaque type...
  pub fn delete(&mut self, key_to_delete: &str) -> DeletionResult {
    match search_sorted_strings_for_str(&self.keys, key_to_delete) {
      Err(_) => DeletionResult::KeyWasNotPresent,
      Ok(idx) => {
        self.keys.remove(idx);
        DeletionResult::DidDelete
      }
    }
  }

  pub fn merge_or_rotate_sibblings(
    btree: &BTree,
    left_node: &mut LeafNode,
    right_node: &mut LeafNode,
  ) -> String {
    // TODO: Must write logic for rotation!!

    let mut keys = left_node.keys.clone();
    keys.extend(right_node.keys.iter().cloned());

    LeafNode::store(btree, keys)
  }
}
