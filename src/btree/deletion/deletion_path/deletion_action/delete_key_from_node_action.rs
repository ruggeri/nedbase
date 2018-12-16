use super::DeletionActionResult;
use locking::{LockSet, LockSetNodeWriteGuard};

// This action deletes the target key from the LeafNode.
pub struct DeleteKeyFromNodeAction {
  pub(super) key_to_delete: String,
  pub(super) node_guard: LockSetNodeWriteGuard,
}

impl DeleteKeyFromNodeAction {
  pub fn execute(mut self, lock_set: &mut LockSet) -> DeletionActionResult {
    // If we mutate a node, we must hold it throughout the transaction
    // for 2PL purposes.
    lock_set.hold_node_write_guard(&self.node_guard);

    // Perform the delete.
    let mut node_ref = self.node_guard.unwrap_node_mut_ref();
    let leaf_node = node_ref
      .unwrap_leaf_node_mut_ref("deletion must happen at a leaf node");
    leaf_node.delete(&self.key_to_delete);

    // Can avoid any bubbling if leaf node never goes deficient.
    if !leaf_node.is_deficient() {
      DeletionActionResult::StopBubbling
    } else {
      DeletionActionResult::ContinueBubbling
    }
  }
}
