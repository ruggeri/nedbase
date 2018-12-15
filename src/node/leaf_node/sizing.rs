use super::LeafNode;
use node::Node;

// These methods all pertain to the size of an LeafNode (number of
// keys).
impl LeafNode {
  pub fn can_delete_without_becoming_deficient(&self) -> bool {
    if self.keys.is_empty() {
      // Special case because else subtraction by one is dangerous!
      return false;
    }

    !Node::is_deficient_size(self.num_keys() - 1, self.max_key_capacity)
  }

  pub fn can_grow_without_split(&self) -> bool {
    self.keys.len() < self.max_key_capacity
  }

  pub fn is_deficient(&self) -> bool {
    Node::is_deficient_size(self.num_keys(), self.max_key_capacity)
  }

  pub fn num_keys(&self) -> usize {
    self.keys.len()
  }
}
