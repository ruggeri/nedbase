use super::InteriorNode;
use node::util::search_sorted_strings_for_str;

// These methods are all ways to move from an InteriorNode to a child.
impl InteriorNode {
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
}
