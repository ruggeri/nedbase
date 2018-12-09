use btree::BTree;
use locking::LockTargetRef;

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
}
