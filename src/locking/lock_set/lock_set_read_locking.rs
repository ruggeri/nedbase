use super::{
  LockSet, LockSetNodeReadGuard, LockSetRootIdentifierReadGuard,
  LockSetValue,
};
use locking::{
  Guard, LockMode, LockTarget, ReadGuard, TransactionMode, WriteGuard,
};
use std::cell::RefCell;
use std::rc::Rc;

// Acquiring a read guard for *holding* (that is, we'll read a value at
// the Node so we need to hold it through the transaction) is the most
// complicated scenario. Depending on what transaction mode we are in
// (ReadWrite), we may actually need to acquire a *write* lock.

impl LockSet {
  pub fn node_read_guard(
    &mut self,
    identifier: &str,
  ) -> LockSetNodeReadGuard {
    // TODO: This String::from seems wasteful just to do a lookup...
    let guard =
      self.read_guard(&LockTarget::Node(String::from(identifier)));
    LockSetNodeReadGuard::from_guard(guard)
  }

  pub fn root_identifier_read_guard(
    &mut self,
  ) -> LockSetRootIdentifierReadGuard {
    let guard = self.read_guard(&LockTarget::RootIdentifier);
    LockSetRootIdentifierReadGuard::from_guard(guard)
  }

  pub fn hold_node_read_guard(
    &mut self,
    node_guard: &LockSetNodeReadGuard,
  ) {
    let strong_ref_cell_guard = node_guard.clone_ref_cell_guard();
    self.held_guards.insert(
      LockTarget::Node(String::from(
        node_guard.unwrap_node_ref().identifier(),
      )),
      strong_ref_cell_guard,
    );
  }

  fn read_guard(
    &mut self,
    lock_target: &LockTarget,
  ) -> Rc<RefCell<Guard>> {
    // If we don't have a copy of this lock, then it's simple: we must
    // acquire it.
    if !self.guards.contains_key(lock_target) {
      return self.acquire_read_guard(lock_target);
    }

    // If we previously acquired this lock, then we should attempt to
    // upgrade the retained lock.
    if let Some(guard) = self.upgrade_for_read(lock_target) {
      return guard;
    }

    // But if we failed the upgrade, we'll have to reacquire after all.
    self.acquire_read_guard(lock_target)
  }

  fn acquire_read_guard(
    &mut self,
    lock_target: &LockTarget,
  ) -> Rc<RefCell<Guard>> {
    // First, acquire the proper guard type. This depends on the
    // transaction mode.
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
    // mutation. This query is asking for a read lock, but someone else
    // in the transaction may want this later for writing. That can
    // happen if we are in ReadWrite mode.
    let guard = RefCell::new(guard);
    // Next, wrap in Rc so that user can hold on to as long as they
    // need.
    let guard = Rc::new(guard);

    // We'll store this in the map for next time. We'll store a weak
    // version so it will get dropped if the user stops using this lock.
    let lock_set_value = LockSetValue {
      lock_mode,
      guard: Rc::downgrade(&guard),
    };
    self.guards.insert(lock_target.clone(), lock_set_value);

    guard
  }

  fn upgrade_for_read(
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

    // If we are in ReadOnly mode, we don't really care if this is a
    // read or a write guard underneath.
    if self.tx_mode == TransactionMode::ReadOnly {
      return Some(guard);
    }

    // But here's a problem: what if we acquired a temporary read lock
    // on this node, and are now forced to take a *write* lock because
    // we are in ReadWrite mode?
    //
    // That would mean we've deadlocked ourself. So we'll panic.
    match lock_mode {
      LockMode::Read => {
        panic!("Tried to acquire a write lock when we already had a temp read guard");
      }

      LockMode::Write => Some(guard),
    }
  }
}
