use btree::BTree;
use node::{
  InsertionResult,
  SplitInfo,
};
use super::InteriorNode;

impl InteriorNode {
  pub fn handle_split(&mut self, btree: &BTree, child_split_info: SplitInfo) -> InsertionResult {
    // TODO: This is slowish. But I'm not sure how to make faster.
    let old_child_idx = self.child_identifiers.iter().position(|id| {
      *id == child_split_info.old_identifier
    }).expect("where did the old child's identifier go?");

    if self.can_grow_without_split() {
      self.splits.insert(old_child_idx, child_split_info.new_median);
      self.child_identifiers[old_child_idx] = child_split_info.new_left_identifier;
      self.child_identifiers.insert(
        old_child_idx + 1,
        child_split_info.new_right_identifier,
      );

      return InsertionResult::DidInsert;
    }

    // Welp. We have to recursively keep splitting.
    self.insert_and_split(btree, child_split_info, old_child_idx)
  }

  fn insert_and_split(&mut self, btree: &BTree, child_split_info: SplitInfo, old_child_idx: usize) -> InsertionResult {
    let new_median_idx = self.max_key_capacity/2;
    let new_median = self.splits[new_median_idx].clone();

    let mut left_splits = self.splits[0..new_median_idx].to_vec();
    let mut left_child_identifiers = self.child_identifiers[0..(new_median_idx + 1)].to_vec();

    let mut right_splits = self.splits[(new_median_idx + 1)..].to_vec();
    let mut right_child_identifiers = self.child_identifiers[(new_median_idx + 1)..].to_vec();

    // Insert the newly split children.
    {
      // Which side should they belong on?
      let (splits, identifiers, old_child_idx) = if old_child_idx < left_child_identifiers.len() {
        (&mut left_splits, &mut left_child_identifiers, old_child_idx)
      } else {
        (&mut right_splits, &mut right_child_identifiers, old_child_idx - left_child_identifiers.len())
      };

      // Do the insertion.
      splits.insert(old_child_idx, child_split_info.new_median);
      identifiers[old_child_idx] = child_split_info.new_left_identifier;
      identifiers.insert(old_child_idx + 1, child_split_info.new_right_identifier);
    }

    // Use the BTree class to create new interior nodes for us. TODO:
    // This will someday be the responsibility of some kind of
    // storage-engine.
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
