use btree::BTree;
use super::common::{
  search_strings_for_str,
  InsertionResult,
  SplitInfo,
};

pub struct InteriorNode {
  pub identifier: String,
  // The rule is that all keys such that `target_key <= keys[idx]` live
  // in child `idx`.
  //
  // Another rule is that for interior nodes the number of child
  // identifiers is always one more than the number of keys.
  pub splits: Vec<String>,
  pub child_identifiers: Vec<String>,
  pub max_key_capacity: usize,
}

impl InteriorNode {
  pub fn child_identifier_by_key(&self, key: &str) -> &str {
    let idx = match search_strings_for_str(&self.splits, key) {
      Ok(idx) => idx,
      Err(idx) => idx,
    };

    &self.child_identifiers[idx]
  }

  pub fn can_grow_without_split(&self) -> bool {
    self.splits.len() < self.max_key_capacity
  }

  pub fn handle_split(&mut self, btree: &BTree, child_result: InsertionResult) -> InsertionResult {
    let child_split_info = match child_result {
      InsertionResult::DidInsert => return InsertionResult::DidInsert,
      InsertionResult::KeyWasAlreadyInserted => return InsertionResult::KeyWasAlreadyInserted,
      InsertionResult::DidInsertWithSplit(split_info) => split_info,
    };

    if self.can_grow_without_split() {
      let old_child_idx = match search_strings_for_str(&self.child_identifiers, &child_split_info.old_identifier) {
        Ok(idx) => idx,
        Err(_) => panic!("Where did the child's identifier go?"),
      };

      self.splits.insert(old_child_idx, child_split_info.new_median);
      self.child_identifiers[old_child_idx] = child_split_info.new_left_identifier;
      self.child_identifiers.insert(
        old_child_idx + 1,
        child_split_info.new_right_identifier,
      );

      return InsertionResult::DidInsert;
    }

    // Else we must split again.
    let new_median_idx = self.max_key_capacity/2;
    let new_median = self.splits[new_median_idx].clone();

    let mut left_splits = self.splits[0..new_median_idx].to_vec();
    let mut left_child_identifiers = self.child_identifiers[0..(new_median_idx + 1)].to_vec();

    let mut right_splits = self.splits[(new_median_idx + 1)..].to_vec();
    let mut right_child_identifiers = self.child_identifiers[(new_median_idx + 1)..].to_vec();

    {
      let (splits, identifiers) = if child_split_info.old_identifier <= new_median {
        (&mut left_splits, &mut left_child_identifiers)
      } else {
        (&mut right_splits, &mut right_child_identifiers)
      };

      let insertion_idx = match search_strings_for_str(&splits, &child_split_info.old_identifier) {
        Ok(_) => panic!("When was insertion key inserted?"),
        Err(idx) => idx,
      };

      splits.insert(insertion_idx, child_split_info.new_median);
      identifiers[insertion_idx] = child_split_info.new_left_identifier;
      identifiers.insert(insertion_idx + 1, child_split_info.new_right_identifier);
    }

    let new_left_identifier = btree.store_new_interior_node(left_splits, left_child_identifiers);
    let new_right_identifier = btree.store_new_interior_node(right_splits, right_child_identifiers);

    InsertionResult::DidInsertWithSplit(SplitInfo {
      old_identifier: self.identifier.clone(),
      new_left_identifier,
      new_right_identifier,
      new_median,
    })
  }
}
