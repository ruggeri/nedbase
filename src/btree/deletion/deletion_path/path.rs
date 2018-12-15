use super::DeletionPathEntry;
use locking::{LockSet, LockSetNodeWriteGuard};
use node::Node;
use std::cell::{Ref, RefMut};

pub struct DeletionPath {
  entries: Vec<DeletionPathEntry>,
}

impl DeletionPath {
  // When we are unstable all the way to the root, we acquire write
  // locks on both the root identifier AND the root node.
  pub fn new_from_unstable_root(
    lock_set: &mut LockSet,
  ) -> DeletionPath {
    let root_identifier_guard = lock_set.root_identifier_write_guard_for_hold();
    let root_node_guard = lock_set.node_write_guard_for_hold(&root_identifier_guard.identifier());

    DeletionPath {
      entries: vec![
        DeletionPathEntry::new_update_root_identifier_entry(
          root_identifier_guard,
          root_node_guard,
        ),
      ],
    }
  }

  // When there is a top stable node, we acquire a write lock on it.
  pub fn new_from_stable_parent(
    lock_set: &mut LockSet,
    identifier: &str,
  ) -> DeletionPath {
    let stable_node_guard = lock_set.node_write_guard_for_hold(identifier);

    DeletionPath {
      entries: vec![DeletionPathEntry::new_top_stable_node_entry(
        stable_node_guard
      )],
    }
  }

  // The last node that was added to this path.
  pub fn last_node_guard_ref(
    &self,
  ) -> &LockSetNodeWriteGuard {
    self.last_path_entry_ref().path_node_guard()
  }

  // The last node that was added to this path.
  pub fn last_node_guard_mut_ref(
    &mut self,
  ) -> &mut LockSetNodeWriteGuard {
    self.last_path_entry_mut_ref().path_node_guard_mut()
  }

  // The last node that was added to this path.
  pub fn last_node_ref(
    &self,
  ) -> Ref<Node> {
    self.last_node_guard_ref().node()
  }

  // The last node that was added to this path.
  pub fn last_node_mut_ref(
    &mut self,
  ) -> RefMut<Node> {
    self.last_node_guard_mut_ref().node_mut()
  }

  // The last entry that was added to this path.
  pub fn last_path_entry_ref(&self) -> &DeletionPathEntry {
    self
      .entries
      .last()
      .expect("path was empty: cannot get last entry")
  }

  // The last entry that was added to this path.
  pub fn last_path_entry_mut_ref(&mut self) -> &mut DeletionPathEntry {
    self
      .entries
      .last_mut()
      .expect("path was empty: cannot get last entry")
  }

  // Pops the most recent entry.
  pub fn pop_last_path_entry(&mut self) -> DeletionPathEntry {
    self
      .entries
      .pop()
      .expect("path was empty: cannot pop last entry")
  }

  // Pushes on a new entry.
  pub fn push(&mut self, path_entry: DeletionPathEntry) {
    self.entries.push(path_entry);
  }
}
