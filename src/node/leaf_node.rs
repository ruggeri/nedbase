use btree::BTree;
use super::common::{
  search_strings_for_str,
  InsertionResult,
  SplitInfo,
};

pub struct LeafNode {
  pub identifier: String,
  pub keys: Vec<String>,
  pub max_key_capacity: usize,
}

impl LeafNode {
  pub fn contains_key(&self, key: &str) -> bool {
    search_strings_for_str(&self.keys, key).is_ok()
  }

  pub fn can_grow_without_split(&self) -> bool {
    self.keys.len() < self.max_key_capacity
  }

  pub fn insert(&mut self, btree: &BTree, key_to_insert: String) -> InsertionResult {
    let insertion_idx = match search_strings_for_str(&self.keys, &key_to_insert) {
      Ok(_) => return InsertionResult::KeyWasAlreadyInserted,
      Err(idx) => idx,
    };

    if self.can_grow_without_split() {
      self.keys.insert(insertion_idx, key_to_insert);
      return InsertionResult::DidInsert
    }

    let mut left_keys = self.keys[0..(self.max_key_capacity/2)].to_vec();
    let mut right_keys = self.keys[(self.max_key_capacity/2)..].to_vec();

    let new_median = left_keys.last().expect("Just split node must have keys").clone();
    {
      let keys = if key_to_insert <= new_median {
        &mut left_keys
      } else {
        &mut right_keys
      };

      let insertion_idx = match search_strings_for_str(keys, &key_to_insert) {
        Ok(_) => panic!("When was insertion key inserted?"),
        Err(idx) => idx,
      };

      keys.insert(insertion_idx, key_to_insert);
    }

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
