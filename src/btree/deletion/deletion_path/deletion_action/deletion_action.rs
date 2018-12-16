use super::{
  DeleteKeyFromNodeAction, DeletionActionResult,
  MergeWithSibblingAction, UpdateRootIdentifierAction,
};
use btree::BTree;
use locking::{LockSet, LockSetNodeWriteGuard, LockSetRootIdentifierWriteGuard};

// DeletionAction represents one of three possibilities:
//
// 1. Delete a key from a LeafNode,
// 2. Merge two nodes because one went deficient.
// 3. Update root identifier if depth of tree shrinks.
pub enum DeletionAction {
  DeleteKeyFromNode(DeleteKeyFromNodeAction),
  MergeWithSibbling(MergeWithSibblingAction),
  UpdateRootIdentifier(UpdateRootIdentifierAction),
}

// Constructor functions to abstract away the underlying types of
// action. Also `DeletionAction#execute` method for the same purpose.
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

  pub fn execute(self, btree: &BTree, lock_set: &mut LockSet) -> DeletionActionResult {
    match self {
      DeletionAction::DeleteKeyFromNode(action) => action.execute(lock_set),

      DeletionAction::MergeWithSibbling(action) => {
        action.execute(btree, lock_set)
      }

      DeletionAction::UpdateRootIdentifier(action) => action.execute(),
    }
  }
}
