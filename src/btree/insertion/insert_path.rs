pub enum InsertPathEntry {
  // For each node we visit, we also keep track of its parent for split
  // unwinding purposes.
  ParentChild {
    parent_node_identifier: String,
    // This may change if we walk right.
    current_node_identifier: String,
  },

  // When we start our descent, we start from a node we *think* is the
  // root (it was at the time we read the root identifier). However, we
  // may immediately have to walk right from the alleged root. Thus, I
  // only call this the *current_node_identifier*.
  //
  // Anyway, if we unwind the path because of splitting, it may be that
  // we must split this node. That may mean we have to create a new
  // root. OR, it may mean that we need to redescend down to learn the
  // new path down, so that we can continue our ascent.
  RootLevelNode {
    // This can be different from the starting root if we walk right
    // from the root.
    alleged_root_identifier: String,
  },
}

impl InsertPathEntry {
  pub fn current_node_identifier(&self) -> &String {
    match self {
      InsertPathEntry::RootLevelNode {
        alleged_root_identifier: ref current_node_identifier,
        ..
      }
      | InsertPathEntry::ParentChild {
        ref current_node_identifier,
        ..
      } => current_node_identifier,
    }
  }

  // We'll update this identifier if we ever have to walk right because
  // we didn't know about a split.
  pub fn update_current_node_identifier(
    &mut self,
    new_current_node_identifier: String,
  ) {
    match self {
      InsertPathEntry::RootLevelNode {
        alleged_root_identifier: ref mut current_node_identifier,
        ..
      }
      | InsertPathEntry::ParentChild {
        ref mut current_node_identifier,
        ..
      } => *current_node_identifier = new_current_node_identifier,
    }
  }
}
