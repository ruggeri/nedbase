use btree::BTree;
use locking::WriteGuard;
use node::InsertionResult;
use std::sync::Arc;
use super::acquire_parent_of_stable_node::acquire_parent_of_stable_node;
use super::acquire_write_guard_path::acquire_write_guard_path;

pub fn optimistic_insert(btree: &Arc<BTree>, insert_key: &str) {
  // Acquire a read lock on the parent of the lowest stable node. Then
  // lock that stable node for writing. And acquire the write path down
  // to the leaf.
  let mut write_guard_path = {
    // Note: parent_of_stable_node might be None if we are splitting the
    // root.
    let parent_of_stable_node = acquire_parent_of_stable_node(btree, insert_key);
    // Note that this will release the read lock on the parent (if any).
    acquire_write_guard_path(btree, parent_of_stable_node, insert_key)
  };

  // Now we perform the insertion at the leaf node. This may trigger a
  // split.
  ::util::log_method_entry("beginning insertion process");
  let mut insertion_result = {
    let mut last_node_write_guard = write_guard_path
      .pop("there should be at least one write node: the leaf to insert into")
      .unwrap_node_write_guard("last write guard should be a node guard");

    last_node_write_guard
      .unwrap_leaf_node_mut_ref("last node on write guard path should be the leaf node")
      .insert(btree, String::from(insert_key))
  };

  // Handle splits for as far up as we need to.
  while let InsertionResult::DidInsertWithSplit(child_split_info) = insertion_result {
    let mut last_write_guard = write_guard_path
      .pop("should not run out of write guards while handling splits");

    match last_write_guard {
      WriteGuard::RootIdentifierWriteGuard(mut identifier_guard) => {
        // We may split all the way to the root.
        ::util::log_method_entry("trying to split root");

        // Create a new root node.
        let new_root_identifier = btree.store_new_root_node(child_split_info);

        // And set this as the new root!
        *identifier_guard = new_root_identifier;

        break
      },

      WriteGuard::NodeWriteGuard(mut node_guard) => {
        insertion_result = node_guard
          .unwrap_interior_node_mut_ref("a split node must have an interior node for a parent")
          .handle_split(btree, child_split_info);
      }
    };
  }

  ::util::log_method_entry("insert completed");
}
