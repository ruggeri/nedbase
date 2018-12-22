use super::UnwindingResult;
use btree::BTree;
use locking::LockSet;
use node::{InteriorNode, SplitInfo};

// We have split a node that we thought of as "root level." If this is
// the root, then we should update the root identifier. But if it no
// longer is, we must redescend to find the path further back.
pub fn unwind_root_level_entry(
  btree: &BTree,
  lock_set: &mut LockSet,
  alleged_root_identifier: String,
  split_info: SplitInfo,
) -> UnwindingResult {
  // First, acquire a write guard on the root identifier since we may
  // have to mutate it.
  //
  // TODO: It may be worth seeing if getting a temp read guard to check
  // if the root identifier changed before write locking decreases lock
  // contention.
  let root_id_guard = lock_set.root_identifier_write_guard();
  let mut root_identifier = root_id_guard.identifier_mut();

  // Did we actually reach the root? If not, let them know we must
  // continue unwinding.
  if alleged_root_identifier != *root_identifier {
    // Root split on us! Uh-oh! We have to redescend before we can
    // continue propagating splits further up.
    return UnwindingResult::MustRedescend;
  }

  // Okay! We actually are spliting the root for reals! Special day!
  *root_identifier = InteriorNode::store_new_root(
    btree,
    alleged_root_identifier,
    split_info,
  );

  UnwindingResult::FinishedUnwinding
}
