use btree::deletion::UnderflowAction;
use locking::{LockSetNodeWriteGuard, LockSetRootIdentifierWriteGuard};

pub enum DeletionPathEntry {
  TopStableNode {
    stable_node_guard: LockSetNodeWriteGuard,
  },

  UnstableNode {
    underflow_action: UnderflowAction,
  },
}

impl DeletionPathEntry {
  pub fn new_top_stable_node_entry(
    stable_node_guard: LockSetNodeWriteGuard,
  ) -> DeletionPathEntry {
    DeletionPathEntry::TopStableNode { stable_node_guard }
  }

  pub fn new_update_root_identifier_entry(
    root_identifier_guard: LockSetRootIdentifierWriteGuard,
    root_node_guard: LockSetNodeWriteGuard,
  ) -> DeletionPathEntry {
    DeletionPathEntry::UnstableNode {
      underflow_action:
        UnderflowAction::new_update_root_identifier_action(
          root_identifier_guard,
          root_node_guard,
        ),
    }
  }

  pub fn new_merge_with_sibbling_entry(
    parent_node_guard: LockSetNodeWriteGuard,
    child_node_guard: LockSetNodeWriteGuard,
    sibbling_node_guard: LockSetNodeWriteGuard,
  ) -> DeletionPathEntry {
    DeletionPathEntry::UnstableNode {
      underflow_action: UnderflowAction::new_merge_with_sibbling_action(
        parent_node_guard,
        child_node_guard,
        sibbling_node_guard,
      ),
    }
  }

  pub fn path_node_guard(&self) -> &LockSetNodeWriteGuard {
    match self {
      DeletionPathEntry::TopStableNode { stable_node_guard } => {
        &stable_node_guard
      }

      DeletionPathEntry::UnstableNode { underflow_action } => {
        &underflow_action.path_node_guard()
      }
    }
  }

  pub fn path_node_guard_mut(&mut self) -> &mut LockSetNodeWriteGuard {
    match self {
      DeletionPathEntry::TopStableNode {
        ref mut stable_node_guard,
      } => stable_node_guard,

      DeletionPathEntry::UnstableNode {
        ref mut underflow_action,
      } => underflow_action.path_node_guard_mut(),
    }
  }
}
