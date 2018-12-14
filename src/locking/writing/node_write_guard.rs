use btree::BTree;
use locking::WriteGuard;
use node::{InteriorNode, Node};
use parking_lot::{RwLock, RwLockWriteGuard};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct NodeWriteGuard {
  _lock: Arc<RwLock<Node>>,
  guard: RwLockWriteGuard<'static, Node>,
}

impl Deref for NodeWriteGuard {
  type Target = Node;

  fn deref(&self) -> &Node {
    &self.guard
  }
}

impl DerefMut for NodeWriteGuard {
  fn deref_mut(&mut self) -> &mut Node {
    &mut self.guard
  }
}

impl NodeWriteGuard {
  pub fn acquire(btree: &BTree, identifier: &str) -> NodeWriteGuard {
    unsafe {
      let lock: Arc<RwLock<Node>> = btree.get_node_arc_lock(&identifier);

      let guard: RwLockWriteGuard<'static, Node> = std::mem::transmute(
        lock.write()
      );

      NodeWriteGuard {
        _lock: lock,
        guard
      }
    }
  }

  pub fn node(&self) -> &Node {
    &(*self)
  }

  pub fn upcast(self) -> WriteGuard {
    WriteGuard::NodeWriteGuard(self)
  }
}

// This method is sort-of monkey-patched here because it's really about
// NodeWriteGuard much more than InteriorNode.
impl InteriorNode {
  pub fn acquire_write_guard_for_child_by_key(
    &self,
    btree: &BTree,
    key: &str,
  ) -> NodeWriteGuard {
    let child_identifier = self.child_identifier_by_key(key);
    NodeWriteGuard::acquire(btree, child_identifier)
  }
}
