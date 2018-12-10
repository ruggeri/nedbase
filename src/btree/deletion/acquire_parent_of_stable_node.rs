use btree::{util, BTree};
use locking::ReadGuard;
use std::sync::Arc;

// Finds highest lock target that may need to be mutated by an
// insertion.
pub fn acquire_parent_of_stable_node(
  btree: &Arc<BTree>,
  key_to_delete: &str,
) -> Option<ReadGuard> {
  util::acquire_parent_of_deepest_node_meeting_test(
    btree,
    key_to_delete,
    |node_ref| node_ref.can_delete_without_becoming_deficient(),
  )
}
