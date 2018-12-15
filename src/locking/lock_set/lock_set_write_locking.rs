use super::{
  LockMode, LockSet, LockSetNodeWriteGuard,
  LockSetRootIdentifierWriteGuard, LockSetValue,
};
use locking::{Guard, LockTarget, TransactionMode, WriteGuard};
use std::cell::RefCell;
use std::rc::Rc;

impl LockSet {
  pub fn node_write_guard_for_hold(
    &mut self,
    identifier: &str,
  ) -> LockSetNodeWriteGuard {
    let guard = self.write_guard_for_hold(&LockTarget::Node(
      String::from(identifier),
    ));
    LockSetNodeWriteGuard::from_guard(guard)
  }

  pub fn root_identifier_write_guard_for_hold(
    &mut self,
  ) -> LockSetRootIdentifierWriteGuard {
    let guard = self.write_guard_for_hold(&LockTarget::RootIdentifier);
    LockSetRootIdentifierWriteGuard::from_guard(guard)
  }

  fn write_guard_for_hold(
    &mut self,
    lock_target: &LockTarget,
  ) -> Rc<RefCell<Guard>> {
    if !self.guards.contains_key(lock_target) {
      return self.acquire_write_guard_for_hold(lock_target);
    }

    // Attempt to upgrade the retained lock.
    if let Some(guard) = self.upgrade_for_write_hold(lock_target) {
      return guard;
    }

    // We failed the upgrade. So we'll have to reacquire.
    self.acquire_write_guard_for_hold(lock_target)
  }

  fn acquire_write_guard_for_hold(
    &mut self,
    lock_target: &LockTarget,
  ) -> Rc<RefCell<Guard>> {
    let guard = match self.tx_mode {
      TransactionMode::ReadOnly => {
        panic!("Must not acquire write guards in ReadOnly transaction");
      }

      TransactionMode::ReadWrite => {
        let guard =
          WriteGuard::acquire_write_guard(&self.btree, lock_target);
        Guard::Write(guard)
      }
    };

    // Next, wrap it in RefCell so that someone can borrow a guard for
    // mutation. (Though this is a read guard.)
    let guard = RefCell::new(guard);
    // Next, wrap in Rc so that user can hold on to as long as they
    // need.
    let guard = Rc::new(guard);

    let lock_set_value = LockSetValue {
      lock_mode: LockMode::Write,
      guard: Rc::downgrade(&guard),
    };

    // Store a weak version in the map.
    self.guards.insert(lock_target.clone(), lock_set_value);

    guard
  }

  fn upgrade_for_write_hold(
    &mut self,
    lock_target: &LockTarget,
  ) -> Option<Rc<RefCell<Guard>>> {
    let LockSetValue { lock_mode, guard } = &self.guards[lock_target];

    if self.tx_mode == TransactionMode::ReadOnly {
      panic!("cannot acquire read locks in ReadOnly mode!");
    }

    let guard = match guard.upgrade() {
      None => return None,
      Some(guard) => guard,
    };

    // The problem: what if we acquired a *temporary* read lock on this
    // node, and are now forced to take a *write* lock because we are in
    // ReadWrite mode?
    match lock_mode {
      LockMode::Read => {
        panic!("Can't hold a write guard on a previously temp guard");
      }

      LockMode::Write => Some(guard),
    }
  }
}
