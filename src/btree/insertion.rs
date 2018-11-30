use locking::{
  LockTarget,
  NodeReadGuard,
  NodeWriteGuard,
  ReadGuard,
  RootIdentifierReadGuard,
  RootIdentifierWriteGuard,
  WriteGuard,
};
use node::{
  InsertionResult,
  Node,
};
use std::sync::Arc;
use super::common::BTree;

enum WriteLockAcquisitionResult {
  TopWriteLockVerificationFailed,
  Succeeded(Vec<WriteGuard>),
}

type LockVerificationPath = Vec<LockTarget>;
type ReadGuardPath = Vec<ReadGuard>;
type WriteGuardPath = Vec<WriteGuard>;

struct InsertionGuards {
  read_guards: ReadGuardPath,
  write_guards: WriteGuardPath,
}

impl BTree {
  // Finds highest lock target that may need to be mutated by an
  // insertion.
  fn build_read_path_for_insert(btree: &Arc<BTree>, key: &str) -> ReadGuardPath {
    ::util::log_method_entry("build_read_path_for_insert starting");

    let mut current_path = ReadGuardPath::new();
    {
      let root_identifier_guard = RootIdentifierReadGuard::acquire(btree);
      let root_node_guard = NodeReadGuard::acquire(btree, &root_identifier_guard);

      current_path.push(ReadGuard::RootIdentifierReadGuard(root_identifier_guard));
      current_path.push(ReadGuard::NodeReadGuard(root_node_guard));
    }

    loop {
      let child_guard = {
        let node_guard = current_path.last().unwrap().unwrap_node_read_guard_ref("expected node");
        match &(**node_guard) {
          Node::LeafNode(_) => break,
          Node::InteriorNode(inode) => {
            let child_identifier = inode.child_identifier_by_key(key);
            ReadGuard::acquire_node_read_guard(btree, child_identifier)
          }
        }
      };

      current_path.push(child_guard);
    };

    while current_path.len() > 1 {
      {
        let read_guard = current_path.last().unwrap();
        let read_guard = read_guard.unwrap_node_read_guard_ref("expected node");

        if read_guard.can_grow_without_split() {
          break;
        }
      }

      current_path.pop();
    }

    current_path
  }

  fn finish_building_insertion_guards(btree: &Arc<BTree>, mut read_guards: ReadGuardPath, key: &str) -> Option<InsertionGuards> {
    let mut write_guards = Vec::new();

    let top_write_lock_location = {
      let top_read_guard = read_guards.pop().unwrap();
      top_read_guard.location().promote_to_val()
    };

    match top_write_lock_location {
      LockTarget::RootIdentifierTarget => {
        let root_identifier_guard = RootIdentifierWriteGuard::acquire(btree);
        let root_guard = WriteGuard::acquire_node_write_guard(btree, &root_identifier_guard);

        write_guards.push(WriteGuard::RootIdentifierWriteGuard(root_identifier_guard));
        write_guards.push(root_guard);
      },
      LockTarget::NodeTarget(identifier) => {
        let node_guard = NodeWriteGuard::acquire(btree, &identifier);
        if !node_guard.can_grow_without_split() {
          // We failed; this is no longer stable.
          return None;
        }

        write_guards.push(WriteGuard::NodeWriteGuard(node_guard));
      }
    }

    // Descend acquiring write guards.
    loop {
      let child_guard = {
        let node_guard = write_guards.last().unwrap().unwrap_node_write_guard_ref("expected node");
        match &(**node_guard) {
          Node::LeafNode(_) => break,
          Node::InteriorNode(inode) => {
            let child_identifier = inode.child_identifier_by_key(key);
            WriteGuard::acquire_node_write_guard(btree, child_identifier)
          }
        }
      };

      write_guards.push(child_guard);
    };

    Some(InsertionGuards {
      read_guards,
      write_guards
    })
  }

  pub fn insert(btree: &Arc<BTree>, key: String) {
    let mut insertion_guards = loop {
      let read_guards = BTree::build_read_path_for_insert(btree, &key);
      match BTree::finish_building_insertion_guards(btree, read_guards, &key) {
        None => continue,
        Some(insertion_guards) => break insertion_guards
      }
    };

    ::util::log_method_entry("beginning insertion process");
    let mut insertion_result = {
      let last_write_guard = insertion_guards.write_guards.pop().expect("should acquire at least one write guard");
      let mut current_node_guard = last_write_guard
        .unwrap_node_write_guard("last write_guard should be a node guard");
      current_node_guard
        .unwrap_leaf_node_mut_ref("Expected leaf node to insert into at bottom")
        .insert(btree, key)
    };

    while let InsertionResult::DidInsertWithSplit(child_split_info) = insertion_result {
      let mut last_write_guard = insertion_guards.write_guards.pop().expect("should not run out of write guards");

      match last_write_guard {
        WriteGuard::RootIdentifierWriteGuard(mut identifier_guard) => {
          ::util::log_method_entry("trying to split root");

          let new_root_identifier = btree.store_new_root_node(
            child_split_info
          );

          *identifier_guard = new_root_identifier;

          break
        },

        WriteGuard::NodeWriteGuard(mut node_guard) => {
          insertion_result = node_guard
            .unwrap_interior_node_mut_ref("expected interior node")
            .handle_split(btree, child_split_info);
        }
      };
    }

    ::util::log_method_entry("insert completed");
  }

  // pub fn insert2(&self, key: String) {
  //   let mut guards = Vec::<WriteGuard>::new();

  //   {
  //     let identifier_guard = RootIdentifierWriteGuard::acquire(self);
  //     let current_node_guard = NodeWriteGuard::acquire(self, &(*identifier_guard.identifier));
  //     guards.push(WriteGuard::RootIdentifierWriteGuard(identifier_guard));
  //     guards.push(WriteGuard::NodeWriteGuard(current_node_guard));
  //   }

  //   loop {
  //     let current_node_guard = {
  //       let node_write_guard = guards.last().unwrap().unwrap_node_write_guard_ref("");

  //       match &(*node_write_guard.node) {
  //         Node::LeafNode(_) => break,
  //         Node::InteriorNode(inode) => {
  //           let child_identifier = inode.child_identifier_by_key(&key);
  //           NodeWriteGuard::acquire(self, child_identifier)
  //         }
  //       }
  //     };

  //     if current_node_guard.node.can_grow_without_split() {
  //       guards.clear();
  //     }

  //     guards.push(WriteGuard::NodeWriteGuard(current_node_guard));
  //   };

  //   let mut insertion_result = {
  //     let last_write_guard = guards.pop().expect("should acquire at least one write guard");
  //     let mut current_node_guard = last_write_guard
  //       .unwrap_node_write_guard("last write_guard should be a node guard");
  //     current_node_guard
  //       .node
  //       .unwrap_leaf_node_mut_ref("Expected leaf node to insert into at bottom")
  //       .insert(self, key)
  //   };

  //   while let InsertionResult::DidInsertWithSplit(child_split_info) = insertion_result {
  //     let mut last_write_guard = guards.pop().expect("should not run out of write guards");

  //     match last_write_guard {
  //       WriteGuard::RootIdentifierWriteGuard(RootIdentifierWriteGuard { ref mut identifier }) => {
  //         let new_root_identifier = self.store_new_interior_node(
  //           vec![child_split_info.new_median],
  //           vec![child_split_info.new_left_identifier, child_split_info.new_right_identifier],
  //         );

  //         **identifier = new_root_identifier;

  //         return
  //       },

  //       WriteGuard::NodeWriteGuard(NodeWriteGuard { ref mut node, .. }) => {
  //         insertion_result = node
  //           .unwrap_interior_node_mut_ref("expected interior node")
  //           .handle_split(self, child_split_info);
  //       }
  //     };
  //   }
  // }
}
