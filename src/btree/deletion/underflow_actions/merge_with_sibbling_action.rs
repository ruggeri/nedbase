use super::UnderflowActionResult;
use btree::BTree;
use locking::LockSetNodeWriteGuard;

pub struct MergeWithSibblingAction {
  pub(super) parent_node_guard: LockSetNodeWriteGuard,
  pub(super) child_node_guard: LockSetNodeWriteGuard,
  pub(super) sibbling_node_guard: LockSetNodeWriteGuard,
}

// Helper struct that performs the merge operation.
impl MergeWithSibblingAction {
  pub fn execute(
    mut self,
    btree: &BTree,
  ) -> UnderflowActionResult {
    // Get the write locks you've acquired on everyone.
    let mut parent_node = self.parent_node_guard.node_mut();
    let mut child_node = self.child_node_guard.node_mut();
    let mut sibbling_node = self.sibbling_node_guard.node_mut();

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
      UnderflowActionResult::ContinueBubbling
    } else {
      UnderflowActionResult::StopBubbling
    }
  }
}
