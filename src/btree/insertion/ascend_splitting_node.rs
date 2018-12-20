use super::{
  InsertPathEntry,
  scan_right_for_write_guard,
};
use btree::BTree;
use locking::LockSet;
use node::{InsertionResult, InteriorNode, SplitInfo};

pub enum AscentResult {
  FinishedSplitting,
  RootWasSplit {
    old_root_level_identifier: String,
    split_info: SplitInfo,
  },
}

pub fn ascend_splitting_nodes(
  btree: &BTree,
  lock_set: &mut LockSet,
  mut insert_path: Vec<InsertPathEntry>,
  mut split_info: SplitInfo,
) -> AscentResult {
  let (old_root_node_identifier, old_root_level_identifier) = loop {
    let path_entry = match insert_path.pop() {
      None => panic!("Can't run out of entries without root node..."),
      Some(path_entry) => path_entry,
    };

    match path_entry {
      InsertPathEntry::ParentChild {
        parent_node_identifier,
        ..
      } => {
        let parent_guard = scan_right_for_write_guard(
          lock_set,
          &parent_node_identifier,
          &split_info.new_median,
        );
        let mut parent_node = parent_guard
          .unwrap_interior_node_mut_ref(
            "only interior nodes can be parents",
          );
        match parent_node.handle_split(btree, split_info) {
          InsertionResult::DidInsertWithSplit(new_split_info) => {
            split_info = new_split_info;
            continue;
          }

          _ => {
            return AscentResult::FinishedSplitting;
          }
        }
      }

      InsertPathEntry::RootLevelNode {
        root_node_identifier,
        root_level_identifier,
        ..
      } => {
        break (root_node_identifier, root_level_identifier);
      }
    }
  };

  println!("YOU MADE IT");

  // You split all the way to the top! Wow!
  let root_id_guard = lock_set.root_identifier_write_guard();
  let mut root_identifier = root_id_guard.identifier_mut();
  println!("{}", old_root_node_identifier);
  println!("{}", *root_identifier);
  if old_root_node_identifier != *root_identifier {
    // Root split on us! Uh-oh!
    return AscentResult::RootWasSplit {
      old_root_level_identifier,
      split_info,
    };
  }

  // Thank god! We get to split the root!
  *root_identifier = InteriorNode::store_new_root(
    btree,
    old_root_node_identifier,
    split_info,
  );

  AscentResult::FinishedSplitting
}
