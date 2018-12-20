use super::{
  LockSet, LockSetNodeReadGuard, LockSetReadGuard, LockSetRootIdentifierReadGuard,
  LockSetValue,
};
use locking::{Guard, LockMode, LockTarget, ReadGuard};
use std::cell::RefCell;
use std::rc::Rc;

// A temporary ReadGuard is the exception to the rule. We can take read
// guards in ReadWrite mode *if* we don't actually read the value there.
// For instance, temporary guards are okay if we're just taking them to
// descend the tree.
//
// It is important that we never try to take a read or write lock *for
// holding* on a node we currently hold a temporary lock on. Otherwise
// we would deadlock ourselves.

impl LockSet {
  pub fn temp_node_read_guard(
    &mut self,
    identifier: &str,
  ) -> LockSetNodeReadGuard {
    // TODO: This String::from seems wasteful just to do a lookup...
    let guard = self
      ._temp_read_guard(&LockTarget::Node(String::from(identifier)));
    LockSetNodeReadGuard::from_guard(guard)
  }

  pub fn temp_root_identifier_read_guard(
    &mut self,
  ) -> LockSetRootIdentifierReadGuard {
    let guard = self._temp_read_guard(&LockTarget::RootIdentifier);
    LockSetRootIdentifierReadGuard::from_guard(guard)
  }

  fn _temp_read_guard(
    &mut self,
    lock_target: &LockTarget,
  ) -> Rc<RefCell<Guard>> {
    // If we don't have a copy of this lock, then it's simple: we must
    // acquire it.
    if !self.guards.contains_key(lock_target) {
      return self.acquire_temp_read_guard(lock_target);
    }

    // If we previously acquired this lock, then we should attempt to
    // upgrade the retained lock.
    if let Some(guard) = self.upgrade_for_temp_read(lock_target) {
      return guard;
    }

    // But if we failed the upgrade, we'll have to reacquire after all.
    self.acquire_temp_read_guard(lock_target)
  }

  fn upgrade_for_temp_read(
    &mut self,
    lock_target: &LockTarget,
  ) -> Option<Rc<RefCell<Guard>>> {
    // This is a simple scenario: we don't care what mode we're in, and
    // we don't care what kind of lock is held behind the scenes. Any
    // lock will do.
    let LockSetValue { guard, .. } = &self.guards[lock_target];
    guard.upgrade()
  }

  fn acquire_temp_read_guard(
    &mut self,
    lock_target: &LockTarget,
  ) -> Rc<RefCell<Guard>> {
    // First, acquire the read lock. This doesn't depend on the
    // transaction mode!
    let guard = ReadGuard::acquire_read_guard(&self.btree, lock_target);
    let guard = Guard::Read(guard);

    // Next, wrap it in RefCell. No one will want to borrow this lock
    // mutably, but we must conform to the type of LockSetValue.
    let guard = RefCell::new(guard);
    // Next, wrap in Rc so that user can hold on to as long as they
    // need.
    let guard = Rc::new(guard);

    // We'll store this in the map for next time. We'll store a weak
    // version so it will get dropped if the user stops using this lock.
    //
    // Indeed, they *better* eventually drop the lock!
    let lock_set_value = LockSetValue {
      lock_mode: LockMode::Read,
      guard: Rc::downgrade(&guard),
    };
    self.guards.insert(lock_target.clone(), lock_set_value);

    guard
  }
}
