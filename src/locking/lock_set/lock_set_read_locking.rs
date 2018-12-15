use super::{
  LockMode, LockSet, LockSetNodeReadGuard,
  LockSetRootIdentifierReadGuard,
};
use locking::{
  Guard, LockTarget, ReadGuard, TransactionMode, WriteGuard,
};
use std::cell::RefCell;
use std::rc::Rc;

impl LockSet {
  pub fn node_read_guard_for_hold(
    &mut self,
    identifier: &str,
  ) -> LockSetNodeReadGuard {
    let guard = self
      .read_guard_for_hold(&LockTarget::Node(String::from(identifier)));
    LockSetNodeReadGuard::from_guard(guard)
  }

  pub fn root_identifier_read_guard_for_hold(
    &mut self,
  ) -> LockSetRootIdentifierReadGuard {
    let guard = self.read_guard_for_hold(&LockTarget::RootIdentifier);
    LockSetRootIdentifierReadGuard::from_guard(guard)
  }

  fn read_guard_for_hold(
    &mut self,
    lock_target: &LockTarget,
  ) -> Rc<RefCell<Guard>> {
    if !self.guards.contains_key(lock_target) {
      return self.acquire_read_guard_for_hold(lock_target);
    }

    // Attempt to upgrade the retained lock.
    if let Some(guard) = self.upgrade_for_read_hold(lock_target) {
      return guard;
    }

    // We failed the upgrade. So we'll have to reacquire.
    self.acquire_read_guard_for_hold(lock_target)
  }

  fn acquire_read_guard_for_hold(
    &mut self,
    lock_target: &LockTarget,
  ) -> Rc<RefCell<Guard>> {
    let (lock_mode, guard) = match self.tx_mode {
      TransactionMode::ReadOnly => {
        let guard =
          ReadGuard::acquire_read_guard(&self.btree, lock_target);
        (LockMode::Read, Guard::Read(guard))
      }

      TransactionMode::ReadWrite => {
        let guard =
          WriteGuard::acquire_write_guard(&self.btree, lock_target);
        (LockMode::Write, Guard::Write(guard))
      }
    };

    // Next, wrap it in RefCell so that someone can borrow a guard for
    // mutation. (Though this is a read guard.)
    let guard = RefCell::new(guard);
    // Next, wrap in Rc so that user can hold on to as long as they
    // need.
    let guard = Rc::new(guard);

    // Store a weak version in the map.
    self
      .guards
      .insert(lock_target.clone(), (lock_mode, Rc::downgrade(&guard)));

    guard
  }

  fn upgrade_for_read_hold(
    &mut self,
    lock_target: &LockTarget,
  ) -> Option<Rc<RefCell<Guard>>> {
    let (lock_mode, guard) = &self.guards[lock_target];

    let guard = match guard.upgrade() {
      None => return None,
      Some(guard) => guard,
    };

    if self.tx_mode == TransactionMode::ReadOnly {
      return Some(guard);
    }

    // The problem: what if we acquired a temporary read lock on this
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
