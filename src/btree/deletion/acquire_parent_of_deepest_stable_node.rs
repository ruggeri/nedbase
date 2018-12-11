use btree::{util, BTree};
use locking::ReadGuard;
use std::sync::Arc;

// Acquires the parent of the deepest stable node. The deepest stable
// parent is the highest node that may need to be modified by a
// deletion.
pub fn acquire_parent_of_deepest_stable_node(
  btree: &Arc<BTree>,
  key_to_delete: &str,
) -> Option<ReadGuard> {
  util::acquire_parent_of_deepest_node_meeting_test(
    btree,
    key_to_delete,
    |node_ref| node_ref.can_delete_without_becoming_deficient(),
  )
}
