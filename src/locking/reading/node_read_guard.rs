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
  pub(in locking) fn acquire(btree: &BTree, identifier: &str) -> NodeReadGuard {
    // This is trickery. `RwLockReadGuard` wants a lifetime: it doesn't
    // want to outlive the `RwLock`. But the `RwLock` *cannot* be lost,
    // because I hold onto it via `Arc`.
    //
    // However, Rust won't understand this. Therefore, I resort to this
    // unsafe code.
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
    let self_node: &Node = self;
    self_node.is_interior_node()
  }

  pub fn is_leaf_node(&self) -> bool {
    let self_node: &Node = self;
    self_node.is_leaf_node()
  }

  pub fn unwrap_interior_node_ref(
    &self,
    message: &'static str,
  ) -> &InteriorNode {
    let self_node: &Node = self;
    self_node.unwrap_interior_node_ref(message)
  }

  pub fn unwrap_leaf_node_ref(
    &self,
    message: &'static str,
  ) -> &LeafNode {
    let self_node: &Node = self;
    self_node.unwrap_leaf_node_ref(message)
  }

  pub fn upcast(self) -> ReadGuard {
    ReadGuard::NodeReadGuard(self)
  }
}
