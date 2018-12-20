use btree::BTree;
use locking::{LockSet, LockSetNodeWriteGuard};
use node::{InsertionResult, InteriorNode, Node, SplitInfo, TraversalDirection};
use std::sync::Arc;

enum InsertPathEntry {
  ParentChild {
    parent_node_identifier: String,
    child_node_identifier: String,
  },

  RootLevelNode {
    root_node_identifier: String,
    root_level_identifier: String,
    // This can be different if we walk right from the root.
    current_node_identifier: String,
  }
}

impl InsertPathEntry {
  pub fn current_node_identifier(&self) -> &String {
    use self::InsertPathEntry::*;

    match self {
      RootLevelNode { ref current_node_identifier, .. } => current_node_identifier,
      ParentChild { ref child_node_identifier, .. } => child_node_identifier,
    }
  }
}

pub fn pessimistic_insert(
  btree: &Arc<BTree>,
  lock_set: &mut LockSet,
  key_to_insert: &str,
) {
  let mut insert_path = descend_to_leaf_with_key(lock_set, key_to_insert, |_| false);

  let mut split_info = {
    // Got to the leaf. Now acquire write-style!
    let leaf_entry = insert_path.last().expect("lock_path_identifiers must never be empty");
    let leaf_identifier = leaf_entry.current_node_identifier();
    let leaf_guard = scan_right_for_write(lock_set, leaf_identifier, key_to_insert);
    // We have the write guard! Let's hold onto it since we mutate it.
    lock_set.hold_node_write_guard(&leaf_guard);

    let mut leaf_node = leaf_guard
      .unwrap_leaf_node_mut_ref("final node is always LeafNode");
    let insertion_result = leaf_node.insert_key(btree, String::from(key_to_insert));

    // Perform the insertion
    let mut split_info = match insertion_result {
      InsertionResult::DidInsertWithSplit(split_info) => split_info,
      _ => return,
    };

    // May have to hold the sibbling; since the key could have ended up
    // there.
    let sibbling_guard = lock_set.node_write_guard(&split_info.new_right_identifier);
    lock_set.hold_node_write_guard(&sibbling_guard);

    split_info
  };

  loop {
    let ascent_result = ascend_splitting_nodes(btree, lock_set, insert_path, split_info);
    match ascent_result {
      AscentResult::FinishedSplitting => return,
      AscentResult::RootWasSplit { old_root_level_identifier, split_info: new_split_info } => {
        insert_path = descend_to_leaf_with_key(lock_set, &new_split_info.new_median, |node_ref| {
          println!("{}", old_root_level_identifier);
          match node_ref {
            Node::LeafNode(..) => false,
            Node::InteriorNode(inode) => {
              println!("{}", inode.level_identifier());
              inode.level_identifier() == old_root_level_identifier
            }
          }
        });
        split_info = new_split_info;
      }
    }
  }
}

fn descend_to_leaf_with_key<F>(lock_set: &mut LockSet, key: &str, stop_early: F) -> Vec<InsertPathEntry>
  where F: Fn(&Node) -> bool {
  let mut insert_path = vec![];

  {
    let root_node_identifier = {
      let root_node_identifier_guard = lock_set.temp_root_identifier_read_guard();
      let root_node_identifier_ref = root_node_identifier_guard.identifier();
      root_node_identifier_ref.clone()
    };

    let root_node_guard = lock_set.temp_node_read_guard(&root_node_identifier);
    let root_node = root_node_guard.unwrap_node_ref();
    let root_level_identifier = match &(*root_node) {
      Node::LeafNode(..) => String::from("LEAF_LEVEL"),
      Node::InteriorNode(inode) => String::from(inode.level_identifier()),
    };
    let current_node_identifier = root_node_identifier.clone();
    insert_path.push(InsertPathEntry::RootLevelNode {
      root_node_identifier,
      root_level_identifier,
      current_node_identifier,
    });
  }

  loop {
    let (current_guard, current_identifier) = {
      let entry = insert_path.last().expect("lock_path_identifiers must never be empty");
      let current_identifier = entry.current_node_identifier();
      let current_guard = lock_set.temp_node_read_guard(current_identifier);

      (current_guard, current_identifier.clone())
    };

    {
      let node_ref = current_guard.unwrap_node_ref();
      if stop_early(&node_ref) {
        return insert_path;
      }
    }

    let direction = {
      current_guard.unwrap_node_ref().traverse_toward(key)
    };

    match direction {
      TraversalDirection::Arrived => break,

      TraversalDirection::MoveDown { child_node_identifier } => {
        insert_path.push(InsertPathEntry::ParentChild {
          parent_node_identifier: current_identifier,
          child_node_identifier,
        });
      }

      TraversalDirection::MoveRight { next_node_identifier } => {
        let mut last_entry = insert_path.last_mut().unwrap();
        match last_entry {
          InsertPathEntry::RootLevelNode { current_node_identifier, .. } => {
            // A noble lie.
            *current_node_identifier = next_node_identifier;
          }

          InsertPathEntry::ParentChild { child_node_identifier, ..} => {
            *child_node_identifier = next_node_identifier;
          }
        }
      }
    }
  }

  insert_path
}

fn scan_right_for_write(lock_set: &mut LockSet, start_identifier: &str, key: &str) -> LockSetNodeWriteGuard {
  let mut current_identifier = String::from(start_identifier);
  loop {
    let mut current_guard = lock_set.node_write_guard(&current_identifier);
    let direction = current_guard.unwrap_node_ref().traverse_toward(key);

    match direction {
      TraversalDirection::Arrived => {
        return current_guard;
      }
      TraversalDirection::MoveDown { .. } => {
        // We are only moving right.
        return current_guard;
      },
      TraversalDirection::MoveRight { next_node_identifier } => {
        current_identifier = next_node_identifier;
      }
    }
  }
}

enum AscentResult {
  FinishedSplitting,
  RootWasSplit {
    old_root_level_identifier: String,
    split_info: SplitInfo,
  },
}

fn ascend_splitting_nodes(
  btree: &BTree,
  lock_set: &mut LockSet,
  mut insert_path: Vec<InsertPathEntry>,
  mut split_info: SplitInfo) -> AscentResult {

  let (old_root_node_identifier, old_root_level_identifier) = loop {
    let path_entry = match insert_path.pop() {
      None => panic!("Can't run out of entries without root node..."),
      Some(path_entry) => path_entry,
    };

    match path_entry {
      InsertPathEntry::ParentChild { parent_node_identifier, .. } => {
        let parent_guard = scan_right_for_write(lock_set, &parent_node_identifier, &split_info.new_median);
        let mut parent_node = parent_guard.unwrap_interior_node_mut_ref("only interior nodes can be parents");
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

      InsertPathEntry::RootLevelNode { root_node_identifier, root_level_identifier, .. } => {
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
    }
  }

  // Thank god! We get to split the root!
  *root_identifier = InteriorNode::store_new_root(
    btree,
    old_root_node_identifier,
    split_info,
  );

  AscentResult::FinishedSplitting
}
