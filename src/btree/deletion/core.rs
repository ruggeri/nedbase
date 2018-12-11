use super::{
  acquire_deletion_path, DeletionPathEntry, UnderflowAction, WriteSet,
};
use btree::BTree;
use std::sync::Arc;

pub fn delete(btree: &Arc<BTree>, key_to_delete: &str) {
  let (mut deletion_path, mut write_set) =
    acquire_deletion_path(btree, key_to_delete);

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
    let (underflow_action, path_node_identifier) =
      match deletion_path.pop_last_path_entry() {
        DeletionPathEntry::TopStableNode { .. } => {
          panic!("TopStableNode is not supposed to go unstable!")
        }

        DeletionPathEntry::UnstableNode {
          underflow_action,
          path_node_identifier,
        } => (underflow_action, path_node_identifier),
      };

    match underflow_action {
      UnderflowAction::UpdateRootIdentifier => {
        let new_root_identifier = {
          let root_node = write_set.get_node_ref(&path_node_identifier);

          if root_node.is_leaf_node() {
            // Nothing to do with an underfull root leaf. We are done!
            return;
          }

          let root_node = root_node.unwrap_interior_node_ref(
            "can't reduce depth if root is already LeafNode",
          );

          // We will only "consume" the root and decrease the height of
          // the BTree when the root has a single child.
          if root_node.num_children() > 1 {
            return;
          }

          String::from(root_node.child_identifier_by_idx(0))
        };

        let root_identifier_guard =
          write_set.get_root_identifier_guard_mut_ref();
        **root_identifier_guard = new_root_identifier;
        return;
      }

      UnderflowAction::MergeWithSibbling {
        parent_node_identifier,
        sibbling_node_identifier,
      } => MergeOperation {
        parent_node_identifier,
        path_node_identifier,
        sibbling_node_identifier,
      }
      .execute(btree, &mut write_set),
    }
  }
}

// Basically a helper just so that Strings are passed in right order (by
// name).
struct MergeOperation {
  parent_node_identifier: String,
  path_node_identifier: String,
  sibbling_node_identifier: String,
}

impl MergeOperation {
  fn execute(self, btree: &BTree, write_set: &mut WriteSet) {
    let parent_node = write_set
      .get_node_mut_ref(&self.parent_node_identifier)
      .unwrap_interior_node_mut_ref("parents must be InteriorNodes");

    let path_node = write_set.get_node_ref(&self.path_node_identifier);

    let sibbling_node_identifier =
      write_set.get_node_ref(&self.sibbling_node_identifier);

    parent_node.merge_children(
      btree,
      path_node,
      sibbling_node_identifier,
    );
  }
}
