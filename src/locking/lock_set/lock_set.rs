use super::LockSetValue;
use btree::BTree;
use locking::{Guard, LockTarget, TransactionMode};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

// The LockSet manages all the locks for a transaction. It's important
// job is that, if within a single transaction, query Q1 wants some
// locks that were acquried by Q2, then the LockSet prevents Q1 from
// trying to re-acquire those held locks, since otherwise this results
// in deadlock.
//
// When in ReadOnly mode, the LockSet will only acquire read locks. When
// in ReadWrite mode, the LockSet will mostly acquire write locks. The
// reason to acquire write locks, even when reading in ReadWrite mode,
// is because of the possibility that we will have to *reacquire*.
//
// By this I mean: If Q1 reads node N, we must hold the lock until the
// end of the transaction. If Q2 then wants to write node N, it cannot
// safely acquire this lock because there is already a read lock here...
//
// However, there is an exception. If a lock is "temporary", we may
// acquire a ReadLock on it even in ReadWrite mode. These must be locks
// where we aren't reading a value; we're just descending through them.

pub struct LockSet {
  pub(super) btree: Arc<BTree>,
  pub(super) guards: HashMap<LockTarget, LockSetValue>,
  pub(super) held_guards: HashMap<LockTarget, Rc<RefCell<Guard>>>,
  pub(super) tx_mode: TransactionMode,
}

impl LockSet {
  pub fn new(btree: &Arc<BTree>, tx_mode: TransactionMode) -> LockSet {
    LockSet {
      btree: Arc::clone(btree),
      guards: HashMap::new(),
      held_guards: HashMap::new(),
      tx_mode,
    }
  }

  // TODO: Super hacky way to hold onto held locks for 2PL.
  pub fn freeze_held_guards(&mut self) {
    for (key, value) in self.guards.iter() {
      let guard = match value.guard.upgrade() {
        None => continue,
        Some(guard) => guard,
      };

      self.held_guards.insert(key.clone(), guard);
    }
  }
}
