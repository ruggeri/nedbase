use btree::BTree;
use locking::{LockTargetRef, ReadGuard};
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
      btree.root_identifier_lock().read()
    })
  }

  pub fn as_str_ref(&self) -> &str {
    &(*self)
  }

  pub fn location(&self) -> LockTargetRef {
    LockTargetRef::RootIdentifierTarget
  }

  pub fn upcast(self) -> ReadGuard {
    ReadGuard::RootIdentifierReadGuard(self)
  }
}
