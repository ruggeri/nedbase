use locking::{
  LockTarget,
  LockTargetRef,
  NodeReadGuard,
  RootIdentifierReadGuard,
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

impl BTree {
  // Finds highest lock target that may need to be mutated by an
  // insertion.
  fn find_top_insert_lock_target(btree: &Arc<BTree>, key: &str) -> LockTarget {
    ::util::log_method_entry("find_top_insert_lock_target starting");
    let mut insert_lock_target = LockTarget::RootIdentifierTarget;
    let mut current_node_guard = {
      let identifier_guard = RootIdentifierReadGuard::acquire(btree);
      NodeReadGuard::acquire(btree, &(*identifier_guard))
    };

    loop {
      if current_node_guard.can_grow_without_split() {
        insert_lock_target = LockTarget::NodeTarget {
          identifier: String::from(current_node_guard.identifier())
        };
      }

      current_node_guard = match &(*current_node_guard) {
        Node::LeafNode(_) => break,
        Node::InteriorNode(inode) => {
          let child_identifier = inode.child_identifier_by_key(key);
          NodeReadGuard::acquire(btree, child_identifier)
        }
      }
    };

    ::util::log_method_entry("find_top_insert_lock_target completed");
    insert_lock_target
  }

  // Verifies that the acquired write lock is still the top-most target
  // that may be mutated by an insertion.
  fn verify_top_insert_lock(btree: &Arc<BTree>, top_insert_lock: &WriteGuard, key: &str) -> bool {
    ::util::log_method_entry("verify_top_insert_lock started");
    let top_insert_node = match top_insert_lock {
      WriteGuard::RootIdentifierWriteGuard { .. } => {
        ::util::log_method_entry("verify_top_insert_lock completed");
        return true;
      }
      WriteGuard::NodeWriteGuard(node_write_guard) => node_write_guard,
    };

    // The target node cannot grow without splitting, thus the parent
    // may need to be mutated.
    //
    // In theory it is possible that the top-most write lock required
    // now lives *below* this node, but that is unlikely.
    if !top_insert_node.can_grow_without_split() {
      ::util::log_method_entry("verify_top_insert_lock completed (cannot grow)");
      return false;
    }

    let top_insert_node_identifier = String::from(top_insert_node.identifier());

    let mut current_node_guard = {
      let root_identifier_guard = match RootIdentifierReadGuard::try_to_acquire(btree) {
        None => {
          // We can be blocked from descending if an ancestor lock is:
          //
          // (1) Currently held for reading,
          // (2) A write lock is queued up,
          // (3) The first lock cannot be cleared because it wants to
          //     descend reading to our top target node.
          //
          // Thus we will cancel our locking attempt if such a scenario
          // appears to occur.
          ::util::log_method_entry("verify_top_insert_lock completed (encountered lower lock)");
          return false
        }
        Some(root_identifier_guard) => root_identifier_guard
      };
      let root_identifier = root_identifier_guard.as_ref();

      // Notice how I never acquire a lock without first checking if
      // this is the identifier I'm looking for. Else I'd deadlock by
      // trying to acquire a read lock when I already have a write lock.
      if *root_identifier == top_insert_node_identifier {
        ::util::log_method_entry("verify_top_insert_lock completed (verification success)");
        return true;
      }

      match NodeReadGuard::try_to_acquire(btree, root_identifier) {
        None => {
          // See above for rationale.
          ::util::log_method_entry("verify_top_insert_lock completed (encountered lower lock)");
          return false
        }
        Some(node_guard) => node_guard
      }
    };

    loop {
      current_node_guard = match &(*current_node_guard) {
        Node::LeafNode(_) => return false,
        Node::InteriorNode(inode) => {
          let child_identifier = inode.child_identifier_by_key(key);
          if child_identifier == top_insert_node_identifier {
            ::util::log_method_entry("verify_top_insert_lock completed (verification success)");
            return true;
          }

          match NodeReadGuard::try_to_acquire(btree, child_identifier) {
            None => {
              // See above for rationale.
              ::util::log_method_entry("verify_top_insert_lock completed (failed; wrong node found)");
              return false
            }
            Some(node_guard) => node_guard
          }
        }
      }
    }
  }

  fn acquire_write_lock_path_for_insert(btree: &Arc<BTree>, key: &str) -> WriteLockAcquisitionResult {
    ::util::log_method_entry("acquire_write_lock_path_for_insert started");
    let top_insert_lock_target = BTree::find_top_insert_lock_target(btree, key);
    let top_insert_guard = WriteGuard::acquire(btree, top_insert_lock_target.as_ref());

    if !BTree::verify_top_insert_lock(btree, &top_insert_guard, key) {
      ::util::log_method_entry("acquire_write_lock_path_for_insert (verification failed)");
      return WriteLockAcquisitionResult::TopWriteLockVerificationFailed;
    }

    let mut write_guards = vec![top_insert_guard];
    if let Some(root_node_guard) = match &write_guards[0] {
      WriteGuard::RootIdentifierWriteGuard(identifier_guard) => {
        let root_node_guard = WriteGuard::acquire(btree, LockTargetRef::NodeTarget { identifier: &(*identifier_guard) });
        Some(root_node_guard)
      }
      WriteGuard::NodeWriteGuard(..) => None
    } {
      write_guards.push(root_node_guard);
    }

    loop {
      let next_write_guard = {
        let last_guard = write_guards.last().expect("write_guards should be non-empty");
        let current_node_guard = last_guard
          .unwrap_node_write_guard_ref("last write_guard should be a node guard");

        let child_identifier = match &(**current_node_guard) {
          Node::LeafNode(_) => break,
          Node::InteriorNode(inode) => inode.child_identifier_by_key(key),
        };

        // TODO: May be able to release prior locks if we hit a stable
        // lock.
        WriteGuard::acquire(btree, LockTargetRef::NodeTarget { identifier: child_identifier })
      };

      write_guards.push(next_write_guard);
    }

    ::util::log_method_entry("acquire_write_lock_path_for_insert (success)");
    WriteLockAcquisitionResult::Succeeded(write_guards)
  }

  pub fn insert(btree: &Arc<BTree>, key: String) {
    ::util::log_method_entry("insert started");
    let mut write_guards = loop {
      match BTree::acquire_write_lock_path_for_insert(btree, &key) {
        WriteLockAcquisitionResult::TopWriteLockVerificationFailed => continue,
        WriteLockAcquisitionResult::Succeeded(write_guards) => break write_guards,
      }
    };

    ::util::log_method_entry("beginning insertion process");
    let mut insertion_result = {
      let last_write_guard = write_guards.pop().expect("should acquire at least one write guard");
      let mut current_node_guard = last_write_guard
        .unwrap_node_write_guard("last write_guard should be a node guard");
      current_node_guard
        .unwrap_leaf_node_mut_ref("Expected leaf node to insert into at bottom")
        .insert(btree, key)
    };

    while let InsertionResult::DidInsertWithSplit(child_split_info) = insertion_result {
      let mut last_write_guard = write_guards.pop().expect("should not run out of write guards");

      match last_write_guard {
        WriteGuard::RootIdentifierWriteGuard(mut identifier_guard) => {
          ::util::log_method_entry("trying to split root");

          let new_root_identifier = btree.store_new_interior_node(
            vec![child_split_info.new_median],
            vec![child_split_info.new_left_identifier, child_split_info.new_right_identifier],
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
