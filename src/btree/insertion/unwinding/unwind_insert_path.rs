use super::{redescend_toward_last_split, UnwindingResult};
use btree::{insertion::InsertPathEntry, BTree};
use locking::LockSet;
use node::SplitInfo;

// Unwinds a path, propagating splits up the tree.
pub fn unwind_insert_path(
  btree: &BTree,
  lock_set: &mut LockSet,
  mut insert_path: Vec<InsertPathEntry>,
  mut split_info: SplitInfo,
) {
  loop {
    // Pop one entry as we scroll back up the tree.
    let path_entry = match insert_path.pop() {
      None => panic!("Shouldn't ever run out of entries without at least reaching an alleged root node..."),
      Some(path_entry) => path_entry,
    };

    // Unwind the entry. I clone here because we may actually need to
    // reuse the split info (if we reloop around).
    let unwinding_result =
      path_entry.unwind_entry(btree, lock_set, split_info.clone());

    // Handle the result of unwinding the entry.
    match unwinding_result {
      // Either the parent didn't split when it handled the child's
      // split, OR
      UnwindingResult::FinishedUnwinding => return,

      // We must continue unwinding, OR
      UnwindingResult::MustContinueUnwinding(new_split_info) => {
        split_info = new_split_info;
      }

      // We couldn't continue unwinding becauuse we ran out of nodes. In
      // that case, redescend to learn the path down to the currently
      // split node. Then continue unwinding :-)
      UnwindingResult::MustRedescend => {
        insert_path =
          redescend_toward_last_split(lock_set, &split_info);
      }
    }
  }
}
