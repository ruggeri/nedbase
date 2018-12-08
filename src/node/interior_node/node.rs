use node::{util::search_sorted_strings_for_str, SplitInfo};

pub struct InteriorNode {
  pub(super) identifier: String,
  // The rule is that all keys such that `target_key <= keys[idx]` live
  // in child `idx`.
  //
  // Another rule is that for interior nodes the number of child
  // identifiers is always one more than the number of keys.
  pub(super) splits: Vec<String>,
  pub(super) child_identifiers: Vec<String>,
  pub(super) max_key_capacity: usize,
}

impl InteriorNode {
  pub fn new(
    identifier: String,
    splits: Vec<String>,
    child_identifiers: Vec<String>,
    max_key_capacity: usize,
  ) -> InteriorNode {
    InteriorNode {
      identifier,
      splits,
      child_identifiers,
      max_key_capacity,
    }
  }

  pub fn new_root(
    identifier: String,
    split_info: SplitInfo,
    max_key_capacity: usize,
  ) -> InteriorNode {
    InteriorNode {
      identifier,
      splits: vec![split_info.new_median],
      child_identifiers: vec![
        split_info.new_left_identifier,
        split_info.new_right_identifier,
      ],
      max_key_capacity,
    }
  }

  pub fn child_identifier_by_key(&self, key: &str) -> &str {
    let idx = match search_sorted_strings_for_str(&self.splits, key) {
      Ok(idx) => idx,
      Err(idx) => idx,
    };

    &self.child_identifiers[idx]
  }

  pub fn can_grow_without_split(&self) -> bool {
    self.splits.len() < self.max_key_capacity
  }

  pub fn identifier(&self) -> &str {
    &self.identifier
  }
}
