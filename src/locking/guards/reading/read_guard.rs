use super::{NodeReadGuard, RootIdentifierReadGuard};
use btree::BTree;
use locking::LockTarget;
use std::sync::Arc;

pub enum ReadGuard {
  RootIdentifierReadGuard(RootIdentifierReadGuard),
  NodeReadGuard(NodeReadGuard),
}

impl ReadGuard {
  pub(in locking) fn acquire_read_guard(
    btree: &Arc<BTree>,
    lock_target: &LockTarget,
  ) -> ReadGuard {
    match lock_target {
      LockTarget::Node(identifier) => {
        Self::acquire_node_read_guard(btree, identifier)
      }
      LockTarget::RootIdentifier => {
        Self::acquire_root_identifier_read_guard(btree)
      }
    }
  }

  pub(in locking) fn acquire_node_read_guard(
    btree: &Arc<BTree>,
    identifier: &str,
  ) -> ReadGuard {
    ReadGuard::NodeReadGuard(NodeReadGuard::acquire(btree, identifier))
  }

  pub(in locking) fn acquire_root_identifier_read_guard(
    btree: &Arc<BTree>,
  ) -> ReadGuard {
    ReadGuard::RootIdentifierReadGuard(
      RootIdentifierReadGuard::acquire(btree),
    )
  }

  pub fn target(&self) -> LockTarget {
    match self {
      ReadGuard::RootIdentifierReadGuard(..) => LockTarget::RootIdentifier,
      ReadGuard::NodeReadGuard(guard) => LockTarget::Node(String::from(guard.identifier()))
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
