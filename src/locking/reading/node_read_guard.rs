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
    ::util::log_node_locking(&format!("trying to acquire read lock on node {}", identifier));
    let lock = btree.get_node_arc_lock(&identifier);

    NodeReadGuard::new(lock, |lock| {
      let guard = lock.read();
      ::util::log_node_locking(&format!("acquired read lock on node {}", identifier));

      guard
    })
  }

  pub fn try_timed_acquire(btree: &BTree, identifier: &str) -> Option<NodeReadGuard> {
    ::util::log_node_locking(&format!("trying timed acquire of read lock on node {}", identifier));
    let lock = btree.get_node_arc_lock(&identifier);

    NodeReadGuard::try_new(lock, |lock| {
      match lock.try_read_for(::std::time::Duration::from_millis(1)) {
        None => {
          ::util::log_node_locking(&format!("abandoned timed read lock acquisition on node {}", identifier));
          Err(())
        }

        Some(node_guard) => {
          ::util::log_node_locking(&format!("acquired read lock on node {}", identifier));
          Ok(node_guard)
        }
      }
    }).ok()
  }

  pub fn location(&self) -> LockTargetRef {
    LockTargetRef::NodeTarget(self.identifier())
  }
}
