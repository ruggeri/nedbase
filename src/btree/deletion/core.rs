use super::{acquire_deletion_path, DeletionPathEntry, WriteSet};
use btree::BTree;
use std::sync::Arc;

pub fn delete(btree: &Arc<BTree>, key_to_delete: &str) {
  let (mut deletion_path, mut write_set) =
    acquire_deletion_path(btree, key_to_delete);

  deletion_path
    .last_node_mut(&mut write_set)
    .unwrap_leaf_node_mut_ref("deletion must happen at a leaf node")
    .delete(key_to_delete);

  loop {
    // Stop bubbling if we're not deficient and don't need merging
    // anymore.
    if !deletion_path.last_node(&write_set).is_deficient() {
      break;
    }

    match deletion_path.pop_last_path_entry() {
      DeletionPathEntry::UnstableRootNode { root_identifier } => {
        handle_unstable_root_node(&mut write_set, root_identifier);
        return;
      }

      DeletionPathEntry::TopStableNode { .. } => {
        panic!("TopStableNode is not supposed to go unstable!")
      }

      DeletionPathEntry::NodeWithMergeSibbling { .. } => {
        unimplemented!();
      }
    }
  }
}

fn handle_unstable_root_node(
  write_set: &mut WriteSet,
  root_identifer: String,
) {
  let root_identifier_guard = write_set.get_root_identifier_guard_mut();
  unimplemented!();
}
