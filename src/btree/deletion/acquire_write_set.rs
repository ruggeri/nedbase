use super::acquire_parent_of_stable_node::acquire_parent_of_stable_node;
use btree::BTree;
use locking::{NodeWriteGuard, ReadGuard, RootIdentifierWriteGuard, WriteGuard};
use std::collections::HashMap;
use std::sync::Arc;

pub enum WriteSetAcquistionResult {
  Success(WriteSet),
  TopNodeWentUnstable,
}

pub enum DeletionPathEntry {
  UnstableRootNode {
    root_identifier: String
  },
  TopStableNode {
    node_identifier: String
  },
  NodeWithMergeSibbling {
    path_node_identifier: String,
    sibbling_node_identifieR: String,
  },
}

pub struct WriteSet {
  map: HashMap<String, WriteGuard>,
  path: Vec<DeletionPathEntry>
}

impl WriteSet {
  pub fn new() -> WriteSet {
    WriteSet {
      map: HashMap::new(),
      path: Vec::new(),
    }
  }

  // When we are unstable all the way to the root, we acquire write
  // locks on both the root identifier AND the root node.
  pub fn acquire_unstable_root_node(&mut self, btree: &Arc<BTree>) {
    let root_identifier_guard = RootIdentifierWriteGuard::acquire(btree);
    let root_identifier = String::from(root_identifier_guard.as_str_ref());
    let unstable_root_node_guard = NodeWriteGuard::acquire(btree, &root_identifier);

    self.map.insert(String::from(""), root_identifier_guard.upcast());
    self.map.insert(root_identifier.clone(), unstable_root_node_guard.upcast());
  }

  // When there is a top stable node, we acquire a write lock on it.
  pub fn acquire_top_stable_node(&mut self, btree: &Arc<BTree>, identifier: &str) {
    let guard = NodeWriteGuard::acquire(btree, identifier);
    self.map.insert(String::from(identifier), guard.upcast());
    self.path.push(DeletionPathEntry::TopStableNode {
      node_identifier: String::from(identifier)
    });
  }

  pub fn current_node(&self) -> &NodeWriteGuard {
    let last_entry = self.path.last().expect("path can never be empty");
    let identifier = match last_entry {
      DeletionPathEntry::UnstableRootNode{ root_identifier } => root_identifier,
      DeletionPathEntry::TopStableNode{ node_identifier } => node_identifier,
      DeletionPathEntry::NodeWithMergeSibbling{ path_node_identifier, .. } => path_node_identifier,
    };

    self.map.get(identifier)
      .expect("locks on path must be present")
      .unwrap_node_write_guard_ref("locks on path are for nodes")
  }
}

pub fn acquire_write_set(btree: &Arc<BTree>, key_to_delete: &str) -> WriteSetAcquistionResult {
  let mut write_set = WriteSet::new();

  match acquire_parent_of_stable_node(btree, key_to_delete) {
    None => {
      // The root itself may be unstable for deletion.
      write_set.acquire_unstable_root_node(btree);
    }

    Some(parent_guard) => {
      match parent_guard {
        ReadGuard::RootIdentifierReadGuard(root_identifier_guard) => {
          write_set.acquire_top_stable_node(btree, root_identifier_guard.as_str_ref())
        }

        ReadGuard::NodeReadGuard(parent_node_guard) => {
          let child_identifier = parent_node_guard
            .unwrap_interior_node_ref("a parent must be an interior node")
            .child_identifier_by_key(key_to_delete);

          write_set.acquire_top_stable_node(btree, child_identifier)
        }
      };

      if !write_set.current_node().can_delete_without_merge() {
        return WriteSetAcquistionResult::TopNodeWentUnstable;
      }
    }
  }

  // TODO: we must now crawl our way down...
  loop {
    unimplemented!();
  }

  WriteSetAcquistionResult::Success(write_set)
}
