use btree::BTree;
use locking::ReadGuard;
use parking_lot::RwLockReadGuard;
use std::ops::Deref;
use std::sync::Arc;

pub struct RootIdentifierReadGuard {
  _btree: Arc<BTree>,
  guard: RwLockReadGuard<'static, String>,
}

impl Deref for RootIdentifierReadGuard {
  type Target = String;

  fn deref(&self) -> &String {
    &self.guard
  }
}

impl RootIdentifierReadGuard {
  pub fn acquire(btree: &Arc<BTree>) -> RootIdentifierReadGuard {
    unsafe {
      let lock = btree.root_identifier_lock();
      let guard: RwLockReadGuard<'static, String> = std::mem::transmute(
        lock.read()
      );

      let btree: Arc<BTree> = Arc::clone(btree);
      RootIdentifierReadGuard {
        _btree: btree,
        guard
      }
    }
  }

  pub fn as_str_ref(&self) -> &str {
    &(*self)
  }

  pub fn upcast(self) -> ReadGuard {
    ReadGuard::RootIdentifierReadGuard(self)
  }
}
