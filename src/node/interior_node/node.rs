use node::Node;
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

  // TODO: This method feels a little odd.
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

  pub fn can_delete_without_becoming_deficient(&self) -> bool {
    if self.splits.is_empty() {
      // Special case because else subtraction by one is dangerous!
      return false;
    }

    !Node::is_deficient_size(
      self.splits.len() - 1,
      self.max_key_capacity,
    )
  }

  pub fn can_grow_without_split(&self) -> bool {
    self.splits.len() < self.max_key_capacity
  }

  pub fn child_identifier_by_key(&self, key: &str) -> &str {
    let idx = match search_sorted_strings_for_str(&self.splits, key) {
      Ok(idx) => idx,
      Err(idx) => idx,
    };

    &self.child_identifiers[idx]
  }

  pub fn child_identifier_by_idx(&self, idx: usize) -> &str {
    &self.child_identifiers[idx]
  }

  pub fn child_idx_by_key(&self, key: &str) -> usize {
    match search_sorted_strings_for_str(&self.splits, key) {
      Ok(idx) => idx,
      Err(idx) => idx,
    }
  }

  pub fn identifier(&self) -> &str {
    &self.identifier
  }

  pub fn is_deficient(&self) -> bool {
    Node::is_deficient_size(self.splits.len(), self.max_key_capacity)
  }

  pub fn sibbling_identifiers_for_idx(
    &self,
    idx: usize,
  ) -> (Option<&str>, Option<&str>) {
    let left_sibbling_identifier = if 0 < idx {
      Some(self.child_identifier_by_idx(idx - 1))
    } else {
      None
    };

    let right_sibbling_identifier =
      if idx < self.child_identifiers.len() - 1 {
        Some(self.child_identifier_by_idx(idx + 1))
      } else {
        None
      };

    (left_sibbling_identifier, right_sibbling_identifier)
  }

  pub fn upcast(self) -> Node {
    Node::InteriorNode(self)
  }
}
