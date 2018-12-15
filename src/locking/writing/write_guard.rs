use btree::BTree;
use locking::{LockTarget, NodeWriteGuard, RootIdentifierWriteGuard};
use std::sync::Arc;

pub enum WriteGuard {
  RootIdentifierWriteGuard(RootIdentifierWriteGuard),
  NodeWriteGuard(NodeWriteGuard),
}

impl WriteGuard {
  pub(in locking) fn acquire_write_guard(btree: &Arc<BTree>, lock_target: &LockTarget) -> WriteGuard {
    match lock_target {
      LockTarget::Node(identifier) => Self::acquire_node_write_guard(btree, identifier),
      LockTarget::RootIdentifier => Self::acquire_root_identifier_write_guard(btree),
    }
  }

  pub(in locking) fn acquire_node_write_guard(
    btree: &Arc<BTree>,
    identifier: &str,
  ) -> WriteGuard {
    WriteGuard::NodeWriteGuard(NodeWriteGuard::acquire(
      btree, identifier,
    ))
  }

  pub(in locking) fn acquire_root_identifier_write_guard(
    btree: &Arc<BTree>,
  ) -> WriteGuard {
    WriteGuard::RootIdentifierWriteGuard(
      RootIdentifierWriteGuard::acquire(btree),
    )
  }

  pub fn unwrap_node_write_guard(
    self,
    message: &'static str,
  ) -> NodeWriteGuard {
    match self {
      WriteGuard::RootIdentifierWriteGuard(..) => panic!(message),
      WriteGuard::NodeWriteGuard(node_write_guard) => node_write_guard,
    }
  }

  pub fn unwrap_node_write_guard_mut_ref(
    &mut self,
    message: &'static str,
  ) -> &mut NodeWriteGuard {
    match self {
      WriteGuard::RootIdentifierWriteGuard(..) => panic!(message),
      WriteGuard::NodeWriteGuard(node_write_guard) => node_write_guard,
    }
  }

  pub fn unwrap_node_write_guard_ref(
    &self,
    message: &'static str,
  ) -> &NodeWriteGuard {
    match self {
      WriteGuard::RootIdentifierWriteGuard(..) => panic!(message),
      WriteGuard::NodeWriteGuard(node_write_guard) => node_write_guard,
    }
  }

  pub fn unwrap_root_identifier_write_guard(
    self,
    message: &'static str,
  ) -> RootIdentifierWriteGuard {
    match self {
      WriteGuard::RootIdentifierWriteGuard(root_identifier_guard) => {
        root_identifier_guard
      }
      WriteGuard::NodeWriteGuard(..) => panic!(message),
    }
  }

  pub fn unwrap_root_identifier_write_guard_mut_ref(
    &mut self,
    message: &'static str,
  ) -> &mut RootIdentifierWriteGuard {
    match self {
      WriteGuard::RootIdentifierWriteGuard(root_identifier_guard) => {
        root_identifier_guard
      }
      WriteGuard::NodeWriteGuard(..) => panic!(message),
    }
  }

  pub fn unwrap_root_identifier_write_guard_ref(
    &self,
    message: &'static str,
  ) -> &RootIdentifierWriteGuard {
    match self {
      WriteGuard::RootIdentifierWriteGuard(root_identifier_guard) => {
        root_identifier_guard
      }
      WriteGuard::NodeWriteGuard(..) => panic!(message),
    }
  }
}
