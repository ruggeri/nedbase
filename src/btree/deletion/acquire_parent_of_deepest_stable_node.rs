use btree::util;
use locking::{LockSet, LockSetReadGuard};

// Acquires the parent of the deepest stable node. The deepest stable
// parent is the highest node that may need to be modified by a
// deletion.
pub fn acquire_parent_of_deepest_stable_node(
  lock_set: &mut LockSet,
  key_to_delete: &str,
) -> Option<LockSetReadGuard> {
  util::acquire_parent_of_deepest_node_meeting_test(
    lock_set,
    key_to_delete,
    |node_ref| node_ref.can_delete_without_becoming_deficient(),
  )
}
