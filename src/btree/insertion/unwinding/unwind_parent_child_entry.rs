use super::UnwindingResult;
use btree::{
  BTree,
  insertion::scan_right_for_write_guard,
};
use locking::LockSet;
use node::{InsertionResult, SplitInfo};

// Handle the split of the child at the parent. This may split the
// parent, requiring further unwinding.
pub fn unwind_parent_child_entry(
  btree: &BTree,
  lock_set: &mut LockSet,
  parent_node_identifier: &str,
  split_info: SplitInfo,
) -> UnwindingResult {
  // Acquire write guard on the parent; or wherever we should be
  // inserting this newly split child.
  let parent_guard = scan_right_for_write_guard(
    lock_set,
    parent_node_identifier,
    &split_info.new_median,
  );

  // Unwrap the parent node.
  let mut parent_node = parent_guard
    .unwrap_interior_node_mut_ref("only interior nodes can be parents");

  // Handle the split at the node. Maybe we have to continue unwinding.
  match parent_node.handle_split(btree, split_info) {
    InsertionResult::DidInsertWithSplit(new_split_info) => {
      UnwindingResult::MustContinueUnwinding(new_split_info)
    }

    _ => UnwindingResult::FinishedUnwinding,
  }
}
