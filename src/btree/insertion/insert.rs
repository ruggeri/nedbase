use super::{
  descend_toward_key, scan_right_for_write_guard, unwind_insert_path,
  DescentDecision,
};
use btree::BTree;
use locking::LockSet;
use node::InsertionResult;
use std::sync::Arc;

pub fn insert(
  btree: &Arc<BTree>,
  lock_set: &mut LockSet,
  key_to_insert: &str,
) {
  // Build a path to the leaf where we should do the inserting.
  let insert_path = descend_toward_key(lock_set, key_to_insert, |_| {
    DescentDecision::ContinueDescending
  });

  // Perform the insert at the leaf node, possibly splitting that leaf.
  let split_info = {
    // Got to the leaf. Now acquire write-style!
    let leaf_entry = insert_path
      .last()
      .expect("lock_path_identifiers must never be empty");
    let leaf_identifier = leaf_entry.current_node_identifier();
    // Must keep in mind that when we acquire the write guard, the
    // target may have split in the meantime.
    let leaf_guard = scan_right_for_write_guard(
      lock_set,
      leaf_identifier,
      key_to_insert,
    );

    // We have the write guard! Let's hold onto it for 2PL since we
    // are updating data stored here.
    lock_set.hold_node_write_guard(&leaf_guard);

    // Perform the insertion.
    let mut leaf_node = leaf_guard
      .unwrap_leaf_node_mut_ref("final node is always LeafNode");
    let insertion_result =
      leaf_node.insert_key(btree, String::from(key_to_insert));

    // If there was no splitting, then there is nothing else to do.
    let mut split_info = match insertion_result {
      InsertionResult::DidInsertWithSplit(split_info) => split_info,
      _ => return,
    };

    // May have to hold the sibbling; since the inserted key could have
    // ended up there. (In that case, it *may* be safe to release the
    // write lock on the node we originally inserted at. However, that
    // lock may already be held for other 2PL inserts previously
    // performed there... Easiest just to hold both locks).
    let sibbling_guard =
      lock_set.node_write_guard(&split_info.new_right_identifier);
    lock_set.hold_node_write_guard(&sibbling_guard);

    split_info
  };

  // Now ascend back up the tree to handle the split of the leaf node.
  // We may have to perform more splits as we move up.
  unwind_insert_path(btree, lock_set, insert_path, split_info);
}
