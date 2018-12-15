use super::InteriorNode;
use btree::BTree;
use node::{InsertionResult, SplitInfo};

impl InteriorNode {
  pub fn handle_split(
    &mut self,
    btree: &BTree,
    child_split_info: SplitInfo,
  ) -> InsertionResult {
    let old_child_idx = self
      .child_identifiers
      .iter()
      .position(|id| *id == child_split_info.old_identifier)
      .expect("where did the old child's identifier go?");

    if self.can_grow_without_split() {
      self
        .splits
        .insert(old_child_idx, child_split_info.new_median);
      self.child_identifiers[old_child_idx] =
        child_split_info.new_left_identifier;
      self.child_identifiers.insert(
        old_child_idx + 1,
        child_split_info.new_right_identifier,
      );

      return InsertionResult::DidInsert;
    }

    // Welp. We have to recursively keep splitting.
    self.insert_and_split(btree, child_split_info, old_child_idx)
  }

  fn insert_and_split(
    &mut self,
    btree: &BTree,
    child_split_info: SplitInfo,
    old_child_idx: usize,
  ) -> InsertionResult {
    let new_median_idx = self.max_key_capacity / 2;
    let new_median = self.splits[new_median_idx].clone();

    // When taking lef_child_identifiers, remember that we need one more
    // child_identifier than split key (thus `=new_median_idx`).
    let mut left_splits = self.splits[0..new_median_idx].to_vec();
    let mut left_child_identifiers =
      self.child_identifiers[0..=new_median_idx].to_vec();

    let mut right_splits = self.splits[(new_median_idx + 1)..].to_vec();
    let mut right_child_identifiers =
      self.child_identifiers[(new_median_idx + 1)..].to_vec();

    // Insert the split nodes of the child into one of the split nodes
    // of the parent.
    {
      // Which side should the child's split nodes belong on?
      let (splits, identifiers, old_child_idx) =
        if old_child_idx < left_child_identifiers.len() {
          (&mut left_splits, &mut left_child_identifiers, old_child_idx)
        } else {
          (
            &mut right_splits,
            &mut right_child_identifiers,
            old_child_idx - left_child_identifiers.len(),
          )
        };

      // Do the insertion.
      splits.insert(old_child_idx, child_split_info.new_median);
      identifiers[old_child_idx] = child_split_info.new_left_identifier;
      identifiers.insert(
        old_child_idx + 1,
        child_split_info.new_right_identifier,
      );
    }

    // Create and store new interior nodes.
    let new_left_identifier =
      InteriorNode::store(btree, left_splits, left_child_identifiers);
    let new_right_identifier =
      InteriorNode::store(btree, right_splits, right_child_identifiers);

    // Return opaque type to user so they can propagate split upward.
    InsertionResult::DidInsertWithSplit(SplitInfo {
      old_identifier: self.identifier.clone(),
      new_left_identifier,
      new_right_identifier,
      new_median,
    })
  }
}
