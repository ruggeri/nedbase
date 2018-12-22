use locking::{LockSet, LockSetNodeWriteGuard};
use node::TraversalDirection;

// This method tries to acquire a write lock on `start_identifier`, but
// in the case that the node has split, will move right to the
// appropriate node.
//
// This method *does not* move down the tree. It *only* scans right.
pub fn scan_right_for_write_guard(
  lock_set: &mut LockSet,
  start_identifier: &str,
  key: &str,
) -> LockSetNodeWriteGuard {
  let mut current_identifier = String::from(start_identifier);
  loop {
    let current_guard = lock_set.node_write_guard(&current_identifier);
    let direction =
      current_guard.unwrap_node_ref().traverse_toward(key);

    match direction {
      TraversalDirection::Arrived => {
        // If we're scanning at leaf level, we'll know to stop because
        // we'll have arrived at the leaf node.
        return current_guard;
      }

      TraversalDirection::MoveDown { .. } => {
        // If we are scanning at interior level, we'll know to stop
        // because we are told to move down a level.
        return current_guard;
      }

      TraversalDirection::MoveRight {
        next_node_identifier,
      } => {
        // Keep moving right!
        current_identifier = next_node_identifier;
      }
    }
  }
}
