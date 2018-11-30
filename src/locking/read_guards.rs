use btree::BTree;
use locking::LockTargetRef;
use std::sync::{Arc};

rental! {
  mod rentals {
    use btree::BTree;
    use node::Node;
    use parking_lot::{RwLock, RwLockReadGuard};
    use std::sync::{Arc};

    #[rental(deref_suffix)]
    pub struct NodeReadGuard {
      lock: Arc<RwLock<Node>>,
      guard: RwLockReadGuard<'lock, Node>,
    }

    #[rental(deref_suffix)]
    pub struct RootIdentifierReadGuard {
      btree: Arc<BTree>,
      guard: RwLockReadGuard<'btree, String>,
    }
  }
}

pub use self::rentals::{NodeReadGuard, RootIdentifierReadGuard};

impl NodeReadGuard {
  pub fn acquire(btree: &BTree, identifier: &str) -> NodeReadGuard {
    ::util::log_node_locking(&format!("trying to acquire read lock on node {}", identifier));
    let lock = btree.get_node_arc_lock(&identifier);

    NodeReadGuard::new(Arc::clone(&lock), |lock| {
      let guard = lock.read();
      ::util::log_node_locking(&format!("acquired read lock on node {}", identifier));

      guard
    })
  }

  pub fn try_to_acquire(btree: &BTree, identifier: &str) -> Option<NodeReadGuard> {
    ::util::log_node_locking(&format!("trying to acquire read lock (timed) on node {}", identifier));
    let lock = btree.get_node_arc_lock(&identifier);

    NodeReadGuard::try_new(Arc::clone(&lock), |lock| {
      match lock.try_read_for(::std::time::Duration::from_millis(1)) {
        None => {
          ::util::log_node_locking(&format!("abandoned read lock acquisition on node {}", identifier));
          Err(())
        },
        Some(node_guard) => {
          ::util::log_node_locking(&format!("acquired read lock on node {}", identifier));
          Ok(node_guard)
        }
      }
    }).ok()
  }

  pub fn location(&self) -> LockTargetRef {
    LockTargetRef::NodeTarget { identifier: self.identifier() }
  }
}

// impl Drop for NodeReadGuard {
//   fn drop(&mut self) {
//     // I've put this here to prohibit anyone from moving the read guard
//     // out. That seems dangerous (is it though?).
//     ::util::thread_log(&format!("released read lock on node {}", self.identifier()));
//   }
// }

impl RootIdentifierReadGuard {
  pub fn acquire(btree: &Arc<BTree>) -> RootIdentifierReadGuard {
    RootIdentifierReadGuard::new(Arc::clone(btree), |btree| {
      ::util::log_root_locking("trying to acquire read lock on root identifier");
      let guard = btree.root_identifier_lock.read();
      ::util::log_root_locking("acquired read lock on root identifier");
      guard
    })
  }

  pub fn try_to_acquire(btree: &Arc<BTree>) -> Option<RootIdentifierReadGuard> {
    ::util::log_root_locking("trying to acquire read lock on root identifier (timed)");

    RootIdentifierReadGuard::try_new(Arc::clone(btree), |btree| {
      match btree.root_identifier_lock.try_read_for(::std::time::Duration::from_millis(1)) {
        None => {
          ::util::log_root_locking("abandoned trying to acquire read lock on root identifier");
          Err(())
        },
        Some(identifier_guard) => {
          ::util::log_root_locking("acquired read lock on root identifier");
          Ok(identifier_guard)
        },
      }
    }).ok()
  }

  pub fn location(&self) -> LockTargetRef {
    LockTargetRef::RootIdentifierTarget
  }
}

// impl Drop for RootIdentifierReadGuard {
//   fn drop(&mut self) {
//     // I've put this here to prohibit anyone from moving the read guard
//     // out. That seems dangerous (is it though?).
//     ::util::thread_log("released read lock on root identifier");
//   }
// }

pub enum ReadGuard {
  RootIdentifierReadGuard(RootIdentifierReadGuard),
  NodeReadGuard(NodeReadGuard),
}

impl ReadGuard {
  pub fn acquire(btree: &Arc<BTree>, target: LockTargetRef) -> ReadGuard {
    match target {
      LockTargetRef::RootIdentifierTarget => {
        ReadGuard::RootIdentifierReadGuard(RootIdentifierReadGuard::acquire(btree))
      },
      LockTargetRef::NodeTarget { identifier } => {
        ReadGuard::NodeReadGuard(NodeReadGuard::acquire(btree, identifier))
      }
    }
  }

  pub fn try_to_acquire(btree: &Arc<BTree>, target: LockTargetRef) -> Option<ReadGuard> {
    match target {
      LockTargetRef::RootIdentifierTarget => {
        RootIdentifierReadGuard::try_to_acquire(btree)
          .map(ReadGuard::RootIdentifierReadGuard)
      },
      LockTargetRef::NodeTarget { identifier } => {
        NodeReadGuard::try_to_acquire(btree, identifier)
          .map(ReadGuard::NodeReadGuard)
      }
    }
  }

  pub fn location(&self) -> LockTargetRef {
    match self {
      ReadGuard::RootIdentifierReadGuard(guard) => guard.location(),
      ReadGuard::NodeReadGuard(guard) => guard.location(),
    }
  }

  pub fn unwrap_root_read_guard_ref(&self, message: &'static str) -> &RootIdentifierReadGuard {
    match self {
      ReadGuard::NodeReadGuard(..) => panic!(message),
      ReadGuard::RootIdentifierReadGuard(root_guard) => root_guard
    }
  }

  pub fn unwrap_node_read_guard_ref(&self, message: &'static str) -> &NodeReadGuard {
    match self {
      ReadGuard::RootIdentifierReadGuard(..) => panic!(message),
      ReadGuard::NodeReadGuard(node_guard) => node_guard
    }
  }
}
