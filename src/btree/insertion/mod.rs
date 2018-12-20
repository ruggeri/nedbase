// mod optimistic_insert;
mod pessimistic_insert;

use btree::BTree;
use locking::LockSet;
use std::sync::Arc;

impl BTree {
  // pub fn optimistic_insert(
  //   btree: &Arc<BTree>,
  //   lock_set: &mut LockSet,
  //   insert_key: &str,
  // ) {
  //   optimistic_insert::optimistic_insert(btree, lock_set, insert_key)
  // }

  pub fn pessimistic_insert(
    btree: &Arc<BTree>,
    lock_set: &mut LockSet,
    insert_key: &str,
  ) {
    pessimistic_insert::pessimistic_insert(btree, lock_set, insert_key)
  }
}
