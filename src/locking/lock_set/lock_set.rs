use super::{
  LockSetNodeReadGuard, LockSetNodeWriteGuard,
  LockSetRootIdentifierReadGuard, LockSetRootIdentifierWriteGuard,
};
use btree::BTree;
use locking::{
  Guard, LockTarget, ReadGuard, TransactionMode, WriteGuard,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::Arc;

enum LockMode {
  Read,
  Write,
}

pub struct LockSet {
  btree: Arc<BTree>,
  guards: HashMap<LockTarget, (LockMode, Weak<RefCell<Guard>>)>,
  tx_mode: TransactionMode,
}

impl LockSet {
  pub fn new_read_lock_set(btree: &Arc<BTree>) -> LockSet {
    LockSet {
      btree: Arc::clone(btree),
      guards: HashMap::new(),
      tx_mode: TransactionMode::ReadOnly,
    }
  }

  pub fn new_write_lock_set(btree: &Arc<BTree>) -> LockSet {
    LockSet {
      btree: Arc::clone(btree),
      guards: HashMap::new(),
      tx_mode: TransactionMode::ReadWrite,
    }
  }

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

    // Store a weak version in the map.
    self.guards.insert(
      lock_target.clone(),
      (LockMode::Write, Rc::downgrade(&guard)),
    );

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

  fn upgrade_for_read_temp(
    &mut self,
    lock_target: &LockTarget,
  ) -> Option<Rc<RefCell<Guard>>> {
    let (_, guard) = &self.guards[lock_target];
    guard.upgrade()
  }

  fn upgrade_for_write_hold(
    &mut self,
    lock_target: &LockTarget,
  ) -> Option<Rc<RefCell<Guard>>> {
    let (lock_mode, guard) = &self.guards[lock_target];

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
