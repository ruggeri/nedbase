mod descend_toward_key;
mod insert;
mod insert_path;
mod scan_right_for_write_guard;
mod unwinding;

pub(self) use self::descend_toward_key::*;
pub(self) use self::insert_path::*;
pub(self) use self::scan_right_for_write_guard::*;
pub(self) use self::unwinding::*;

use btree::BTree;
use locking::LockSet;
use std::sync::Arc;

impl BTree {
  pub fn insert(
    btree: &Arc<BTree>,
    lock_set: &mut LockSet,
    insert_key: &str,
  ) {
    insert::insert(btree, lock_set, insert_key)
  }
}
