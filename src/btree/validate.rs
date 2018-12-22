use super::BTree;
use locking::LockSet;
use node::Node;

impl BTree {
  pub fn validate(&self, lock_set: &mut LockSet) {
    // Checking starts at the root.
    let root_identifier_guard = lock_set.temp_root_identifier_read_guard();
    let root_identifier = root_identifier_guard.identifier();

    Node::validate_root(lock_set, root_identifier.as_str());
  }
}
