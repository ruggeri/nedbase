use super::LeafNode;
use btree::BTree;
use node::DeletionResult;

impl LeafNode {
  pub fn deletion(
    &mut self,
    _btree: &BTree,
    _key_to_delete: String,
  ) -> DeletionResult {
    // TODO: Implement me!
    unimplemented!()
  }
}
