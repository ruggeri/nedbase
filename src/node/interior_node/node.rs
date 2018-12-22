use node::{MaxValue, Node};

#[derive(Debug)]
pub struct InteriorNode {
  // These fields are public in the `interior_node` module, as other
  // methods of InteriorNode will need them and are defined in sibbling
  // modules.
  pub(super) identifier: String,
  // The rule is that all keys such that `target_key <= keys[idx]` live
  // in child `idx`.
  //
  // Another rule is that for interior nodes the number of child
  // identifiers is always one more than the number of keys.
  pub(super) splits: Vec<String>,
  pub(super) child_identifiers: Vec<String>,
  pub(super) max_value: MaxValue,
  pub(super) next_node_identifier: Option<String>,
  pub(super) max_key_capacity: usize,
}

impl InteriorNode {
  pub fn identifier(&self) -> &str {
    &self.identifier
  }

  pub fn max_value(&self) -> &MaxValue {
    &self.max_value
  }

  pub fn next_node_identifier(&self) -> Option<&String> {
    self.next_node_identifier.as_ref()
  }

  pub fn splits(&self) -> &Vec<String> {
    &self.splits
  }

  // It is sometimes useful to convert InteriorNode to a Node for
  // purposes of storage in places where you wouldn't otherwise know
  // what kind of Node you need.
  pub fn upcast(self) -> Node {
    Node::InteriorNode(self)
  }
}
