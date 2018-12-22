use super::InteriorNode;
use locking::LockSet;
use node::{Node, StringComparisonValue};

impl InteriorNode {
  pub fn validate(
    &self,
    lock_set: &mut LockSet,
    min_value: StringComparisonValue<&str>,
    max_value: StringComparisonValue<&str>,
  ) {
    // All keys must be greater than the low limit.
    let mut prev_split_value = min_value;

    // max_value passed in from parent should equal the max_value of
    // the node.
    if max_value != self.max_value() {
      panic!("max_value should equal max_value passed in from parent");
    }

    for (idx, split_value) in self.splits().iter().enumerate() {
      // Keys must be in ascending order (with no duplicates).
      if prev_split_value.is_ge_to(split_value) {
        println!("{}", split_value);
        println!("{:?}", prev_split_value);
        panic!("Keys are out of order!");
      }

      // All values must be less than or equal to the max_value.
      if !max_value.is_ge_to(split_value) {
        println!("{}", split_value);
        println!("{:?}", max_value);
        panic!("max_value disobeyed");
      }

      // Recursively check each child node.
      let child_identifier = self.child_identifier_by_idx(idx);
      Node::validate(
        lock_set,
        child_identifier,
        prev_split_value,
        StringComparisonValue::DefiniteValue(split_value),
      );

      prev_split_value =
        StringComparisonValue::DefiniteValue(split_value);
    }

    // There's one last child we didn't check!
    let child_identifier =
      self.child_identifier_by_idx(self.num_split_keys());
    Node::validate(
      lock_set,
      child_identifier,
      prev_split_value,
      max_value,
    );
  }
}
