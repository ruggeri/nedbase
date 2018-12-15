use super::{
  LockMode, LockSet, LockSetNodeReadGuard,
  LockSetRootIdentifierReadGuard,
};
use locking::{Guard, LockTarget, ReadGuard};
use std::cell::RefCell;
use std::rc::Rc;

impl LockSet {
  pub fn node_read_guard_for_temp(
    &mut self,
    identifier: &str,
  ) -> LockSetNodeReadGuard {
    let guard = self
      .read_guard_for_temp(&LockTarget::Node(String::from(identifier)));
    LockSetNodeReadGuard::from_guard(guard)
  }

  pub fn root_identifier_read_guard_for_temp(
    &mut self,
  ) -> LockSetRootIdentifierReadGuard {
    let guard = self.read_guard_for_temp(&LockTarget::RootIdentifier);
    LockSetRootIdentifierReadGuard::from_guard(guard)
  }

  fn read_guard_for_temp(
    &mut self,
    lock_target: &LockTarget,
  ) -> Rc<RefCell<Guard>> {
    if !self.guards.contains_key(lock_target) {
      return self.acquire_read_guard_for_temp(lock_target);
    }

    // Attempt to upgrade the retained lock.
    if let Some(guard) = self.upgrade_for_read_temp(lock_target) {
      return guard;
    }

    // We failed the upgrade. So we'll have to reacquire.
    self.acquire_read_guard_for_temp(lock_target)
  }

  fn upgrade_for_read_temp(
    &mut self,
    lock_target: &LockTarget,
  ) -> Option<Rc<RefCell<Guard>>> {
    let (_, guard) = &self.guards[lock_target];
    guard.upgrade()
  }

  fn acquire_read_guard_for_temp(
    &mut self,
    lock_target: &LockTarget,
  ) -> Rc<RefCell<Guard>> {
    let lock_mode = LockMode::Read;
    let guard = ReadGuard::acquire_read_guard(&self.btree, lock_target);
    let guard = Guard::Read(guard);

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

  fn acquire_temp_read_guard(
    &mut self,
    lock_target: &LockTarget,
  ) -> Rc<RefCell<Guard>> {
    let guard = ReadGuard::acquire_read_guard(&self.btree, lock_target);
    let guard = Guard::Read(guard);

    // Next, wrap it in RefCell so that someone can borrow a guard for
    // mutation. (Though this is a read guard.)
    let guard = RefCell::new(guard);
    // Next, wrap in Rc so that user can hold on to as long as they
    // need.
    let guard = Rc::new(guard);

    // Store a weak version in the map.
    self.guards.insert(
      lock_target.clone(),
      (LockMode::Read, Rc::downgrade(&guard)),
    );

    guard
  }
}
