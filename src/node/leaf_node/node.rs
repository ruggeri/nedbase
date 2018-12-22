use node::util::search_sorted_strings_for_str;
use node::{Node, StringComparisonValue, TraversalDirection};

#[derive(Debug)]
pub struct LeafNode {
  pub(super) identifier: String,
  pub(super) keys: Vec<String>,
  pub(super) max_value: StringComparisonValue<String>,
  pub(super) next_node_identifier: Option<String>,
  pub(super) max_key_capacity: usize,
}

impl LeafNode {
  pub fn contains_key(&self, key: &str) -> bool {
    search_sorted_strings_for_str(&self.keys, key).is_ok()
  }

  pub fn identifier(&self) -> &str {
    &self.identifier
  }

  pub fn keys(&self) -> &Vec<String> {
    &self.keys
  }

  pub fn max_value(&self) -> StringComparisonValue<&str> {
    self.max_value.as_ref()
  }

  pub fn next_node_identifier(&self) -> Option<&String> {
    self.next_node_identifier.as_ref()
  }

  pub fn traverse_toward(&self, key: &str) -> TraversalDirection {
    if self.max_value.is_ge_to(key) {
      TraversalDirection::Arrived
    } else {
      let next_node_identifier = self
        .next_node_identifier
        .clone()
        .expect("node with definite max value must have next");
      TraversalDirection::MoveRight {
        next_node_identifier,
      }
    }
  }

  pub fn upcast(self) -> Node {
    Node::LeafNode(self)
  }
}
