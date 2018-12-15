use super::LockMode;
use btree::BTree;
use locking::{Guard, LockTarget, TransactionMode};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;
use std::sync::Arc;

pub struct LockSet {
  pub(super) btree: Arc<BTree>,
  pub(super) guards:
    HashMap<LockTarget, (LockMode, Weak<RefCell<Guard>>)>,
  pub(super) tx_mode: TransactionMode,
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
}
