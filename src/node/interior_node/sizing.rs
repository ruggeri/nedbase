use super::InteriorNode;
use node::Node;

// These methods all pertain to the size of an InteriorNode (number of
// keys/children).
impl InteriorNode {
  pub fn can_delete_without_becoming_deficient(&self) -> bool {
    if self.splits.is_empty() {
      // Special case because else subtraction by one is dangerous!
      return false;
    }

    !Node::_is_deficient(
      self.num_split_keys() - 1,
      self.max_key_capacity,
    )
  }

  pub fn can_grow_without_split(&self) -> bool {
    self.num_split_keys() < self.max_key_capacity
  }

  pub fn is_deficient(&self) -> bool {
    Node::_is_deficient(self.num_split_keys(), self.max_key_capacity)
  }

  pub fn is_overfull(&self) -> bool {
    self.num_split_keys() > self.max_key_capacity
  }

  pub fn num_children(&self) -> usize {
    self.child_identifiers.len()
  }

  pub fn num_split_keys(&self) -> usize {
    self.splits.len()
  }
}
