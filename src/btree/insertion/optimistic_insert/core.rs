use super::acquire_write_guard_path;
use btree::BTree;
use locking::{LockSet, WriteGuard};
use node::{InsertionResult, InteriorNode};
use std::sync::Arc;

pub fn optimistic_insert(
  btree: &Arc<BTree>,
  lock_set: &mut LockSet,
  insert_key: &str,
) {
  // Acquire a read lock on the parent of the lowest stable node. Then
  // lock that stable node for writing. And acquire the write path down
  // to the leaf.
  let mut write_guard_path =
    acquire_write_guard_path(lock_set, insert_key);

  // Now we perform the insertion at the leaf node. This may trigger a
  // split.
  let mut insertion_result = {
    let mut last_guard = write_guard_path
      .pop("there should be at least one write node: the leaf to insert into");

    // For 2PL purposes, we must hold the write guard through the rest
    // of the transaction.
    lock_set.hold_write_guard(&last_guard);

    // Perform the insert.
    let mut last_node = last_guard
      .unwrap_node_mut_ref("last write guard should be a node guard");
    let insertion_result = last_node
      .unwrap_leaf_node_mut_ref(
        "last node on write guard path should be the leaf node",
      )
      .insert(btree, String::from(insert_key));

    if let InsertionResult::DidInsertWithSplit(ref child_split_info) =
      insertion_result
    {
      // If we have split new leaves, we want to keep and hold locks on
      // them for 2PL purposes.
      let left_split_guard = lock_set.node_write_guard(&child_split_info.new_left_identifier);
      let right_split_guard = lock_set.node_write_guard(&child_split_info.new_right_identifier);
      lock_set.hold_node_write_guard(&left_split_guard);
      lock_set.hold_node_write_guard(&right_split_guard);
    }

    insertion_result
  };

  // Handle splits for as far up as we need to.
  while let InsertionResult::DidInsertWithSplit(child_split_info) =
    insertion_result
  {
    let mut last_write_guard = write_guard_path
      .pop("should not run out of write guards while handling splits");

    match &mut (*last_write_guard.guard_mut()) {
      WriteGuard::NodeWriteGuard(ref mut node_guard) => {
        insertion_result = node_guard
          .unwrap_interior_node_mut_ref(
            "a split node must have an interior node for a parent",
          )
          .handle_split(btree, child_split_info);
      }

      WriteGuard::RootIdentifierWriteGuard(
        ref mut identifier_guard,
      ) => {
        // We may split all the way to the root. First, create a new
        // root node.
        let new_root_identifier =
          InteriorNode::store_new_root(btree, child_split_info);

        // Next, set this as the new root!
        **identifier_guard = new_root_identifier;

        break;
      }
    };
  }
}
