use btree::BTree;
use locking::{LockTargetRef, ReadGuard};
use node::{InteriorNode, LeafNode, Node};

rental! {
  mod rentals {
    use node::Node;
    use parking_lot::{RwLock, RwLockReadGuard};
    use std::sync::Arc;

    #[rental(deref_suffix)]
    pub struct NodeReadGuard {
      lock: Arc<RwLock<Node>>,
      guard: RwLockReadGuard<'lock, Node>,
    }
  }
}

pub use self::rentals::NodeReadGuard;

impl NodeReadGuard {
  pub fn acquire(btree: &BTree, identifier: &str) -> NodeReadGuard {
    let lock = btree.get_node_arc_lock(&identifier);
    NodeReadGuard::new(lock, |lock| {
      lock.read()
    })
  }

  pub fn is_interior_node(&self) -> bool {
    self.node().is_interior_node()
  }

  pub fn is_leaf_node(&self) -> bool {
    self.node().is_leaf_node()
  }

  pub fn location(&self) -> LockTargetRef {
    LockTargetRef::NodeTarget(self.identifier())
  }

  pub fn node(&self) -> &Node {
    &(*self)
  }

  pub fn unwrap_interior_node_ref(&self, message: &'static str) -> &InteriorNode {
    self.node().unwrap_interior_node_ref(message)
  }

  pub fn unwrap_leaf_node_ref(&self, message: &'static str) -> &LeafNode {
    self.node().unwrap_leaf_node_ref(message)
  }

  pub fn upcast(self) -> ReadGuard {
    ReadGuard::NodeReadGuard(self)
  }
}

// This method is sort-of monkey-patched here because it's really about
// NodeReadGuard much more than InteriorNode.
impl InteriorNode {
  pub fn acquire_read_guard_for_child_by_key(&self, btree: &BTree, key: &str) -> NodeReadGuard {
    let child_identifier = self.child_identifier_by_key(key);
    NodeReadGuard::acquire(btree, child_identifier)
  }
}
