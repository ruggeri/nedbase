use super::InteriorNode;
use btree::BTree;
use node::{
  util::search_sorted_strings_for_str, InsertionResult, MaxValue,
  SplitInfo,
};

impl InteriorNode {
  pub fn handle_split(
    &mut self,
    btree: &BTree,
    child_split_info: SplitInfo,
  ) -> InsertionResult {
    if !self.max_value.is_ge_to(&child_split_info.new_median) {
      println!("max_value: {:?}", self.max_value);
      println!("new_median: {:?}", child_split_info.new_median);
      panic!("Ancestor must have split in the meantime...")
    }

    let split_idx = match search_sorted_strings_for_str(
      &self.splits,
      &child_split_info.new_median,
    ) {
      Ok(_) => panic!("median should never be re-inserted"),
      Err(split_idx) => split_idx,
    };

    // Note that the left child is already in the list of
    // child_identifiers.
    self.splits.insert(split_idx, child_split_info.new_median);
    self
      .child_identifiers
      .insert(split_idx + 1, child_split_info.new_right_identifier);

    if !self.is_overfull() {
      InsertionResult::DidInsert
    } else {
      // Welp. We have to recursively keep splitting.
      self.split(btree)
    }
  }

  fn split(&mut self, btree: &BTree) -> InsertionResult {
    let new_median_idx = self.max_key_capacity / 2;
    let new_median = self.splits[new_median_idx].clone();

    // When taking lef_child_identifiers, remember that we need one more
    // child_identifier than split key (thus `=new_median_idx`).
    let left_splits = self.splits[0..new_median_idx].to_vec();
    let left_child_identifiers =
      self.child_identifiers[0..=new_median_idx].to_vec();

    let right_splits = self.splits[(new_median_idx + 1)..].to_vec();
    let right_child_identifiers =
      self.child_identifiers[(new_median_idx + 1)..].to_vec();

    // Create and store new interior nodes.
    let new_right_identifier = InteriorNode::store(
      btree,
      String::from(self.level_identifier()),
      right_splits,
      right_child_identifiers,
      self.max_value.clone(),
      self.next_node_identifier.clone(),
    );

    self.splits = left_splits;
    self.child_identifiers = left_child_identifiers;
    self.max_value = MaxValue::DefiniteValue(new_median.clone());
    self.next_node_identifier = Some(new_right_identifier.clone());

    // Return opaque type to user so they can propagate split upward.
    InsertionResult::DidInsertWithSplit(SplitInfo {
      new_right_identifier,
      new_median,
    })
  }
}
