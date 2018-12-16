use super::DeletionActionResult;
use btree::BTree;
use locking::{LockSet, LockSetNodeWriteGuard};
use node::MergeOrRotateResult;

// This action merges (or rotates) a deficient node with a sibbling.
pub struct MergeWithSibblingAction {
  pub(super) parent_node_guard: LockSetNodeWriteGuard,
  pub(super) child_node_guard: LockSetNodeWriteGuard,
  pub(super) sibbling_node_guard: LockSetNodeWriteGuard,
}

impl MergeWithSibblingAction {
  pub fn execute(mut self, btree: &BTree, lock_set: &mut LockSet) -> DeletionActionResult {
    // Perform the deletion. Do mutable borrow for shortest period of
    // time.
    let merge_result = {
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
        )
    };

    // Now do read borrow since you won't modify any node for the rest
    // of the method.
    let parent_node = self.parent_node_guard.unwrap_node_ref();
    let child_node = self.child_node_guard.unwrap_node_ref();
    let sibbling_node =
      self.sibbling_node_guard.unwrap_node_ref();

    // If the leaf node (where the deletion occured) is split or is
    // rotated, we must hold locks on either its rotation sibbling or
    // the merged node.
    if child_node.is_leaf_node() {
      match merge_result {
        MergeOrRotateResult::DidMerge { merge_node_identifier } => {
          let merge_node_guard = lock_set.node_write_guard(&merge_node_identifier);
          lock_set.hold_node_write_guard(&merge_node_guard);
        }

        MergeOrRotateResult::DidRotate => {
          let sibbling_identifier = sibbling_node.identifier();
          let sibbling_node_guard = lock_set.node_write_guard(&sibbling_identifier);
          lock_set.hold_node_write_guard(&sibbling_node_guard);
        }
      }
    }

    // If after merge our parent is fine, we can stop.
    if parent_node.is_deficient() {
      DeletionActionResult::ContinueBubbling
    } else {
      DeletionActionResult::StopBubbling
    }
  }
}
