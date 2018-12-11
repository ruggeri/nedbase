use super::acquire_write_guard_path;
use btree::BTree;
use locking::WriteGuard;
use node::InsertionResult;
use std::sync::Arc;

pub fn optimistic_insert(btree: &Arc<BTree>, insert_key: &str) {
  // Acquire a read lock on the parent of the lowest stable node. Then
  // lock that stable node for writing. And acquire the write path down
  // to the leaf.
  let mut write_guard_path =
    acquire_write_guard_path(btree, insert_key);

  // Now we perform the insertion at the leaf node. This may trigger a
  // split.
  let mut insertion_result = {
    write_guard_path
      .pop("there should be at least one write node: the leaf to insert into")
      .unwrap_node_write_guard("last write guard should be a node guard")
      .unwrap_leaf_node_mut_ref(
        "last node on write guard path should be the leaf node",
      )
      .insert(btree, String::from(insert_key))
  };

  // Handle splits for as far up as we need to.
  while let InsertionResult::DidInsertWithSplit(child_split_info) =
    insertion_result
  {
    let mut last_write_guard = write_guard_path
      .pop("should not run out of write guards while handling splits");

    match last_write_guard {
      WriteGuard::NodeWriteGuard(mut node_guard) => {
        insertion_result = node_guard
          .unwrap_interior_node_mut_ref(
            "a split node must have an interior node for a parent",
          )
          .handle_split(btree, child_split_info);
      }

      WriteGuard::RootIdentifierWriteGuard(mut identifier_guard) => {
        // We may split all the way to the root. First, create a new
        // root node.
        let new_root_identifier =
          btree.store_new_root_node(child_split_info);

        // Next, set this as the new root!
        *identifier_guard = new_root_identifier;

        break;
      }
    };
  }
}
