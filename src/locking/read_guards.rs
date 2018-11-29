use btree::BTree;
use node::Node;
use std::mem;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use super::lock_target::LockTargetRef;

pub struct NodeReadGuard<'a> {
  pub lock: Arc<RwLock<Node>>,
  pub node: RwLockReadGuard<'a, Node>,
}

impl<'a> NodeReadGuard<'a> {
  pub fn acquire(btree: &'a BTree, identifier: &str) -> NodeReadGuard<'a> {
    let lock = btree.get_node_arc_lock(&identifier);
    NodeReadGuard::acquire_from_lock(lock)
  }

  pub fn acquire_from_lock(lock: Arc<RwLock<Node>>) -> NodeReadGuard<'a> {
    let node = lock.read().expect("Other threads shouldn't panic with lock");
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
  }
}

pub struct RootIdentifierReadGuard<'a> {
  pub identifier: RwLockReadGuard<'a, String>,
}

impl<'a> RootIdentifierReadGuard<'a> {
  pub fn acquire(btree: &'a BTree) -> RootIdentifierReadGuard<'a> {
    let identifier = btree.root_identifier_lock.read().expect("Other threads shouldn't panic with lock");
    RootIdentifierReadGuard {
      identifier
    }
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
