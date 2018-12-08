use btree::BTree;
use locking::LockTargetRef;

rental! {
  mod rentals {
    use node::Node;
    use parking_lot::{RwLock, RwLockWriteGuard};
    use std::sync::Arc;

    #[rental(deref_mut_suffix)]
    pub struct NodeWriteGuard {
        lock: Arc<RwLock<Node>>,
        guard: RwLockWriteGuard<'lock, Node>,
    }
  }
}

pub use self::rentals::NodeWriteGuard;

impl NodeWriteGuard {
  pub fn acquire(btree: &BTree, identifier: &str) -> NodeWriteGuard {
    ::util::log_node_locking(&format!(
      "trying to acquire write lock on node {}",
      identifier
    ));
    let lock = btree.get_node_arc_lock(&identifier);

    NodeWriteGuard::new(lock, |lock| {
      let guard = lock.write();
      ::util::log_node_locking(&format!(
        "acquired write lock on node {}",
        identifier
      ));
      guard
    })
  }

  pub fn location(&self) -> LockTargetRef {
    LockTargetRef::NodeTarget(self.identifier())
  }
}
