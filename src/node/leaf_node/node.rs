use node::{
  util::search_sorted_strings_for_str
};

pub struct LeafNode {
  pub(super) identifier: String,
  pub(super) keys: Vec<String>,
  pub(super) max_key_capacity: usize,
}

impl LeafNode {
  pub fn new(identifier: String, keys: Vec<String>, max_key_capacity: usize) -> LeafNode {
    LeafNode { identifier, keys, max_key_capacity }
  }

  pub fn contains_key(&self, key: &str) -> bool {
    search_sorted_strings_for_str(&self.keys, key).is_ok()
  }

  pub fn can_grow_without_split(&self) -> bool {
    self.keys.len() < self.max_key_capacity
  }

  pub fn identifier(&self) -> &str {
    &self.identifier
  }
}
