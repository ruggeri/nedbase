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
      ::util::log_root_locking(
        "trying to acquire read lock on root identifier",
      );
      let guard = btree.root_identifier_lock().read();
      ::util::log_root_locking("acquired read lock on root identifier");
      guard
    })
  }

  pub fn location(&self) -> LockTargetRef {
    LockTargetRef::RootIdentifierTarget
  }
}
