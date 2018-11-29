use locking::{
  LockTarget,
  LockTargetRef,
  NodeReadGuard,
  NodeWriteGuard,
  RootIdentifierReadGuard,
  RootIdentifierWriteGuard,
  WriteGuard,
};
use node::{
  InsertionResult,
  Node,
};
use super::common::BTree;

enum WriteLockAcquisitionResult<'a> {
  TopWriteLockVerificationFailed,
  Succeeded(Vec<WriteGuard<'a>>),
}

impl BTree {
  // Finds highest lock target that may need to be mutated by an
  // insertion.
  fn find_top_insert_lock_target(&self, key: &str) -> LockTarget {
    let mut insert_lock_target = LockTarget::RootIdentifierTarget;
    let mut current_node_guard = {
      let identifier_guard = RootIdentifierReadGuard::acquire(self);
      NodeReadGuard::acquire(self, &(*identifier_guard.identifier))
    };

    loop {
      if current_node_guard.node.can_grow_without_split() {
        insert_lock_target = LockTarget::NodeTarget {
          identifier: String::from(current_node_guard.node.identifier())
        };
      }

      current_node_guard = match &(*current_node_guard.node) {
        Node::LeafNode(_) => break,
        Node::InteriorNode(inode) => {
          let child_identifier = inode.child_identifier_by_key(key);
          NodeReadGuard::acquire(self, child_identifier)
        }
      }
    };

    insert_lock_target
  }

  // Verifies that the acquired write lock is still the top-most target
  // that may be mutated by an insertion.
  fn verify_top_insert_lock(&self, top_insert_lock: &WriteGuard, key: &str) -> bool {
    let top_insert_node = match top_insert_lock {
      WriteGuard::RootIdentifierWriteGuard { .. } => return true,
      WriteGuard::NodeWriteGuard(NodeWriteGuard { node, .. }) => node,
    };

    // The target node cannot grow without splitting, thus the parent
    // may need to be mutated.
    //
    // In theory it is possible that the top-most write lock required
    // now lives *below* this node, but that is unlikely.
    if !top_insert_node.can_grow_without_split() {
      return false;
    }

    let top_insert_node_identifier = String::from(top_insert_node.identifier());

    let mut current_node_guard = {
      let root_identifier_guard = RootIdentifierReadGuard::acquire(self);
      let root_identifier = root_identifier_guard.identifier.as_ref();

      // Notice how I never acquire a lock without first checking if
      // this is the identifier I'm looking for. Else I'd deadlock by
      // trying to acquire a read lock when I already have a write lock.
      if root_identifier == top_insert_node_identifier {
        return true;
      }

      NodeReadGuard::acquire(self, root_identifier)
    };

    loop {
      current_node_guard = match &(*current_node_guard.node) {
        Node::LeafNode(_) => return false,
        Node::InteriorNode(inode) => {
          let child_identifier = inode.child_identifier_by_key(key);
          if child_identifier == top_insert_node_identifier {
            return true;
          }

          NodeReadGuard::acquire(self, child_identifier)
        }
      }
    }
  }

  fn acquire_write_lock_path_for_insert(&self, key: &str) -> WriteLockAcquisitionResult {
    let top_insert_lock_target = self.find_top_insert_lock_target(key);
    let top_insert_guard = WriteGuard::acquire(self, top_insert_lock_target.as_ref());

    if !self.verify_top_insert_lock(&top_insert_guard, key) {
      return WriteLockAcquisitionResult::TopWriteLockVerificationFailed;
    }

    let mut write_guards = vec![top_insert_guard];
    if let Some(root_node_guard) = match &write_guards[0] {
      WriteGuard::RootIdentifierWriteGuard(RootIdentifierWriteGuard { identifier }) => {
        let root_node_guard = WriteGuard::acquire(self, LockTargetRef::NodeTarget { identifier: &(*identifier) });
        Some(root_node_guard)
      }
      WriteGuard::NodeWriteGuard(..) => None
    } {
      write_guards.push(root_node_guard);
    }

    loop {
      let next_write_guard = {
        let last_guard = write_guards.last().expect("write_guards should be non-empty");
        let current_node_guard = &last_guard
          .unwrap_node_write_guard_ref("last write_guard should be a node guard")
          .node;

        let child_identifier = match &(**current_node_guard) {
          Node::LeafNode(_) => break,
          Node::InteriorNode(inode) => inode.child_identifier_by_key(key),
        };

        // TODO: May be able to release prior locks if we hit a stable
        // lock.
        WriteGuard::acquire(self, LockTargetRef::NodeTarget { identifier: child_identifier })
      };

      write_guards.push(next_write_guard);
    }

    WriteLockAcquisitionResult::Succeeded(write_guards)
  }

  pub fn insert(&self, key: String) {
    let mut write_guards = loop {
      match self.acquire_write_lock_path_for_insert(&key) {
        WriteLockAcquisitionResult::TopWriteLockVerificationFailed => continue,
        WriteLockAcquisitionResult::Succeeded(write_guards) => break write_guards,
      }
    };

    let mut insertion_result = {
      let last_write_guard = write_guards.pop().expect("should acquire at least one write guard");
      let mut current_node_guard = last_write_guard
        .unwrap_node_write_guard("last write_guard should be a node guard");
      current_node_guard
        .node
        .unwrap_leaf_node_mut_ref("Expected leaf node to insert into at bottom")
        .insert(self, key)
    };

    while let InsertionResult::DidInsertWithSplit(child_split_info) = insertion_result {
      let mut last_write_guard = write_guards.pop().expect("should not run out of write guards");

      match last_write_guard {
        WriteGuard::RootIdentifierWriteGuard(RootIdentifierWriteGuard { ref mut identifier }) => {
          let new_root_identifier = self.store_new_interior_node(
            vec![child_split_info.new_median],
            vec![child_split_info.new_left_identifier, child_split_info.new_right_identifier],
          );

          **identifier = new_root_identifier;

          return
        },

        WriteGuard::NodeWriteGuard(NodeWriteGuard { ref mut node, .. }) => {
          insertion_result = node
            .unwrap_interior_node_mut_ref("expected interior node")
            .handle_split(self, child_split_info);
        }
      };
    }
  }
}
