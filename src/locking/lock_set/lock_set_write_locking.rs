use super::{
  LockSet, LockSetNodeWriteGuard, LockSetRootIdentifierWriteGuard,
  LockSetValue, LockSetWriteGuard,
};
use locking::{
  Guard, LockMode, LockTarget, TransactionMode, WriteGuard,
};
use std::cell::RefCell;
use std::rc::Rc;

// Acquiring a write guard (which is always for holding) is probably the
// simplest scenario.

impl LockSet {
  pub fn node_write_guard(
    &mut self,
    identifier: &str,
  ) -> LockSetNodeWriteGuard {
    // TODO: This String::from seems wasteful just to do a lookup...
    let guard =
      self.write_guard(&LockTarget::Node(String::from(identifier)));
    LockSetNodeWriteGuard::from_guard(guard)
  }

  pub fn root_identifier_write_guard(
    &mut self,
  ) -> LockSetRootIdentifierWriteGuard {
    let guard = self.write_guard(&LockTarget::RootIdentifier);
    LockSetRootIdentifierWriteGuard::from_guard(guard)
  }

  pub fn hold_node_write_guard(
    &mut self,
    node_guard: &LockSetNodeWriteGuard,
  ) {
    let strong_ref_cell_guard = node_guard.clone_ref_cell_guard();
    let target = strong_ref_cell_guard.borrow().target();
    self.held_guards.insert(target, strong_ref_cell_guard);
  }

  pub fn hold_write_guard(&mut self, node_guard: &LockSetWriteGuard) {
    let strong_ref_cell_guard = node_guard.clone_ref_cell_guard();
    let target = strong_ref_cell_guard.borrow().target();
    self.held_guards.insert(target, strong_ref_cell_guard);
  }

  fn write_guard(
    &mut self,
    lock_target: &LockTarget,
  ) -> Rc<RefCell<Guard>> {
    // First: you can't get write locks in ReadOnly mode!
    if self.tx_mode == TransactionMode::ReadOnly {
      panic!("cannot acquire read locks in ReadOnly mode!");
    }

    // If we don't have a copy of this lock, then it's simple: we must
    // acquire it.
    if !self.guards.contains_key(lock_target) {
      return self.acquire_write_guard(lock_target);
    }

    // If we previously acquired this lock, then we should attempt to
    // upgrade the retained lock.
    if let Some(guard) = self.upgrade_for_write(lock_target) {
      return guard;
    }

    // But if we failed the upgrade, we'll have to reacquire after all.
    self.acquire_write_guard(lock_target)
  }

  fn acquire_write_guard(
    &mut self,
    lock_target: &LockTarget,
  ) -> Rc<RefCell<Guard>> {
    // Acquire the write guard.
    let guard =
      WriteGuard::acquire_write_guard(&self.btree, lock_target);
    let guard = Guard::Write(guard);

    // Next, wrap it in RefCell so that someone can borrow a guard for
    // mutation.
    let guard = RefCell::new(guard);
    // Next, wrap in Rc so that user can hold on to as long as they
    // need.
    let guard = Rc::new(guard);

    // We'll store this in the map for next time. We'll store a weak
    // version so it will get dropped if the user stops using this lock.
    let lock_set_value = LockSetValue {
      lock_mode: LockMode::Write,
      guard: Rc::downgrade(&guard),
    };
    self.guards.insert(lock_target.clone(), lock_set_value);

    guard
  }

  fn upgrade_for_write(
    &mut self,
    lock_target: &LockTarget,
  ) -> Option<Rc<RefCell<Guard>>> {
    // First, get the weak guard we stored earlier.
    let LockSetValue { lock_mode, guard } = &self.guards[lock_target];

    // If the upgrade fails, then darn.
    let guard = match guard.upgrade() {
      None => return None,
      Some(guard) => guard,
    };

    // But here's a problem: what if we acquired a temporary read lock
    // on this node. That is incompatible with our desire to take a
    // write lock now.
    //
    // That would mean we've deadlocked ourself. So we'll panic.
    match lock_mode {
      LockMode::Read => {
        panic!("Can't hold a write guard on a previously temp guard");
      }

      LockMode::Write => Some(guard),
    }
  }
}
