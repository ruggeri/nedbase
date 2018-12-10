use super::write_set::WriteSet;
use btree::BTree;
use locking::NodeWriteGuard;
use std::sync::Arc;

pub enum DeletionPathEntry {
  UnstableRootNode {
    root_identifier: String,
  },

  TopStableNode {
    node_identifier: String,
  },

  NodeWithMergeSibbling {
    parent_node_identifier: String,
    path_node_identifier: String,
    sibbling_node_identifier: String,
  },
}

pub struct DeletionPath {
  entries: Vec<DeletionPathEntry>,
}

impl DeletionPath {
  // When we are unstable all the way to the root, we acquire write
  // locks on both the root identifier AND the root node.
  pub fn new_from_unstable_root(
    btree: &Arc<BTree>,
    write_set: &mut WriteSet,
  ) -> DeletionPath {
    let root_identifier = String::from(
      write_set.acquire_root_identifier(btree).as_str_ref(),
    );
    write_set.acquire_node_guard(btree, &root_identifier);

    DeletionPath {
      entries: vec![DeletionPathEntry::UnstableRootNode {
        root_identifier: root_identifier,
      }],
    }
  }

  // When there is a top stable node, we acquire a write lock on it.
  pub fn new_from_stable_parent(
    btree: &Arc<BTree>,
    write_set: &mut WriteSet,
    identifier: &str,
  ) -> DeletionPath {
    write_set.acquire_node_guard(btree, identifier);

    DeletionPath {
      entries: vec![DeletionPathEntry::TopStableNode {
        node_identifier: String::from(identifier),
      }],
    }
  }

  // The last identifier that was added to this path.
  pub fn last_identifier_of_path(&self) -> &str {
    let last_entry = self.last_path_entry();
    match last_entry {
      DeletionPathEntry::UnstableRootNode { root_identifier } => {
        &root_identifier
      }
      DeletionPathEntry::TopStableNode { node_identifier } => {
        &node_identifier
      }
      DeletionPathEntry::NodeWithMergeSibbling {
        path_node_identifier,
        ..
      } => &path_node_identifier,
    }
  }

  // The last node that was added to this path.
  pub fn last_node<'a>(
    &self,
    write_set: &'a WriteSet,
  ) -> &'a NodeWriteGuard {
    let last_identifier = self.last_identifier_of_path();
    write_set.get_node(last_identifier)
  }

  // The last node that was added to this path.
  pub fn last_node_mut<'a>(
    &self,
    write_set: &'a mut WriteSet,
  ) -> &'a mut NodeWriteGuard {
    let last_identifier = self.last_identifier_of_path();
    write_set.get_mut_node(last_identifier)
  }

  // The last entry that was added to this path.
  pub fn last_path_entry(&self) -> &DeletionPathEntry {
    self
      .entries
      .last()
      .expect("path was empty: cannot get last entry")
  }

  // Pops the most recent entry.
  pub fn pop_last_path_entry(&mut self) -> DeletionPathEntry {
    self
      .entries
      .pop()
      .expect("path was empty: cannot pop last entry")
  }

  // Pushes on a new entry. Intended to always be a
  // NodeWithMergeSibbling, but this is not enforced here...
  pub fn push(&mut self, path_entry: DeletionPathEntry) {
    self.entries.push(path_entry);
  }
}
