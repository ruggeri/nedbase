use btree::BTree;
use locking::WriteGuard;
use node::Node;
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
  pub(in locking) fn acquire(
    btree: &BTree,
    identifier: &str,
  ) -> NodeWriteGuard {
    // This is trickery. `RwLockWriteGuard` wants a lifetime: it doesn't
    // want to outlive the `RwLock`. But the `RwLock` *cannot* be lost,
    // because I hold onto it via `Arc`.
    //
    // However, Rust won't understand this. Therefore, I resort to this
    // unsafe code.
    unsafe {
      let lock: Arc<RwLock<Node>> =
        btree.get_node_arc_lock(&identifier);

      let guard: RwLockWriteGuard<'static, Node> =
        std::mem::transmute(lock.write());

      NodeWriteGuard { _lock: lock, guard }
    }
  }

  pub fn upcast(self) -> WriteGuard {
    WriteGuard::NodeWriteGuard(self)
  }
}
