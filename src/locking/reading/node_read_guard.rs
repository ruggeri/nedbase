use btree::BTree;
use locking::ReadGuard;
use node::{InteriorNode, LeafNode, Node};
use parking_lot::{RwLock, RwLockReadGuard};
use std::ops::Deref;
use std::sync::Arc;

pub struct NodeReadGuard {
  _lock: Arc<RwLock<Node>>,
  guard: RwLockReadGuard<'static, Node>,
}

impl Deref for NodeReadGuard {
  type Target = Node;

  fn deref(&self) -> &Node {
    &self.guard
  }
}

impl NodeReadGuard {
  pub fn acquire(btree: &BTree, identifier: &str) -> NodeReadGuard {
    unsafe {
      let lock: Arc<RwLock<Node>> = btree.get_node_arc_lock(&identifier);

      let guard: RwLockReadGuard<'static, Node> = std::mem::transmute(
        lock.read()
      );

      NodeReadGuard {
        _lock: lock,
        guard
      }
    }
  }

  pub fn is_interior_node(&self) -> bool {
    self.node().is_interior_node()
  }

  pub fn is_leaf_node(&self) -> bool {
    self.node().is_leaf_node()
  }

  pub fn node(&self) -> &Node {
    self
  }

  pub fn unwrap_interior_node_ref(
    &self,
    message: &'static str,
  ) -> &InteriorNode {
    self.node().unwrap_interior_node_ref(message)
  }

  pub fn unwrap_leaf_node_ref(
    &self,
    message: &'static str,
  ) -> &LeafNode {
    self.node().unwrap_leaf_node_ref(message)
  }

  pub fn upcast(self) -> ReadGuard {
    ReadGuard::NodeReadGuard(self)
  }
}

// This method is sort-of monkey-patched here because it's really about
// NodeReadGuard much more than InteriorNode.
impl InteriorNode {
  pub fn acquire_read_guard_for_child_by_key(
    &self,
    btree: &BTree,
    key: &str,
  ) -> NodeReadGuard {
    let child_identifier = self.child_identifier_by_key(key);
    NodeReadGuard::acquire(btree, child_identifier)
  }
}
