use btree::BTree;
use locking::LockTargetRef;
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
    ::util::log_node_locking(&format!(
      "trying to acquire read lock on node {}",
      identifier
    ));
    let lock = btree.get_node_arc_lock(&identifier);

    NodeReadGuard::new(lock, |lock| {
      let guard = lock.read();
      ::util::log_node_locking(&format!(
        "acquired read lock on node {}",
        identifier
      ));

      guard
    })
  }

  pub fn location(&self) -> LockTargetRef {
    LockTargetRef::NodeTarget(self.identifier())
  }

  pub fn is_interior_node(&self) -> bool {
    self.node().is_interior_node()
  }

  pub fn is_leaf_node(&self) -> bool {
    self.node().is_leaf_node()
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
}
