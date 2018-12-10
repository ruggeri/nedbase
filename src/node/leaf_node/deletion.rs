use super::LeafNode;
use btree::BTree;
use node::util::search_sorted_strings_for_str;
use node::DeletionResult;

impl LeafNode {
  // Deletion will not perform any rebalancing; that function must be
  // handled by the caller. This feels weird because of the asymetry
  // with insertion/splitting. Even then, the caller had to handle a
  // split, though...
  pub fn delete(&mut self, key_to_delete: &str) -> DeletionResult {
    match search_sorted_strings_for_str(&self.keys, key_to_delete) {
      Err(_) => DeletionResult::KeyWasNotPresent,
      Ok(idx) => {
        self.keys.remove(idx);
        DeletionResult::DidDelete
      }
    }
  }

  pub fn merge_sibblings(
    btree: &BTree,
    left_node: &LeafNode,
    right_node: &LeafNode,
  ) -> LeafNode {
    // TODO: Write me!
    unimplemented!()
  }
}
