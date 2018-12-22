use super::Node;
use locking::LockSet;
use node::StringComparisonValue;

impl Node {
  pub fn validate(
    lock_set: &mut LockSet,
    node_identifier: &str,
    min_value: StringComparisonValue<&str>,
    max_value: StringComparisonValue<&str>,
  ) {
    let child_guard = lock_set.temp_node_read_guard(node_identifier);
    let child_node_ref = child_guard.unwrap_node_ref();

    match &(*child_node_ref) {
      Node::InteriorNode(inode) => {
        inode.validate(lock_set, min_value, max_value);
      }

      Node::LeafNode(lnode) => lnode.validate(min_value, max_value),
    }
  }

  pub fn validate_root(lock_set: &mut LockSet, node_identifier: &str) {
    Node::validate(
      lock_set,
      node_identifier,
      StringComparisonValue::NegativeInfinity,
      StringComparisonValue::Infinity,
    );
  }
}
