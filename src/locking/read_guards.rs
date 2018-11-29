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
    let node = lock.read();
    ::util::thread_log(&format!("acquired read lock on node {}", identifier));
    let guard = NodeReadGuard { lock: Arc::clone(&lock), node };

    unsafe {
      mem::transmute(guard)
    }
  }

  pub fn try_to_acquire(btree: &'a BTree, identifier: &str) -> Option<NodeReadGuard<'a>> {
    ::util::thread_log(&format!("trying to acquire read lock on node {}", identifier));
    let lock = btree.get_node_arc_lock(&identifier);
    let node = match lock.try_read_for(::std::time::Duration::from_millis(1)) {
      None => {
        ::util::thread_log(&format!("abandoned read lock acquisition on node {}", identifier));
        return None
      },
      Some(node_guard) => node_guard
    };
    ::util::thread_log(&format!("acquired read lock on node {}", identifier));
    let guard = Some(NodeReadGuard { lock: Arc::clone(&lock), node });

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

  pub fn try_to_acquire(btree: &'a BTree) -> Option<RootIdentifierReadGuard<'a>> {
    ::util::thread_log("trying to acquire read lock on root identifier");
    let identifier = match btree.root_identifier_lock.try_read_for(::std::time::Duration::from_millis(1)) {
      None => {
        ::util::thread_log("abandoned trying to acquire read lock on root identifier");
        return None
      },
      Some(identifier_guard) => identifier_guard,
    };
    ::util::thread_log("acquired read lock on root identifier");

    Some(RootIdentifierReadGuard {
      identifier
    })
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
