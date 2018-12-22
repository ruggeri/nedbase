use super::LeafNode;
use btree::BTree;
use node::{
  util::search_sorted_strings_for_str, InsertionResult, SplitInfo,
  StringComparisonValue,
};

// These are methods for inserting a value into the LeafNode, and for
// splitting a LeafNode when it becomes full.
impl LeafNode {
  pub fn insert_key(
    &mut self,
    btree: &BTree,
    key_to_insert: String,
  ) -> InsertionResult {
    // Is this a no-op? Key already inserted?
    let insertion_idx =
      match search_sorted_strings_for_str(&self.keys, &key_to_insert) {
        Ok(_) => return InsertionResult::KeyWasAlreadyInserted,
        Err(idx) => idx,
      };

    self.keys.insert(insertion_idx, key_to_insert);

    if !self.is_overfull() {
      InsertionResult::DidInsert
    } else {
      // Welp, we have to split after all.
      self.split(btree)
    }
  }

  fn split(&mut self, btree: &BTree) -> InsertionResult {
    // We divide the keys into left/right portions.
    let left_keys = self.keys[0..(self.max_key_capacity / 2)].to_vec();
    let right_keys = self.keys[(self.max_key_capacity / 2)..].to_vec();

    // We choose a new median.
    let new_median = left_keys
      .last()
      .expect("Just split node must have keys")
      .clone();

    // Create and store new right sibbilng leaf node.
    let new_right_identifier = LeafNode::store(
      btree,
      right_keys,
      self.max_value.clone(),
      self.next_node_identifier.clone(),
    );
    self.keys = left_keys;
    self.max_value =
      StringComparisonValue::DefiniteValue(new_median.clone());
    self.next_node_identifier = Some(new_right_identifier.clone());

    // Let the caller know we split so that they can add the new
    // sibbling as a child of the previous level.
    InsertionResult::DidInsertWithSplit(SplitInfo {
      new_right_identifier,
      new_median,
    })
  }
}
