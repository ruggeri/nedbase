mod acquire_deletion_path;
mod acquire_parent_of_stable_node;
mod core;
mod deletion_path;
mod write_set;

use btree::BTree;
use std::sync::Arc;

impl BTree {
  pub fn delete(btree: &Arc<BTree>, insert_key: &str) {
    core::delete(btree, insert_key)
  }
}
