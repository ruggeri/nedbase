use super::acquire_deletion_path::acquire_deletion_path;
use btree::BTree;
use std::sync::Arc;

pub fn delete(btree: &Arc<BTree>, key_to_delete: &str) {
  let (deletion_path, mut write_set) =
    acquire_deletion_path(btree, key_to_delete);

  deletion_path
    .last_node_mut(&mut write_set)
    .unwrap_leaf_node_mut_ref("deletion must happen at a leaf node")
    .delete(key_to_delete);

  // TODO: right now no merging is happening!!

  // loop {
  //   let last_path_entry = write_set
  //     .pop_last_path_entry("path must not run out prematurely");
  //   match last_path_entry {
  //     DeletionPathEntry::TopStableNode { node_identifier } => {
  //       handle_top_stable_node(write_set, &node_identifier);
  //       break;
  //     }
  //   }

  //   // TODO: Implement me!
  //   return;
  // }
}
