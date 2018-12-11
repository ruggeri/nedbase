use super::DeletionPathEntry;
use btree::deletion::WriteSet;
use btree::BTree;
use locking::NodeWriteGuard;
use std::sync::Arc;

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
    let root_node_identifier = String::from(
      write_set.acquire_root_identifier(btree).as_str_ref(),
    );
    write_set.acquire_node_guard(btree, &root_node_identifier);

    DeletionPath {
      entries: vec![
        DeletionPathEntry::new_update_root_identifier_entry(
          root_node_identifier,
        ),
      ],
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
      entries: vec![DeletionPathEntry::new_top_stable_node_entry(
        String::from(identifier),
      )],
    }
  }

  // The last identifier that was added to this path.
  pub fn last_identifier_of_path(&self) -> &str {
    let last_entry = self.last_path_entry_ref();
    last_entry.path_node_identifier()
  }

  // The last node that was added to this path.
  pub fn last_node_ref<'a>(
    &self,
    write_set: &'a WriteSet,
  ) -> &'a NodeWriteGuard {
    let last_identifier = self.last_identifier_of_path();
    write_set.get_node_ref(last_identifier)
  }

  // The last node that was added to this path.
  pub fn last_node_mut_ref<'a>(
    &self,
    write_set: &'a mut WriteSet,
  ) -> &'a mut NodeWriteGuard {
    let last_identifier = self.last_identifier_of_path();
    write_set.get_node_mut_ref(last_identifier)
  }

  // The last entry that was added to this path.
  pub fn last_path_entry_ref(&self) -> &DeletionPathEntry {
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

  // Pushes on a new entry.
  pub fn push(&mut self, path_entry: DeletionPathEntry) {
    self.entries.push(path_entry);
  }
}
