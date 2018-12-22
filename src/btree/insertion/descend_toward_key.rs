use super::InsertPathEntry;
use locking::LockSet;
use node::{Node, TraversalDirection};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum DescentDecision {
  ContinueDescending,
  StopEarly,
}

// Descends toward the key. We may stop early if instructed.
pub fn descend_toward_key<F>(
  lock_set: &mut LockSet,
  key: &str,
  stop_early: F,
) -> Vec<InsertPathEntry>
where
  F: Fn(&Node) -> DescentDecision,
{
  let mut insert_path = vec![];

  // Start the path off at the alleged root.
  {
    // First get the root's identifier.
    let root_node_identifier = {
      let root_node_identifier_guard =
        lock_set.temp_root_identifier_read_guard();
      let root_node_identifier_ref =
        root_node_identifier_guard.identifier();
      root_node_identifier_ref.clone()
    };

    insert_path.push(InsertPathEntry::RootLevelNode {
      alleged_root_identifier: root_node_identifier,
    });
  }

  loop {
    // First get the current identifier/guard.
    let (current_identifier, current_guard) = {
      let entry = insert_path
        .last()
        .expect("insert path must never be empty as we descend");
      let current_identifier = entry.current_node_identifier().clone();
      let current_guard =
        lock_set.temp_node_read_guard(&current_identifier);

      (current_identifier, current_guard)
    };

    // Unwrap node.
    let node_ref = current_guard.unwrap_node_ref();

    // Let them stop early if they feel they have descended far enough.
    if stop_early(&node_ref) == DescentDecision::StopEarly {
      return insert_path;
    }

    // Decide which direction to move in.
    let direction = node_ref.traverse_toward(key);

    match direction {
      // We made it all the way to the bottom! Rejoice!
      TraversalDirection::Arrived => return insert_path,

      // Move down toward the leaves.
      TraversalDirection::MoveDown {
        child_node_identifier,
      } => {
        insert_path.push(InsertPathEntry::ParentChild {
          parent_node_identifier: current_identifier,
          current_node_identifier: String::from(child_node_identifier),
        });
      }

      // We may have to move right if we missed a split. In that case,
      // we update the existing entry along the path.
      TraversalDirection::MoveRight {
        next_node_identifier,
      } => {
        let mut last_entry = insert_path.last_mut().unwrap();
        last_entry.update_current_node_identifier(String::from(
          next_node_identifier,
        ));
      }
    }
  }
}
