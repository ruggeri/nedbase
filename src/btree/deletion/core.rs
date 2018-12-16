use super::{acquire_deletion_path, DeletionActionResult};
use btree::BTree;
use locking::LockSet;
use std::sync::Arc;

pub fn delete(
  btree: &Arc<BTree>,
  lock_set: &mut LockSet,
  key_to_delete: &str,
) {
  // Acquire locks. `deletion_path` holds the locks, and remembers what
  // to do at each step.
  let mut deletion_path =
    acquire_deletion_path(lock_set, key_to_delete);

  // TODO: Super hacky way to hold onto held locks for 2PL.
  lock_set.freeze_held_guards();

  loop {
    // Perform each of the actions. The first one (presumably) will
    // delete the key.
    let action = deletion_path.pop_action();
    let result = action.execute(btree);

    // Action may have us stop if we hit a stable parent or consume the
    // root.
    if let DeletionActionResult::StopBubbling = result {
      return;
    }
  }
}
