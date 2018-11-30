use btree::BTree;
use locking::{LockTargetRef, NodeWriteGuard, RootIdentifierWriteGuard};
use std::sync::Arc;

pub enum WriteGuard {
  RootIdentifierWriteGuard(RootIdentifierWriteGuard),
  NodeWriteGuard(NodeWriteGuard),
}

impl WriteGuard {
  pub fn acquire_node_write_guard(btree: &Arc<BTree>, identifier: &str) -> WriteGuard {
    WriteGuard::NodeWriteGuard(NodeWriteGuard::acquire(btree, identifier))
  }

  pub fn acquire_root_identifier_write_guard(btree: &Arc<BTree>) -> WriteGuard {
    WriteGuard::RootIdentifierWriteGuard(RootIdentifierWriteGuard::acquire(btree))
  }

  pub fn acquire(btree: &Arc<BTree>, target: LockTargetRef) -> WriteGuard {
    match target {
      LockTargetRef::RootIdentifierTarget => {
        Self::acquire_root_identifier_write_guard(btree)
      },
      LockTargetRef::NodeTarget(identifier) => {
        Self::acquire_node_write_guard(btree, identifier)
      }
    }
  }

  // pub fn acquire_if_no_current_writers(btree: &'a BTree, target: LockTargetRef) -> Option<WriteGuard<'a>> {
  //   match target {
  //     LockTargetRef::RootIdentifierTarget => {
  //       RootIdentifierWriteGuard::acquire_if_no_current_writers(btree)
  //         .map(WriteGuard::RootIdentifierWriteGuard)
  //     },
  //     LockTargetRef::NodeTarget { identifier } => {
  //       NodeWriteGuard::acquire_if_no_current_writers(btree, identifier)
  //         .map(WriteGuard::NodeWriteGuard)
  //     }
  //   }
  // }

  pub fn unwrap_node_write_guard_ref(&self, message: &'static str) -> &NodeWriteGuard {
    match self {
      WriteGuard::RootIdentifierWriteGuard(..) => panic!(message),
      WriteGuard::NodeWriteGuard(node_write_guard) => node_write_guard
    }
  }

  pub fn unwrap_root_identifier_write_guard_ref(&self, message: &'static str) -> &RootIdentifierWriteGuard {
    match self {
      WriteGuard::RootIdentifierWriteGuard(root_identifier_guard) => root_identifier_guard,
      WriteGuard::NodeWriteGuard(node_write_guard) => panic!(message),
    }
  }

  pub fn unwrap_node_write_guard(self, message: &'static str) -> NodeWriteGuard {
    match self {
      WriteGuard::RootIdentifierWriteGuard(..) => panic!(message),
      WriteGuard::NodeWriteGuard(node_write_guard) => node_write_guard
    }
  }

  pub fn unwrap_root_identifier_write_guard(self, message: &'static str) -> RootIdentifierWriteGuard {
    match self {
      WriteGuard::RootIdentifierWriteGuard(root_identifier_guard) => root_identifier_guard,
      WriteGuard::NodeWriteGuard(node_write_guard) => panic!(message),
    }
  }

  pub fn location(&self) -> LockTargetRef {
    match self {
      WriteGuard::RootIdentifierWriteGuard(guard) => guard.location(),
      WriteGuard::NodeWriteGuard(guard) => guard.location(),
    }
  }
}
