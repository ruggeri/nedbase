use locking::{
  LockTarget,
  LockTargetRef,
  NodeReadGuard,
  ReadGuard,
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

type LockVerificationPath = Vec<LockTarget>;

impl BTree {
  // Finds highest lock target that may need to be mutated by an
  // insertion.
  fn find_top_insert_lock_target(btree: &Arc<BTree>, key: &str) -> LockVerificationPath {
    ::util::log_method_entry("find_top_insert_lock_target starting");

    let mut current_path = vec![LockTarget::RootIdentifierTarget];
    let mut target_lock_path = current_path.clone();

    let mut current_node_guard = {
      let identifier_guard = RootIdentifierReadGuard::acquire(btree);
      NodeReadGuard::acquire(btree, &(*identifier_guard))
    };

    loop {
      let current_node_target = LockTarget::NodeTarget {
        identifier: String::from(current_node_guard.identifier())
      };

      current_path.push(current_node_target);

      if current_node_guard.can_grow_without_split() {
        target_lock_path = current_path.clone();
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
    target_lock_path
  }

  // Verifies that the acquired write lock is still the top-most target
  // that may be mutated by an insertion.
  fn verify_top_insert_lock(btree: &Arc<BTree>, path_to_target_lock: &LockVerificationPath, top_insert_guard: &WriteGuard, key: &str) -> bool {
    ::util::log_method_entry("verify_top_insert_lock started");
    let top_insert_node = match top_insert_guard {
      WriteGuard::RootIdentifierWriteGuard { .. } => {
        ::util::log_method_entry("verify_top_insert_lock completed");
        // Splitting all the way to the root is always a valid
        // possibility.
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

    let mut current_lock_guard = match ReadGuard::try_to_acquire(btree, LockTargetRef::RootIdentifierTarget) {
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

    for next_element_of_path in path_to_target_lock {
      let next_lock_guard = {
        let current_location = current_lock_guard.location();
        match (current_location, next_element_of_path) {
          (LockTargetRef::RootIdentifierTarget, LockTarget::NodeTarget { .. }) => {
            ::util::log_method_entry("verify_top_insert_lock completed (failed; went off path)");
            return false;
          },
          (LockTargetRef::NodeTarget { .. }, LockTarget::RootIdentifierTarget) => {
            ::util::log_method_entry("verify_top_insert_lock completed (failed; went off path)");
            return false;
          }
          (LockTargetRef::NodeTarget { identifier: current_identifier }, LockTarget::NodeTarget { identifier: expected_identifier }) => {
            if current_identifier != expected_identifier {
              ::util::log_method_entry("verify_top_insert_lock completed (failed; went off path)");
              return false;
            }
          }
          _ => {
            // Everything else means we are matching!
          }
        }

        let child_location = match &current_lock_guard {
          ReadGuard::RootIdentifierReadGuard(current_root_guard) => {
            LockTargetRef::NodeTarget { identifier: &(*current_root_guard) }
          }
          ReadGuard::NodeReadGuard(current_node_guard) => {
            match &(**current_node_guard) {
              Node::LeafNode(..) => {
                panic!("Did not expect to hit leaf node along path");
              }
              Node::InteriorNode(inode) => {
                LockTargetRef::NodeTarget {
                  identifier: inode.child_identifier_by_key(key)
                }
              }
            }
          }
        };

        // Both locations on the path match. Good. Where should we descend next?
        // Do we keep descending, or are we done?
        let top_insert_guard_location = top_insert_guard.location();
        if child_location == top_insert_guard_location {
          // We got all the way to the target!
          ::util::log_method_entry("verify_top_insert_lock completed (verification success)");
          return true;
        }

        // Else we must keep descending.
        let next_lock_guard = match ReadGuard::try_to_acquire(btree, child_location) {
          None => {
            // See above rationale.
            ::util::log_method_entry("verify_top_insert_lock completed (encountered lower lock)");
            return false
          }
          Some(child_guard) => child_guard
        };

        next_lock_guard
      };

      current_lock_guard = next_lock_guard;
    }

    panic!("How did we get here??");
  }

  fn acquire_write_lock_path_for_insert(btree: &Arc<BTree>, key: &str) -> WriteLockAcquisitionResult {
    ::util::log_method_entry("acquire_write_lock_path_for_insert started");
    let target_lock_path = BTree::find_top_insert_lock_target(btree, key);
    let top_insert_target = target_lock_path.last().unwrap();
    let top_insert_guard = WriteGuard::acquire(btree, top_insert_target.as_ref());

    if !BTree::verify_top_insert_lock(btree, &target_lock_path, &top_insert_guard, key) {
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
