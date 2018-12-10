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

  pub fn can_delete_without_becoming_deficient(&self) -> bool {
    if self.keys.is_empty() {
      // Special case because else subtraction by one is dangerous!
      return false
    }

    !Node::is_deficient_size(self.keys.len() - 1, self.max_key_capacity)
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

  pub fn is_deficient(&self) -> bool {
    Node::is_deficient_size(self.keys.len(), self.max_key_capacity)
  }

  pub fn upcast(self) -> Node {
    Node::LeafNode(self)
  }
}
