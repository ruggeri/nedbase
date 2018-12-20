use super::InsertPathEntry;
use locking::LockSet;
use node::{Node, TraversalDirection};

pub fn descend_to_key<F>(
  lock_set: &mut LockSet,
  key: &str,
  stop_early: F,
) -> Vec<InsertPathEntry>
where
  F: Fn(&Node) -> bool,
{
  let mut insert_path = vec![];

  {
    let root_node_identifier = {
      let root_node_identifier_guard =
        lock_set.temp_root_identifier_read_guard();
      let root_node_identifier_ref =
        root_node_identifier_guard.identifier();
      root_node_identifier_ref.clone()
    };

    let root_node_guard =
      lock_set.temp_node_read_guard(&root_node_identifier);
    let root_node = root_node_guard.unwrap_node_ref();
    let root_level_identifier = match &(*root_node) {
      Node::LeafNode(..) => String::from("LEAF_LEVEL"),
      Node::InteriorNode(inode) => {
        String::from(inode.level_identifier())
      }
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
      let entry = insert_path
        .last()
        .expect("lock_path_identifiers must never be empty");
      let current_identifier = entry.current_node_identifier();
      let current_guard =
        lock_set.temp_node_read_guard(current_identifier);

      (current_guard, current_identifier.clone())
    };

    {
      let node_ref = current_guard.unwrap_node_ref();
      if stop_early(&node_ref) {
        return insert_path;
      }
    }

    let direction =
      { current_guard.unwrap_node_ref().traverse_toward(key) };

    match direction {
      TraversalDirection::Arrived => break,

      TraversalDirection::MoveDown {
        child_node_identifier,
      } => {
        insert_path.push(InsertPathEntry::ParentChild {
          parent_node_identifier: current_identifier,
          child_node_identifier,
        });
      }

      TraversalDirection::MoveRight {
        next_node_identifier,
      } => {
        let mut last_entry = insert_path.last_mut().unwrap();
        match last_entry {
          InsertPathEntry::RootLevelNode {
            current_node_identifier,
            ..
          } => {
            // A noble lie.
            *current_node_identifier = next_node_identifier;
          }

          InsertPathEntry::ParentChild {
            child_node_identifier,
            ..
          } => {
            *child_node_identifier = next_node_identifier;
          }
        }
      }
    }
  }

  insert_path
}
