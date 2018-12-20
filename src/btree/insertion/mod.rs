mod ascend_splitting_node;
mod descend_to_key;
mod insert;
mod insert_path;
mod scan_right_for_write_guard;

pub(in self) use self::ascend_splitting_node::*;
pub(in self) use self::descend_to_key::*;
pub(in self) use self::insert_path::*;
pub(in self) use self::scan_right_for_write_guard::*;

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
