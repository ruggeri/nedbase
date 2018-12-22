use super::BTree;
use locking::LockSet;
use node::Node;

impl BTree {
  pub fn validate(&self, lock_set: &mut LockSet) {
    let current_node_identifier = {
      let guard = lock_set.temp_root_identifier_read_guard();
      let guard_identifier = guard.identifier();
      guard_identifier.clone()
    };

    // Checking starts at the root.
    Node::validate_root(lock_set, &current_node_identifier);
  }
}
