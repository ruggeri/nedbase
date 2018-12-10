use super::acquire_parent_of_stable_node::acquire_parent_of_stable_node;
use btree::BTree;
use locking::{NodeWriteGuard, ReadGuard, RootIdentifierWriteGuard, WriteGuard};
use std::collections::HashMap;
use std::sync::Arc;

type SibblingIdentifierPair = (Option<String>, Option<String>);

pub enum WriteSetAcquisitionResult {
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
    sibbling_node_identifier: String,
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

    self.path.push(DeletionPathEntry::UnstableRootNode {
      root_identifier: root_identifier
    });
  }

  // When there is a top stable node, we acquire a write lock on it.
  pub fn acquire_top_stable_node(&mut self, btree: &Arc<BTree>, identifier: &str) {
    let guard = NodeWriteGuard::acquire(btree, identifier);
    self.map.insert(String::from(identifier), guard.upcast());
    self.path.push(DeletionPathEntry::TopStableNode {
      node_identifier: String::from(identifier)
    });
  }

  // When there is a top stable node, we acquire a write lock on it.
  pub fn acquire_node_and_sibbling(&mut self, btree: &Arc<BTree>, path_node_identifier: String, sibbling_node_identifiers: SibblingIdentifierPair) {
    let path_node_guard = NodeWriteGuard::acquire(btree, &path_node_identifier);

    let (sibbling_node_identifier, sibbling_node_guard) = match sibbling_node_identifiers {
      (None, None) => panic!("how can a node not have any sibblings?"),

      (Some(sibbling_node_identifier), None) | (None, Some(sibbling_node_identifier)) => {
        let sibbling_node_guard = NodeWriteGuard::acquire(btree, &sibbling_node_identifier);
        (sibbling_node_identifier, sibbling_node_guard)
      }

      (Some(left_sibbling_node_identifier), Some(right_sibbling_node_identifier)) => {
        let left_sibbling_guard = NodeWriteGuard::acquire(btree, &left_sibbling_node_identifier);

        if left_sibbling_guard.can_delete_without_merge() {
          (left_sibbling_node_identifier, left_sibbling_guard)
        } else {
          // I doubt randomization would help. You will take values from
          // left until you can't anymore, and THEN you'll take from
          // right. So it doesn't matter. You won't merge until you
          // must.
          let right_sibbling_guard = NodeWriteGuard::acquire(btree, &right_sibbling_node_identifier);
          (right_sibbling_node_identifier, right_sibbling_guard)
        }
      }
    };

    self.map.insert(path_node_identifier.clone(), path_node_guard.upcast());
    self.map.insert(sibbling_node_identifier.clone(), sibbling_node_guard.upcast());

    self.path.push(DeletionPathEntry::NodeWithMergeSibbling {
      path_node_identifier,
      sibbling_node_identifier,
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

  pub fn current_node_mut(&mut self) -> &mut NodeWriteGuard {
    let last_entry = self.path.last().expect("path can never be empty");
    let identifier = match last_entry {
      DeletionPathEntry::UnstableRootNode{ root_identifier } => root_identifier,
      DeletionPathEntry::TopStableNode{ node_identifier } => node_identifier,
      DeletionPathEntry::NodeWithMergeSibbling{ path_node_identifier, .. } => path_node_identifier,
    };

    self.map.get_mut(identifier)
      .expect("locks on path must be present")
      .unwrap_node_write_guard_mut_ref("locks on path are for nodes")
  }
}

pub fn acquire_write_set(
  btree: &Arc<BTree>,
  key_to_delete: &str,
) -> WriteSet {
  loop {
    // Note that this will release the read lock on the parent (if any).
    let write_guard_acquisition_result = maybe_acquire_write_set(
      btree,
      key_to_delete,
    );

    match write_guard_acquisition_result {
      WriteSetAcquisitionResult::TopNodeWentUnstable => {
        // The deepest stable node may go unstable due to simultaneous
        // delete, which means we must try everything again.
        continue;
      }

      WriteSetAcquisitionResult::Success(write_set) => {
        // Hopefully the deepest stable node stayed stable! Then we can
        // continue.
        return write_set;
      }
    }
  }
}

fn maybe_acquire_write_set(btree: &Arc<BTree>, key_to_delete: &str) -> WriteSetAcquisitionResult {
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
        return WriteSetAcquisitionResult::TopNodeWentUnstable;
      }
    }
  }

  // TODO: we must now crawl our way down...
  loop {
    if write_set.current_node().is_leaf_node() {
      break;
    }

    let (path_node_identifier, sibbling_node_identifiers) = {
      let current_node = write_set
        .current_node()
        .unwrap_interior_node_ref("must not descend through leaves");

      let child_idx = current_node.child_idx_by_key(key_to_delete);
      let path_node_identifier = current_node.child_identifier_by_idx(child_idx);
      let sibbling_node_identifiers = current_node.sibbling_identifiers_for_idx(child_idx);

      (
        String::from(path_node_identifier),
        (
          sibbling_node_identifiers.0.map(String::from),
          sibbling_node_identifiers.1.map(String::from),
        )
      )
    };

    write_set.acquire_node_and_sibbling(btree, path_node_identifier, sibbling_node_identifiers);
  }

  WriteSetAcquisitionResult::Success(write_set)
}
