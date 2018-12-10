use btree::BTree;
use locking::{LockTargetRef, NodeReadGuard, RootIdentifierReadGuard};
use std::sync::Arc;

pub enum ReadGuard {
  RootIdentifierReadGuard(RootIdentifierReadGuard),
  NodeReadGuard(NodeReadGuard),
}

impl ReadGuard {
  pub fn acquire(
    btree: &Arc<BTree>,
    target: LockTargetRef,
  ) -> ReadGuard {
    match target {
      LockTargetRef::RootIdentifierTarget => {
        ReadGuard::acquire_root_identifier_read_guard(btree)
      }

      LockTargetRef::NodeTarget(identifier) => {
        ReadGuard::acquire_node_read_guard(btree, identifier)
      }
    }
  }

  pub fn acquire_node_read_guard(
    btree: &Arc<BTree>,
    identifier: &str,
  ) -> ReadGuard {
    ReadGuard::NodeReadGuard(NodeReadGuard::acquire(btree, identifier))
  }

  pub fn acquire_root_identifier_read_guard(
    btree: &Arc<BTree>,
  ) -> ReadGuard {
    ReadGuard::RootIdentifierReadGuard(
      RootIdentifierReadGuard::acquire(btree),
    )
  }

  pub fn location(&self) -> LockTargetRef {
    match self {
      ReadGuard::RootIdentifierReadGuard(guard) => guard.location(),
      ReadGuard::NodeReadGuard(guard) => guard.location(),
    }
  }

  pub fn unwrap_node_read_guard(
    self,
    message: &'static str,
  ) -> NodeReadGuard {
    match self {
      ReadGuard::RootIdentifierReadGuard(..) => panic!(message),
      ReadGuard::NodeReadGuard(node_guard) => node_guard,
    }
  }

  pub fn unwrap_node_read_guard_ref(
    &self,
    message: &'static str,
  ) -> &NodeReadGuard {
    match self {
      ReadGuard::RootIdentifierReadGuard(..) => panic!(message),
      ReadGuard::NodeReadGuard(node_guard) => node_guard,
    }
  }

  pub fn unwrap_root_identifier_read_guard_ref(
    &self,
    message: &'static str,
  ) -> &RootIdentifierReadGuard {
    match self {
      ReadGuard::NodeReadGuard(..) => panic!(message),
      ReadGuard::RootIdentifierReadGuard(root_guard) => root_guard,
    }
  }
}
