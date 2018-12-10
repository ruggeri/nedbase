use btree::{util, BTree};
use locking::ReadGuard;
use std::sync::Arc;

// Finds highest lock target that may need to be mutated by an
// insertion.
pub fn acquire_parent_of_stable_node(
  btree: &Arc<BTree>,
  insert_key: &str,
) -> Option<ReadGuard> {
  util::acquire_parent_of_deepest_node_meeting_test(
    btree,
    insert_key,
    |node_ref| node_ref.can_grow_without_split(),
  )
}
