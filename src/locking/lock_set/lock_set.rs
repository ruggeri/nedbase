use super::{LockSetNodeReadGuard, LockSetRootIdentifierReadGuard};
use locking::{Guard, LockTarget, TransactionMode};
use btree::BTree;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::Arc;

pub struct LockSet {
  btree: Arc<BTree>,
  guards: HashMap<LockTarget, Weak<RefCell<Guard>>>,
  tx_mode: TransactionMode
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

  pub fn node_read_guard(&mut self, identifier: &str) -> LockSetNodeReadGuard {
    let guard = self.acquire_read_guard(&LockTarget::Node(String::from(identifier)));
    let unwrap_msg = "Using LockTarget::Node should ensure a Node guard is acquired";
    LockSetNodeReadGuard::from_guard(guard, unwrap_msg)
  }

  pub fn root_identifier_read_guard(&mut self) -> LockSetRootIdentifierReadGuard {
    let guard = self.acquire_read_guard(&LockTarget::RootIdentifier);
    let unwrap_msg = "Using LockTarget::RootIdentifier should ensure a RootIdentifier guard is acquired";
    LockSetRootIdentifierReadGuard::from_guard(guard, unwrap_msg)
  }

  // This depends on what mode we are in.
  fn acquire_read_guard(&mut self, lock_target: &LockTarget) -> Rc<RefCell<Guard>> {
    let guard = Guard::acquire_guard(&self.btree, self.tx_mode, lock_target);

    // Next, wrap it in RefCell so that someone can borrow a guard for
    // mutation. (Though this is a read guard.)
    let guard = RefCell::new(guard);
    // Next, wrap in Rc so that user can hold on to as long as they
    // need.
    let guard = Rc::new(guard);

    // Store a weak version in the map.
    self.guards.insert(lock_target.clone(), Rc::downgrade(&guard));

    guard
  }
}
