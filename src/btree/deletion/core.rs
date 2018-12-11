use super::{
  acquire_deletion_path, DeletionPathEntry, UnderflowActionResult,
};
use btree::BTree;
use std::sync::Arc;

pub fn delete(btree: &Arc<BTree>, key_to_delete: &str) {
  // Acquire locks.
  let (mut deletion_path, mut write_set) =
    acquire_deletion_path(btree, key_to_delete);

  // Perform the delete at the LeafNode.
  {
    let leaf_node = deletion_path
      .last_node_mut_ref(&mut write_set)
      .unwrap_leaf_node_mut_ref("deletion must happen at a leaf node");

    leaf_node.delete(key_to_delete);

    // Can avoid any bubblingn if leaf node never goes deficient.
    if !leaf_node.is_deficient() {
      return;
    }
  }

  loop {
    // Unwrap the action we must take for this deficient node.
    let underflow_action = match deletion_path.pop_last_path_entry() {
      DeletionPathEntry::TopStableNode { .. } => {
        panic!("TopStableNode is not supposed to go unstable!")
      }

      DeletionPathEntry::UnstableNode { underflow_action } => {
        underflow_action
      }
    };

    // Execute the action.
    let result = underflow_action.execute(btree, &mut write_set);

    // Action may have us stop if we hit a stable parent or consume the
    // root.
    if let UnderflowActionResult::StopBubbling = result {
      return;
    }
  }
}
