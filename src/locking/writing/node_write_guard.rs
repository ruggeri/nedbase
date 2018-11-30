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
    ::util::log_node_locking(&format!("trying to acquire write lock on node {}", identifier));
    let lock = btree.get_node_arc_lock(&identifier);

    NodeWriteGuard::new(lock, |lock| {
      let guard = lock.write();
      ::util::log_node_locking(&format!("acquired write lock on node {}", identifier));
      guard
    })
  }

  // pub fn acquire_if_no_current_writers(btree: &BTree, identifier: &str) -> Option<NodeWriteGuard<'a>> {
  //   ::util::thread_log(&format!("trying to acquire read lock for upgrade on node {}", identifier));
  //   let lock = btree.get_node_arc_lock(&identifier);
  //   let node = match lock.try_upgradable_read() {
  //     None => {
  //       ::util::thread_log(&format!("could not acquire read lock for upgrade on node {}", identifier));
  //       return None
  //     },
  //     Some(upgradable_read_guard) => {
  //       ::util::thread_log(&format!("acquired read lock for upgrade on node {}", identifier));
  //       ::util::thread_log(&format!("trying to upgrade write lock on node {}", identifier));
  //       RwLockUpgradableReadGuard::upgrade(upgradable_read_guard)
  //     }
  //   };
  //   ::util::thread_log(&format!("did upgrade write lock on node {}", identifier));

  //   let guard = Some(NodeWriteGuard { lock: Arc::clone(&lock), node });

  //   unsafe {
  //     mem::transmute(guard)
  //   }
  // }

  pub fn location(&self) -> LockTargetRef {
    LockTargetRef::NodeTarget(self.identifier())
  }
}

// TODO: I can't implement this drop logic it seems? Rental complains?

// impl Drop for NodeWriteGuard {
//   fn drop(&mut self) {
//     // I've put this here to prohibit anyone from moving the write guard
//     // out. That seems dangerous (is it though?).
//     ::util::thread_log(&format!("released write lock on node {}", self.identifier()));
//   }
// }
