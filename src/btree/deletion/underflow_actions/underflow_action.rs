use super::{
  MergeWithSibblingAction, UnderflowActionResult,
  UpdateRootIdentifierAction,
};
use btree::deletion::WriteSet;
use btree::BTree;

pub enum UnderflowAction {
  MergeWithSibbling(MergeWithSibblingAction),
  UpdateRootIdentifier(UpdateRootIdentifierAction),
}

impl UnderflowAction {
  pub fn new_merge_with_sibbling_action(
    parent_node_identifier: String,
    child_node_identifier: String,
    sibbling_node_identifier: String,
  ) -> UnderflowAction {
    UnderflowAction::MergeWithSibbling(MergeWithSibblingAction {
      parent_node_identifier,
      child_node_identifier,
      sibbling_node_identifier,
    })
  }

  pub fn new_update_root_identifier_action(
    root_node_identifier: String,
  ) -> UnderflowAction {
    UnderflowAction::UpdateRootIdentifier(UpdateRootIdentifierAction {
      root_node_identifier,
    })
  }

  pub fn path_node_identifier(&self) -> &str {
    match self {
      UnderflowAction::MergeWithSibbling(action) => {
        &action.child_node_identifier
      }

      UnderflowAction::UpdateRootIdentifier(action) => {
        &action.root_node_identifier
      }
    }
  }

  pub fn execute(
    &self,
    btree: &BTree,
    write_set: &mut WriteSet,
  ) -> UnderflowActionResult {
    match self {
      UnderflowAction::MergeWithSibbling(action) => {
        action.execute(btree, write_set)
      }

      UnderflowAction::UpdateRootIdentifier(action) => {
        action.execute(write_set)
      }
    }
  }
}
