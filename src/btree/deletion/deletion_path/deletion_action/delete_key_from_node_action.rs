use super::DeletionActionResult;
use locking::LockSetNodeWriteGuard;

pub struct DeleteKeyFromNodeAction {
  pub(super) key_to_delete: String,
  pub(super) node_guard: LockSetNodeWriteGuard,
}

// Helper struct that performs the merge operation.
impl DeleteKeyFromNodeAction {
  pub fn execute(mut self) -> DeletionActionResult {
    let mut node_ref = self.node_guard.unwrap_node_mut_ref();
    let leaf_node = node_ref
      .unwrap_leaf_node_mut_ref("deletion must happen at a leaf node");

    leaf_node.delete(&self.key_to_delete);

    // Can avoid any bubblingn if leaf node never goes deficient.
    if !leaf_node.is_deficient() {
      DeletionActionResult::StopBubbling
    } else {
      DeletionActionResult::ContinueBubbling
    }
  }
}
