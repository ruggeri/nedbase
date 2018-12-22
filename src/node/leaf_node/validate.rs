use super::LeafNode;
use node::StringComparisonValue;

impl LeafNode {
  pub fn validate(
    &self,
    min_value: StringComparisonValue<&str>,
    max_value: StringComparisonValue<&str>,
  ) {
    // All keys must be greater than the low limit.
    let mut prev_value = min_value;
    for key in self.keys() {
      // Keys must be in ascending order (with no duplicates).
      if prev_value.is_ge_to(key) {
        println!("{}", key);
        println!("{:?}", prev_value);
        panic!("Keys are out of order!");
      }

      // All values must be less than or equal to the high limit.
      if !max_value.is_ge_to(key) {
        println!("{}", key);
        println!("{:?}", max_value);
        panic!("High limit disobeyed");
      }

      prev_value = StringComparisonValue::DefiniteValue(key);
    }
  }
}
