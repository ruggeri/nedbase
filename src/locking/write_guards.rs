use btree::BTree;
use node::Node;
use std::mem;
use parking_lot::{RwLock, RwLockWriteGuard};
use std::sync::{Arc};
use super::lock_target::LockTargetRef;

pub struct NodeWriteGuard<'a> {
  // This exists to ensure that the underlying lock lives as long as
  // needed.
  pub lock: Arc<RwLock<Node>>,
  pub node: RwLockWriteGuard<'a, Node>,
}

impl<'a> NodeWriteGuard<'a> {
  pub fn acquire(btree: &BTree, identifier: &str) -> NodeWriteGuard<'a> {
    ::util::thread_log(&format!("trying to acquire write lock on node {}", identifier));
    let lock = btree.get_node_arc_lock(&identifier);
    let node = lock.write();
    let guard = NodeWriteGuard { lock: Arc::clone(&lock), node };
    ::util::thread_log(&format!("acquired write lock on node {}", identifier));

    unsafe {
      mem::transmute(guard)
    }
  }
}

impl<'a> Drop for NodeWriteGuard<'a> {
  fn drop(&mut self) {
    // I've put this here to prohibit anyone from moving the write guard
    // out. That seems dangerous (is it though?).
    ::util::thread_log(&format!("released write lock on node {}", self.node.identifier()));
  }
}

pub struct RootIdentifierWriteGuard<'a> {
  pub identifier: RwLockWriteGuard<'a, String>,
}

impl<'a> RootIdentifierWriteGuard<'a> {
  pub fn acquire(btree: &'a BTree) -> RootIdentifierWriteGuard<'a> {
    ::util::thread_log("trying to acquire write lock on root identifier");
    let identifier = btree.root_identifier_lock.write();
    ::util::thread_log("did acquire write lock on root identifier");
    RootIdentifierWriteGuard {
      identifier
    }
  }
}

impl<'a> Drop for RootIdentifierWriteGuard<'a> {
  fn drop(&mut self) {
    // I've put this here to prohibit anyone from moving the write guard
    // out. That seems dangerous (is it though?).
    ::util::thread_log("released write lock on root identifier");
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
