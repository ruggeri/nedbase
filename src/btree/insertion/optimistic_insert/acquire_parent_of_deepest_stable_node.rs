use btree::{util, BTree};
use locking::ReadGuard;
use std::sync::Arc;

// Acquires the parent of the deepest stable node. The deepest stable
// parent is the highest node that may need to be modified by an
// insertion.
pub fn acquire_parent_of_deepest_stable_node(
  btree: &Arc<BTree>,
  insert_key: &str,
) -> Option<ReadGuard> {
  util::acquire_parent_of_deepest_node_meeting_test(
    btree,
    insert_key,
    |node_ref| node_ref.can_grow_without_split(),
  )
}
