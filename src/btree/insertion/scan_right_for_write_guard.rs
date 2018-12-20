use locking::{LockSet, LockSetNodeWriteGuard};
use node::TraversalDirection;

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
        return current_guard;
      }
      TraversalDirection::MoveDown { .. } => {
        // We are only moving right.
        return current_guard;
      }
      TraversalDirection::MoveRight {
        next_node_identifier,
      } => {
        current_identifier = next_node_identifier;
      }
    }
  }
}
