pub enum InsertPathEntry {
  ParentChild {
    parent_node_identifier: String,
    child_node_identifier: String,
  },

  RootLevelNode {
    root_node_identifier: String,
    root_level_identifier: String,
    // This can be different if we walk right from the root.
    current_node_identifier: String,
  },
}

impl InsertPathEntry {
  pub fn current_node_identifier(&self) -> &String {
    use self::InsertPathEntry::*;

    match self {
      RootLevelNode {
        ref current_node_identifier,
        ..
      } => current_node_identifier,
      ParentChild {
        ref child_node_identifier,
        ..
      } => child_node_identifier,
    }
  }
}
