use super::{
  AscentResult,
  ascend_splitting_nodes,
  descend_to_key,
  scan_right_for_write_guard,
};
use btree::BTree;
use locking::LockSet;
use node::{InsertionResult, Node};
use std::sync::Arc;

pub fn insert(
  btree: &Arc<BTree>,
  lock_set: &mut LockSet,
  key_to_insert: &str,
) {
  let mut insert_path =
    descend_to_key(lock_set, key_to_insert, |_| false);

  let mut split_info = {
    // Got to the leaf. Now acquire write-style!
    let leaf_entry = insert_path
      .last()
      .expect("lock_path_identifiers must never be empty");
    let leaf_identifier = leaf_entry.current_node_identifier();
    let leaf_guard =
      scan_right_for_write_guard(lock_set, leaf_identifier, key_to_insert);
    // We have the write guard! Let's hold onto it since we mutate it.
    lock_set.hold_node_write_guard(&leaf_guard);

    let mut leaf_node = leaf_guard
      .unwrap_leaf_node_mut_ref("final node is always LeafNode");
    let insertion_result =
      leaf_node.insert_key(btree, String::from(key_to_insert));

    // Perform the insertion
    let mut split_info = match insertion_result {
      InsertionResult::DidInsertWithSplit(split_info) => split_info,
      _ => return,
    };

    // May have to hold the sibbling; since the key could have ended up
    // there.
    let sibbling_guard =
      lock_set.node_write_guard(&split_info.new_right_identifier);
    lock_set.hold_node_write_guard(&sibbling_guard);

    split_info
  };

  loop {
    let ascent_result =
      ascend_splitting_nodes(btree, lock_set, insert_path, split_info);
    match ascent_result {
      AscentResult::FinishedSplitting => return,
      AscentResult::RootWasSplit {
        old_root_level_identifier,
        split_info: new_split_info,
      } => {
        insert_path = descend_to_key(
          lock_set,
          &new_split_info.new_median,
          |node_ref| {
            println!("{}", old_root_level_identifier);
            match node_ref {
              Node::LeafNode(..) => false,
              Node::InteriorNode(inode) => {
                println!("{}", inode.level_identifier());
                inode.level_identifier() == old_root_level_identifier
              }
            }
          },
        );
        split_info = new_split_info;
      }
    }
  }
}
