use super::acquire_write_set::{
  acquire_write_set
};
use btree::BTree;
use std::sync::Arc;

pub fn delete(btree: &Arc<BTree>, key_to_delete: &str) {
  let mut write_set = acquire_write_set(btree, key_to_delete);

  write_set
    .current_node_mut()
    .unwrap_leaf_node_mut_ref("deletion must happen at leaf node");

  loop {
    // TODO: Implement me!
    return
  }
}
