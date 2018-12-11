use super::{
  acquire_deletion_path, DeletionPathEntry, UnderflowAction, WriteSet,
};
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

    // TODO: Ugly!
    match underflow_action {
      UnderflowAction::MergeWithSibbling {
        parent_node_identifier,
        sibbling_node_identifier,
      } => MergeOperation {
        parent_node_identifier,
        path_node_identifier,
        sibbling_node_identifier,
      }
      .execute(btree, &mut write_set),

      UnderflowAction::UpdateRootIdentifier => {
        UpdateRootIdentifierOperation {
          path_node_identifier,
        }
        .execute(&mut write_set)
      }
    }
  }
}

// Helper struct that performs the merge operation.
struct MergeOperation {
  parent_node_identifier: String,
  path_node_identifier: String,
  sibbling_node_identifier: String,
}

impl MergeOperation {
  fn execute(self, btree: &BTree, write_set: &mut WriteSet) {
    // Get the write locks you've acquired on everyone.
    let parent_node = write_set
      .get_node_mut_ref(&self.parent_node_identifier)
      .unwrap_interior_node_mut_ref("parents must be InteriorNodes");

    let path_node =
      write_set.get_node_mut_ref(&self.path_node_identifier);

    let sibbling_node_identifier =
      write_set.get_node_mut_ref(&self.sibbling_node_identifier);

    // And then have the parent perform the merge or rotation.
    parent_node.merge_or_rotate_children(
      btree,
      path_node,
      sibbling_node_identifier,
    );
  }
}

struct UpdateRootIdentifierOperation {
  path_node_identifier: String,
}

impl UpdateRootIdentifierOperation {
  fn execute(self, write_set: &mut WriteSet) {
    let new_root_identifier = {
      // First, get the root_node.
      let root_node =
        write_set.get_node_ref(&self.path_node_identifier);

      // Next, if the root node is a leaf, there is nothing to do.
      if root_node.is_leaf_node() {
        return;
      }

      // Okay, so the root is an InteriorNode...
      let root_node = root_node.unwrap_interior_node_ref(
        "can't reduce depth if root is already LeafNode",
      );

      // We will only "consume" the root and decrease the height of
      // the BTree when the root has a *single child*.
      if root_node.num_children() > 1 {
        return;
      }

      // Okay! We do want to pull up the root. The new root will be the
      // only child of the root node. So we get its identifier.
      String::from(root_node.child_identifier_by_idx(0))
    };

    // Now set the root identifier to finish the compaction.
    let root_identifier_guard =
      write_set.get_root_identifier_guard_mut_ref();
    **root_identifier_guard = new_root_identifier;

    return;
  }
}
