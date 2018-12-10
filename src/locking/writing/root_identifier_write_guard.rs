use btree::BTree;
use locking::{LockTargetRef, WriteGuard};
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
    let btree = Arc::clone(btree);

    RootIdentifierWriteGuard::new(btree, |btree| {
      btree.root_identifier_lock().write()
    })
  }

  pub fn as_str_ref(&self) -> &str {
    &(*self)
  }

  pub fn location(&self) -> LockTargetRef {
    LockTargetRef::RootIdentifierTarget
  }

  pub fn upcast(self) -> WriteGuard {
    WriteGuard::RootIdentifierWriteGuard(self)
  }
}
