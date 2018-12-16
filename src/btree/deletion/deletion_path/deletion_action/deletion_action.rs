use super::{
  DeleteKeyFromNodeAction, DeletionActionResult,
  MergeWithSibblingAction, UpdateRootIdentifierAction,
};
use btree::BTree;
use locking::{LockSetNodeWriteGuard, LockSetRootIdentifierWriteGuard};

pub enum DeletionAction {
  DeleteKeyFromNode(DeleteKeyFromNodeAction),
  MergeWithSibbling(MergeWithSibblingAction),
  UpdateRootIdentifier(UpdateRootIdentifierAction),
}

impl DeletionAction {
  pub fn delete_key_from_node(
    node_guard: LockSetNodeWriteGuard,
    key_to_delete: &str,
  ) -> DeletionAction {
    let action = DeleteKeyFromNodeAction {
      node_guard,
      key_to_delete: String::from(key_to_delete),
    };

    DeletionAction::DeleteKeyFromNode(action)
  }

  pub fn merge_with_sibbling(
    parent_node_guard: LockSetNodeWriteGuard,
    child_node_guard: LockSetNodeWriteGuard,
    sibbling_node_guard: LockSetNodeWriteGuard,
  ) -> DeletionAction {
    let action = MergeWithSibblingAction {
      parent_node_guard,
      child_node_guard,
      sibbling_node_guard,
    };

    DeletionAction::MergeWithSibbling(action)
  }

  pub fn update_root_identifier(
    root_identifier_guard: LockSetRootIdentifierWriteGuard,
    root_node_guard: LockSetNodeWriteGuard,
  ) -> DeletionAction {
    let action = UpdateRootIdentifierAction {
      root_identifier_guard,
      root_node_guard,
    };

    DeletionAction::UpdateRootIdentifier(action)
  }

  pub fn execute(self, btree: &BTree) -> DeletionActionResult {
    match self {
      DeletionAction::DeleteKeyFromNode(action) => action.execute(),

      DeletionAction::MergeWithSibbling(action) => {
        action.execute(btree)
      }

      DeletionAction::UpdateRootIdentifier(action) => action.execute(),
    }
  }
}
