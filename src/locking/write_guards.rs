use btree::BTree;
use std::sync::{Arc};
use super::lock_target::LockTargetRef;

rental! {
  mod rentals {
    use btree::BTree;
    use node::Node;
    use parking_lot::{RwLock, RwLockWriteGuard};
    use std::sync::{Arc};

    #[rental(deref_mut_suffix)]
    pub struct NodeWriteGuard {
        lock: Arc<RwLock<Node>>,
        guard: RwLockWriteGuard<'lock, Node>,
    }

    #[rental(deref_mut_suffix)]
    pub struct RootIdentifierWriteGuard {
      btree: Arc<BTree>,
      guard: RwLockWriteGuard<'btree, String>,
    }
  }
}

pub use self::rentals::NodeWriteGuard;
pub use self::rentals::RootIdentifierWriteGuard;

// pub struct NodeWriteGuard<'a> {
//   // This exists to ensure that the underlying lock lives as long as
//   // needed.
//   pub lock: Arc<RwLock<Node>>,
//   pub node: RwLockWriteGuard<'a, Node>,
// }

impl NodeWriteGuard {
  pub fn acquire(btree: &BTree, identifier: &str) -> NodeWriteGuard {
    ::util::log_node_locking(&format!("trying to acquire write lock on node {}", identifier));
    let lock = btree.get_node_arc_lock(&identifier);

    NodeWriteGuard::new(Arc::clone(&lock), |lock| {
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
}

// impl Drop for NodeWriteGuard {
//   fn drop(&mut self) {
//     // I've put this here to prohibit anyone from moving the write guard
//     // out. That seems dangerous (is it though?).
//     ::util::thread_log(&format!("released write lock on node {}", self.identifier()));
//   }
// }

// pub struct RootIdentifierWriteGuard {
//   pub identifier: RwLockWriteGuard<'a, String>,
// }

impl RootIdentifierWriteGuard {
  pub fn acquire(btree: &Arc<BTree>) -> RootIdentifierWriteGuard {
    ::util::log_root_locking("trying to acquire write lock on root identifier");
    let btree = Arc::clone(btree);

    RootIdentifierWriteGuard::new(btree, |btree| {
      let guard = btree.root_identifier_lock.write();
      ::util::log_root_locking("acquired write lock on root identifier");
      guard
    })
  }

  // pub fn acquire_if_no_current_writers(btree: &'a BTree) -> Option<RootIdentifierWriteGuard<'a>> {
  //   ::util::thread_log("trying to acquire read lock for upgrade on root identifier");
  //   let identifier = match btree.root_identifier_lock.try_upgradable_read() {
  //     None => {
  //       ::util::thread_log("could not acquire read lock for upgrade on root identifier");
  //       return None;
  //     },
  //     Some(upgradable_read_guard) => {
  //       ::util::thread_log("acquired read lock for upgrade on root identifier");
  //       ::util::thread_log("trying to upgrade read lock on root identifier");
  //       RwLockUpgradableReadGuard::upgrade(upgradable_read_guard)
  //     }
  //   };
  //   ::util::thread_log("acquired write lock on root identifier");

  //   Some(RootIdentifierWriteGuard {
  //     identifier
  //   })
  // }
}

// impl Drop for RootIdentifierWriteGuard {
//   fn drop(&mut self) {
//     // I've put this here to prohibit anyone from moving the write guard
//     // out. That seems dangerous (is it though?).
//     ::util::thread_log("released write lock on root identifier");
//   }
// }

pub enum WriteGuard {
  RootIdentifierWriteGuard(RootIdentifierWriteGuard),
  NodeWriteGuard(NodeWriteGuard),
}

impl WriteGuard {
  pub fn acquire(btree: &Arc<BTree>, target: LockTargetRef) -> WriteGuard {
    match target {
      LockTargetRef::RootIdentifierTarget => {
        WriteGuard::RootIdentifierWriteGuard(RootIdentifierWriteGuard::acquire(btree))
      },
      LockTargetRef::NodeTarget { identifier } => {
        WriteGuard::NodeWriteGuard(NodeWriteGuard::acquire(btree, identifier))
      }
    }
  }

  // pub fn acquire_if_no_current_writers(btree: &'a BTree, target: LockTargetRef) -> Option<WriteGuard<'a>> {
  //   match target {
  //     LockTargetRef::RootIdentifierTarget => {
  //       RootIdentifierWriteGuard::acquire_if_no_current_writers(btree)
  //         .map(WriteGuard::RootIdentifierWriteGuard)
  //     },
  //     LockTargetRef::NodeTarget { identifier } => {
  //       NodeWriteGuard::acquire_if_no_current_writers(btree, identifier)
  //         .map(WriteGuard::NodeWriteGuard)
  //     }
  //   }
  // }

  pub fn unwrap_node_write_guard_ref(&self, message: &'static str) -> &NodeWriteGuard {
    match self {
      WriteGuard::RootIdentifierWriteGuard(..) => panic!(message),
      WriteGuard::NodeWriteGuard(nwg) => nwg
    }
  }

  pub fn unwrap_node_write_guard(self, message: &'static str) -> NodeWriteGuard {
    match self {
      WriteGuard::RootIdentifierWriteGuard(..) => panic!(message),
      WriteGuard::NodeWriteGuard(nwg) => nwg
    }
  }
}
