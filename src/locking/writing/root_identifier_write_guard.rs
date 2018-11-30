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

  pub fn location(&self) -> LockTargetRef {
    LockTargetRef::RootIdentifierTarget
  }
}

// TODO: I can't implement this drop logic it seems? Rental complains?

// impl Drop for RootIdentifierWriteGuard {
//   fn drop(&mut self) {
//     // I've put this here to prohibit anyone from moving the write guard
//     // out. That seems dangerous (is it though?).
//     ::util::thread_log("released write lock on root identifier");
//   }
// }
