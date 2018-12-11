use btree::deletion::UnderflowAction;

pub enum DeletionPathEntry {
  TopStableNode { node_identifier: String },

  UnstableNode { underflow_action: UnderflowAction },
}

impl DeletionPathEntry {
  pub fn new_top_stable_node_entry(
    node_identifier: String,
  ) -> DeletionPathEntry {
    DeletionPathEntry::TopStableNode { node_identifier }
  }

  pub fn new_update_root_identifier_entry(
    root_identifier: String,
  ) -> DeletionPathEntry {
    DeletionPathEntry::UnstableNode {
      underflow_action:
        UnderflowAction::new_update_root_identifier_action(
          root_identifier,
        ),
    }
  }

  pub fn new_merge_with_sibbling_entry(
    parent_identifier: String,
    child_identifier: String,
    sibbling_identifier: String,
  ) -> DeletionPathEntry {
    DeletionPathEntry::UnstableNode {
      underflow_action: UnderflowAction::new_merge_with_sibbling_action(
        parent_identifier,
        child_identifier,
        sibbling_identifier,
      ),
    }
  }

  pub fn path_node_identifier(&self) -> &str {
    match self {
      DeletionPathEntry::TopStableNode { node_identifier } => {
        &node_identifier
      }

      DeletionPathEntry::UnstableNode { underflow_action } => {
        &underflow_action.path_node_identifier()
      }
    }
  }
}
