use btree::util;
use locking::{LockSet, LockSetReadGuard};

// Acquires the parent of the deepest stable node. The deepest stable
// parent is the highest node that may need to be modified by an
// insertion.
pub fn acquire_parent_of_deepest_stable_node(
  lock_set: &mut LockSet,
  insert_key: &str,
) -> Option<LockSetReadGuard> {
  util::acquire_parent_of_deepest_node_meeting_test(
    lock_set,
    insert_key,
    |node_ref| node_ref.can_grow_without_split(),
  )
}
