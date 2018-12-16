use super::DeletionActionResult;
use locking::{LockSetNodeWriteGuard, LockSetRootIdentifierWriteGuard};

pub struct UpdateRootIdentifierAction {
  pub(super) root_identifier_guard: LockSetRootIdentifierWriteGuard,
  pub(super) root_node_guard: LockSetNodeWriteGuard,
}

impl UpdateRootIdentifierAction {
  pub fn execute(&self) -> DeletionActionResult {
    let new_root_identifier = {
      // First, get the root_node.
      let root_node = self.root_node_guard.unwrap_node_ref();

      // Next, if the root node is a leaf, there is nothing to do.
      if root_node.is_leaf_node() {
        return DeletionActionResult::StopBubbling;
      }

      // Okay, so the root is an InteriorNode...
      let root_node = root_node.unwrap_interior_node_ref(
        "can't reduce depth if root is already LeafNode",
      );

      // We will only "consume" the root and decrease the height of
      // the BTree when the root has a *single child*.
      if root_node.num_children() > 1 {
        return DeletionActionResult::StopBubbling;
      }

      // Okay! We do want to pull up the root. The new root will be the
      // only child of the root node. So we get its identifier.
      String::from(root_node.child_identifier_by_idx(0))
    };

    // Now set the root identifier to finish the compaction.
    let mut root_identifier =
      self.root_identifier_guard.identifier_mut();
    *root_identifier = new_root_identifier;

    // Now that we have a new root everything is completed.
    DeletionActionResult::StopBubbling
  }
}
