use btree::BTree;
use node::{
  InsertionResult,
  SplitInfo,
  util::search_sorted_strings_for_str
};
use super::LeafNode;

impl LeafNode {
  pub fn insert(&mut self, btree: &BTree, key_to_insert: String) -> InsertionResult {
    // Is this a no-op? Key already inserted?
    let insertion_idx = match search_sorted_strings_for_str(&self.keys, &key_to_insert) {
      Ok(_) => return InsertionResult::KeyWasAlreadyInserted,
      Err(idx) => idx,
    };

    // It's easy to insert if we can grow.
    if self.can_grow_without_split() {
      self.keys.insert(insertion_idx, key_to_insert);
      return InsertionResult::DidInsert
    }

    // Welp, we have to split after all.
    self.insert_and_split(btree, key_to_insert)
  }

  fn insert_and_split(&self, btree: &BTree, key_to_insert: String) -> InsertionResult {
    // We divide the keys into left/right portions.
    let mut left_keys = self.keys[0..(self.max_key_capacity/2)].to_vec();
    let mut right_keys = self.keys[(self.max_key_capacity/2)..].to_vec();

    // We choose a new median. TODO: should this be chosen randomly?
    let new_median = left_keys.last().expect("Just split node must have keys").clone();

    // We must insert the new key into one of the halves.
    {
      let keys = if key_to_insert <= new_median {
        &mut left_keys
      } else {
        &mut right_keys
      };

      let insertion_idx = match search_sorted_strings_for_str(keys, &key_to_insert) {
        Ok(_) => panic!("key_to_insert wasn't supposed to have been inserted..."),
        Err(idx) => idx,
      };

      keys.insert(insertion_idx, key_to_insert);
    }

    // Use the BTree class to create new leaf nodes for us. TODO: This
    // will someday be the responsibility of some kind of
    // storage-engine.
    let new_left_identifier = btree.store_new_leaf_node(left_keys);
    let new_right_identifier = btree.store_new_leaf_node(right_keys);

    InsertionResult::DidInsertWithSplit(SplitInfo {
      old_identifier: self.identifier.clone(),
      new_left_identifier,
      new_right_identifier,
      new_median,
    })
  }
}
