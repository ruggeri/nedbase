use super::{
  MergeWithSibblingAction, UnderflowActionResult,
  UpdateRootIdentifierAction,
};
use btree::BTree;
use locking::{LockSetNodeWriteGuard, LockSetRootIdentifierWriteGuard};

pub enum UnderflowAction {
  MergeWithSibbling(MergeWithSibblingAction),
  UpdateRootIdentifier(UpdateRootIdentifierAction),
}

impl UnderflowAction {
  pub fn new_merge_with_sibbling_action(
    parent_node_guard: LockSetNodeWriteGuard,
    child_node_guard: LockSetNodeWriteGuard,
    sibbling_node_guard: LockSetNodeWriteGuard,
  ) -> UnderflowAction {
    UnderflowAction::MergeWithSibbling(MergeWithSibblingAction {
      parent_node_guard,
      child_node_guard,
      sibbling_node_guard,
    })
  }

  pub fn new_update_root_identifier_action(
    root_identifier_guard: LockSetRootIdentifierWriteGuard,
    root_node_guard: LockSetNodeWriteGuard,
  ) -> UnderflowAction {
    UnderflowAction::UpdateRootIdentifier(UpdateRootIdentifierAction {
      root_identifier_guard,
      root_node_guard,
    })
  }

  pub fn path_node_guard(&self) -> &LockSetNodeWriteGuard {
    match self {
      UnderflowAction::MergeWithSibbling(action) => {
        &action.child_node_guard
      }

      UnderflowAction::UpdateRootIdentifier(action) => {
        &action.root_node_guard
      }
    }
  }

  pub fn path_node_guard_mut(&mut self) -> &mut LockSetNodeWriteGuard {
    match self {
      UnderflowAction::MergeWithSibbling(ref mut action) => {
        &mut action.child_node_guard
      }

      UnderflowAction::UpdateRootIdentifier(ref mut action) => {
        &mut action.root_node_guard
      }
    }
  }

  pub fn execute(
    self,
    btree: &BTree,
  ) -> UnderflowActionResult {
    match self {
      UnderflowAction::MergeWithSibbling(action) => {
        action.execute(btree)
      }

      UnderflowAction::UpdateRootIdentifier(action) => {
        action.execute()
      }
    }
  }
}
