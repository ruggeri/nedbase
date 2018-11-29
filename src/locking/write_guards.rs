use btree::BTree;
use node::Node;
use std::mem;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use super::lock_target::LockTargetRef;

pub struct NodeWriteGuard<'a> {
  // This exists to ensure that the underlying lock lives as long as
  // needed.
  pub lock: Arc<RwLock<Node>>,
  pub node: RwLockWriteGuard<'a, Node>,
}

impl<'a> NodeWriteGuard<'a> {
  pub fn acquire(btree: &BTree, identifier: &str) -> NodeWriteGuard<'a> {
    let lock = btree.get_node_arc_lock(&identifier);
    NodeWriteGuard::acquire_from_lock(lock)
  }

  pub fn acquire_from_lock(lock: Arc<RwLock<Node>>) -> NodeWriteGuard<'a> {
    let node = lock.write().expect("Other threads shouldn't panic with lock");
    let guard = NodeWriteGuard { lock: Arc::clone(&lock), node };

    unsafe {
      mem::transmute(guard)
    }
  }
}

pub struct RootIdentifierWriteGuard<'a> {
  pub identifier: RwLockWriteGuard<'a, String>,
}

impl<'a> RootIdentifierWriteGuard<'a> {
  pub fn acquire(btree: &'a BTree) -> RootIdentifierWriteGuard<'a> {
    let identifier = btree.root_identifier_lock.write().expect("Other threads shouldn't panic with lock");
    RootIdentifierWriteGuard {
      identifier
    }
  }
}

pub enum WriteGuard<'a> {
  RootIdentifierWriteGuard(RootIdentifierWriteGuard<'a>),
  NodeWriteGuard(NodeWriteGuard<'a>),
}

impl<'a> WriteGuard<'a> {
  pub fn acquire(btree: &'a BTree, target: LockTargetRef) -> WriteGuard<'a> {
    match target {
      LockTargetRef::RootIdentifierTarget => {
        WriteGuard::RootIdentifierWriteGuard(RootIdentifierWriteGuard::acquire(btree))
      },
      LockTargetRef::NodeTarget { identifier } => {
        WriteGuard::NodeWriteGuard(NodeWriteGuard::acquire(btree, identifier))
      }
    }
  }

  pub fn unwrap_node_write_guard_ref(&self, message: &'static str) -> &NodeWriteGuard {
    match self {
      WriteGuard::RootIdentifierWriteGuard(..) => panic!(message),
      WriteGuard::NodeWriteGuard(nwg) => nwg
    }
  }

  pub fn unwrap_node_write_guard(self, message: &'static str) -> NodeWriteGuard<'a> {
    match self {
      WriteGuard::RootIdentifierWriteGuard(..) => panic!(message),
      WriteGuard::NodeWriteGuard(nwg) => nwg
    }
  }
}
