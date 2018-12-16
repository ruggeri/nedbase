use super::DeletionActionResult;
use btree::BTree;
use locking::LockSetNodeWriteGuard;

// This action merges (or rotates) a deficient node with a sibbling.
pub struct MergeWithSibblingAction {
  pub(super) parent_node_guard: LockSetNodeWriteGuard,
  pub(super) child_node_guard: LockSetNodeWriteGuard,
  pub(super) sibbling_node_guard: LockSetNodeWriteGuard,
}

impl MergeWithSibblingAction {
  pub fn execute(mut self, btree: &BTree) -> DeletionActionResult {
    // Get the write locks you've acquired on everyone.
    let mut parent_node = self.parent_node_guard.unwrap_node_mut_ref();
    let mut child_node = self.child_node_guard.unwrap_node_mut_ref();
    let mut sibbling_node =
      self.sibbling_node_guard.unwrap_node_mut_ref();

    // And then have the parent perform the merge or rotation.
    parent_node
      .unwrap_interior_node_mut_ref("parents must be InteriorNodes")
      .merge_or_rotate_children(
        btree,
        &mut child_node,
        &mut sibbling_node,
      );

    // If after merge our parent is fine, we can stop.
    if parent_node.is_deficient() {
      DeletionActionResult::ContinueBubbling
    } else {
      DeletionActionResult::StopBubbling
    }
  }
}
