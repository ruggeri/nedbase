use super::LeafNode;
use node::util::search_sorted_strings_for_str;
use node::DeletionResult;

impl LeafNode {
  // Deletion will not perform any rebalancing; that function must be
  // handled by the caller.
  pub fn delete(&mut self, key_to_delete: &str) -> DeletionResult {
    match search_sorted_strings_for_str(&self.keys, key_to_delete) {
      Err(_) => DeletionResult::KeyWasNotPresent,
      Ok(idx) => {
        self.keys.remove(idx);
        DeletionResult::DidDelete
      }
    }
  }
}
