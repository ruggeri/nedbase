use super::{acquire_deletion_path, DeletionPathEntry};
use btree::BTree;
use std::sync::Arc;

pub fn delete(btree: &Arc<BTree>, key_to_delete: &str) {
  // Acquire locks.
  let (mut deletion_path, mut write_set) =
    acquire_deletion_path(btree, key_to_delete);

  // Perform the delete at the LeafNode.
  deletion_path
    .last_node_mut_ref(&mut write_set)
    .unwrap_leaf_node_mut_ref("deletion must happen at a leaf node")
    .delete(key_to_delete);

  loop {
    // Stop bubbling if we're not deficient and don't need merging
    // anymore.
    if !deletion_path.last_node_ref(&write_set).is_deficient() {
      break;
    }

    // Unwrap the action we must take for this deficient node.
    let underflow_action = match deletion_path.pop_last_path_entry() {
      DeletionPathEntry::TopStableNode { .. } => {
        panic!("TopStableNode is not supposed to go unstable!")
      }

      DeletionPathEntry::UnstableNode { underflow_action } => {
        underflow_action
      }
    };

    underflow_action.execute(btree, &mut write_set);
  }
}
