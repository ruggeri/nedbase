use btree::util;
use locking::{LockSet, LockSetReadGuard};

// Acquires the parent of the deepest stable node. The deepest stable
// parent is the highest node that may need to be modified by a
// deletion.
pub fn acquire_parent_of_deepest_stable_node(
  lock_set: &mut LockSet,
  key_to_delete: &str,
) -> Option<LockSetReadGuard> {
  // TODO: This is a little more than needed. If the root is deficient,
  // that doesn't mean we necessarily have to lock the root identifier.
  // That is only true when we need to shrink the depth of the tree
  // (when the root has one item).
  //
  // That means that, if a merge of level-2 children make the root
  // deficient, we will unnecesarily think like we are going to merge
  // the root. Meaning we'll take the write lock on the root identifier.
  //
  // I *think* this isn't a big deal, since holding a write lock on the
  // root identifier isn't more contentious than holding a write lock on
  // the root node.
  util::acquire_parent_of_deepest_node_meeting_test(
    lock_set,
    key_to_delete,
    |node_ref| node_ref.can_delete_without_becoming_deficient(),
  )
}
