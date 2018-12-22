use super::{UnwindingResult, unwind_parent_child_entry, unwind_root_level_entry};
use btree::{
  BTree,
  insertion::InsertPathEntry,
};
use locking::LockSet;
use node::SplitInfo;

impl InsertPathEntry {
  // Unwind one entry of the path, handling a split that occurred.
  pub fn unwind_entry(
    self,
    btree: &BTree,
    lock_set: &mut LockSet,
    split_info: SplitInfo,
  ) -> UnwindingResult {
    match self {
      InsertPathEntry::ParentChild {
        parent_node_identifier,
        ..
      } => unwind_parent_child_entry(
        btree,
        lock_set,
        &parent_node_identifier,
        split_info,
      ),

      InsertPathEntry::RootLevelNode { alleged_root_identifier } => {
        unwind_root_level_entry(
          btree,
          lock_set,
          alleged_root_identifier,
          split_info,
        )
      }
    }
  }
}
