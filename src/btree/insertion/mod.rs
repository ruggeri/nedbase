mod optimistic_insert;
mod pessimistic_insert;

use btree::BTree;
use std::sync::Arc;

impl BTree {
  pub fn optimistic_insert(btree: &Arc<BTree>, insert_key: &str) {
    optimistic_insert::optimistic_insert(btree, insert_key)
  }

  pub fn pessimistic_insert(btree: &Arc<BTree>, insert_key: &str) {
    pessimistic_insert::pessimistic_insert(btree, insert_key)
  }
}
