mod acquire_parent_of_stable_node;
mod acquire_write_set;
mod core;

use btree::BTree;
use std::sync::Arc;

impl BTree {
  pub fn delete(btree: &Arc<BTree>, insert_key: &str) {
    core::delete(btree, insert_key)
  }
}
