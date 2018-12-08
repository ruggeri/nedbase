use btree::BTree;
use locking::LockTargetRef;
use std::sync::Arc;

rental! {
  mod rentals {
    use btree::BTree;
    use parking_lot::RwLockWriteGuard;
    use std::sync::Arc;

    #[rental(deref_mut_suffix)]
    pub struct RootIdentifierWriteGuard {
      btree: Arc<BTree>,
      guard: RwLockWriteGuard<'btree, String>,
    }
  }
}

pub use self::rentals::RootIdentifierWriteGuard;

impl RootIdentifierWriteGuard {
  pub fn acquire(btree: &Arc<BTree>) -> RootIdentifierWriteGuard {
    ::util::log_root_locking(
      "trying to acquire write lock on root identifier",
    );
    let btree = Arc::clone(btree);

    RootIdentifierWriteGuard::new(btree, |btree| {
      let guard = btree.root_identifier_lock().write();
      ::util::log_root_locking(
        "acquired write lock on root identifier",
      );
      guard
    })
  }

  pub fn location(&self) -> LockTargetRef {
    LockTargetRef::RootIdentifierTarget
  }
}
