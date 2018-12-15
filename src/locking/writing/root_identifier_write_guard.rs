use btree::BTree;
use locking::WriteGuard;
use parking_lot::RwLockWriteGuard;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct RootIdentifierWriteGuard {
  _btree: Arc<BTree>,
  guard: RwLockWriteGuard<'static, String>,
}

impl Deref for RootIdentifierWriteGuard {
  type Target = String;

  fn deref(&self) -> &String {
    &self.guard
  }
}

impl DerefMut for RootIdentifierWriteGuard {
  fn deref_mut(&mut self) -> &mut String {
    &mut self.guard
  }
}

impl RootIdentifierWriteGuard {
  // This is trickery. `RwLockWriteGuard` wants a lifetime: it doesn't
  // want to outlive the `BTree`. But the `BTree` *cannot* be lost,
  // because I hold onto it via `Arc`.
  //
  // However, Rust won't understand this. Therefore, I resort to this
  // unsafe code.
  pub(in locking) fn acquire(btree: &Arc<BTree>) -> RootIdentifierWriteGuard {
    unsafe {
      let lock = btree.root_identifier_lock();
      let guard: RwLockWriteGuard<'static, String> = std::mem::transmute(
        lock.write()
      );

      let btree: Arc<BTree> = Arc::clone(btree);
      RootIdentifierWriteGuard {
        _btree: btree,
        guard
      }
    }
  }

  pub fn as_str_ref(&self) -> &str {
    &(*self)
  }

  pub fn upcast(self) -> WriteGuard {
    WriteGuard::RootIdentifierWriteGuard(self)
  }
}
