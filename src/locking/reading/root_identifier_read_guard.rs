use btree::BTree;
use locking::LockTargetRef;
use std::sync::Arc;

rental! {
  mod rentals {
    use btree::BTree;
    use parking_lot::RwLockReadGuard;
    use std::sync::Arc;

    #[rental(deref_suffix)]
    pub struct RootIdentifierReadGuard {
      btree: Arc<BTree>,
      guard: RwLockReadGuard<'btree, String>,
    }
  }
}

pub use self::rentals::RootIdentifierReadGuard;

impl RootIdentifierReadGuard {
  pub fn acquire(btree: &Arc<BTree>) -> RootIdentifierReadGuard {
    RootIdentifierReadGuard::new(Arc::clone(btree), |btree| {
      ::util::log_root_locking("trying to acquire read lock on root identifier");
      let guard = btree.root_identifier_lock.read();
      ::util::log_root_locking("acquired read lock on root identifier");
      guard
    })
  }

  pub fn try_timed_acquire(btree: &Arc<BTree>) -> Option<RootIdentifierReadGuard> {
    ::util::log_root_locking("trying timed acquire of read lock on root identifier (timed)");

    RootIdentifierReadGuard::try_new(Arc::clone(btree), |btree| {
      match btree.root_identifier_lock.try_read_for(::std::time::Duration::from_millis(1)) {
        None => {
          ::util::log_root_locking("abandoned timed read lock acquisition on root identifier");
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

// TODO: I can't implement this drop logic it seems? Rental complains?

// impl Drop for RootIdentifierReadGuard {
//   fn drop(&mut self) {
//     ::util::thread_log("released read lock on root identifier");
//   }
// }
