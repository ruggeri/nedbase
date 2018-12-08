use btree::BTree;
use node::DeletionResult;
use super::LeafNode;

impl LeafNode {
  pub fn deletion(&mut self, _btree: &BTree, _key_to_delete: String) -> DeletionResult {
    // TODO: Implement me!
    unimplemented!()
  }
}
