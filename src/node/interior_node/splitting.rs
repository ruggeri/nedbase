use super::InteriorNode;
use btree::BTree;
use node::{
  util::search_sorted_strings_for_str, SplitInfo, StringComparisonValue,
};

impl InteriorNode {
  // This method is used to "handle" the split of a child. Normally that
  // means simply adding a new split key and child to this node. But
  // sometimes we must recursively split the parent node!
  pub fn handle_split(
    &mut self,
    btree: &BTree,
    child_split_info: SplitInfo,
  ) -> Option<SplitInfo> {
    if !self.max_value.is_ge_to(&child_split_info.new_median) {
      // This can happen if we split a child, move back to the parent,
      // but the parent has itself split, and the new child should be
      // placed in a node to the right of the parent.
      //
      // The caller should be careful of that situation.
      panic!("Parent must have split in the meantime...")
    }

    let split_idx = match search_sorted_strings_for_str(
      &self.splits,
      &child_split_info.new_median,
    ) {
      Ok(_) => panic!("median should never be re-inserted"),
      Err(split_idx) => split_idx,
    };

    // Note that the left child is already attached to a node at this
    // level. Only need to connect up the right child.
    self.splits.insert(split_idx, child_split_info.new_median);
    self
      .child_identifiers
      .insert(split_idx + 1, child_split_info.new_right_identifier);

    if !self.is_overfull() {
      // No further split occurred.
      None
    } else {
      // Welp. We have to recursively keep splitting.
      Some(self.split(btree))
    }
  }

  fn split(&mut self, btree: &BTree) -> SplitInfo {
    let new_median_idx = self.max_key_capacity / 2;

    // Split the split values into left and right. The last of the left
    // splits is in truth going to be the new median.
    let right_splits = self.splits.split_off(new_median_idx + 1);
    let new_median = self.splits.pop().unwrap();
    // When taking lef_child_identifiers, remember that we need one more
    // child_identifier than split key.
    let right_child_identifiers =
      self.child_identifiers.split_off(new_median_idx + 1);

    // Extract values needed to move to right sibbling.
    let right_max_value = std::mem::replace(
      &mut self.max_value,
      StringComparisonValue::DefiniteValue(new_median.clone()),
    );
    // Note that None is temporary here.
    let right_next_node_identifier =
      std::mem::replace(&mut self.next_node_identifier, None);

    // Create the right sibbling.
    let new_right_identifier = InteriorNode::store(
      btree,
      right_splits,
      right_child_identifiers,
      right_max_value,
      right_next_node_identifier,
    );

    // Update ourself, connecting us to the newly budded sibbling.
    self.next_node_identifier = Some(new_right_identifier.clone());

    // Return opaque type to user so they can propagate split upward.
    SplitInfo {
      new_median,
      new_right_identifier,
    }
  }
}
