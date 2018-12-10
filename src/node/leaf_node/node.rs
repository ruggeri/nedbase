use node::util::search_sorted_strings_for_str;
use node::Node;

pub struct LeafNode {
  pub(super) identifier: String,
  pub(super) keys: Vec<String>,
  pub(super) max_key_capacity: usize,
}

impl LeafNode {
  pub fn new(
    identifier: String,
    keys: Vec<String>,
    max_key_capacity: usize,
  ) -> LeafNode {
    LeafNode {
      identifier,
      keys,
      max_key_capacity,
    }
  }

  pub fn can_delete_without_merge(&self) -> bool {
    self.max_key_capacity / 2 < self.keys.len()
  }

  pub fn can_grow_without_split(&self) -> bool {
    self.keys.len() < self.max_key_capacity
  }

  pub fn contains_key(&self, key: &str) -> bool {
    search_sorted_strings_for_str(&self.keys, key).is_ok()
  }

  pub fn identifier(&self) -> &str {
    &self.identifier
  }

  pub fn upcast(self) -> Node {
    Node::LeafNode(self)
  }
}
