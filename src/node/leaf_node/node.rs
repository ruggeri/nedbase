use node::util::search_sorted_strings_for_str;
use node::Node;

pub struct LeafNode {
  pub(super) identifier: String,
  pub(super) keys: Vec<String>,
  pub(super) max_key_capacity: usize,
}

impl LeafNode {
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
