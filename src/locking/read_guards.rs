use btree::BTree;
use node::Node;
use std::mem;
use parking_lot::{RwLock, RwLockReadGuard};
use std::sync::{Arc};
use super::lock_target::LockTargetRef;

pub struct NodeReadGuard<'a> {
  pub lock: Arc<RwLock<Node>>,
  pub node: RwLockReadGuard<'a, Node>,
}

impl<'a> NodeReadGuard<'a> {
  pub fn acquire(btree: &'a BTree, identifier: &str) -> NodeReadGuard<'a> {
    ::util::thread_log(&format!("trying to acquire read lock on node {}", identifier));
    let lock = btree.get_node_arc_lock(&identifier);
    let guard = NodeReadGuard::acquire_from_lock(lock);
    ::util::thread_log(&format!("acquired read lock on node {}", identifier));

    guard
  }

  pub fn acquire_from_lock(lock: Arc<RwLock<Node>>) -> NodeReadGuard<'a> {
    let node = lock.read();
    let guard = NodeReadGuard { lock: Arc::clone(&lock), node };

    unsafe {
      mem::transmute(guard)
    }
  }
}

impl<'a> Drop for NodeReadGuard<'a> {
  fn drop(&mut self) {
    // I've put this here to prohibit anyone from moving the read guard
    // out. That seems dangerous (is it though?).
    ::util::thread_log(&format!("released read lock on node {}", self.node.identifier()));
  }
}

pub struct RootIdentifierReadGuard<'a> {
  pub identifier: RwLockReadGuard<'a, String>,
}

impl<'a> RootIdentifierReadGuard<'a> {
  pub fn acquire(btree: &'a BTree) -> RootIdentifierReadGuard<'a> {
    ::util::thread_log("trying to acquire read lock on root identifier");
    let identifier = btree.root_identifier_lock.read();
    ::util::thread_log("acquired read lock on root identifier");
    RootIdentifierReadGuard {
      identifier
    }
  }
}

impl<'a> Drop for RootIdentifierReadGuard<'a> {
  fn drop(&mut self) {
    // I've put this here to prohibit anyone from moving the read guard
    // out. That seems dangerous (is it though?).
    ::util::thread_log("released read lock on root identifier");
  }
}

pub enum ReadGuard<'a> {
  RootIdentifierReadGuard(RootIdentifierReadGuard<'a>),
  NodeReadGuard(NodeReadGuard<'a>),
}

impl<'a> ReadGuard<'a> {
  pub fn acquire(btree: &'a BTree, target: LockTargetRef) -> ReadGuard<'a> {
    match target {
      LockTargetRef::RootIdentifierTarget => {
        ReadGuard::RootIdentifierReadGuard(RootIdentifierReadGuard::acquire(btree))
      },
      LockTargetRef::NodeTarget { identifier } => {
        ReadGuard::NodeReadGuard(NodeReadGuard::acquire(btree, identifier))
      }
    }
  }
}
