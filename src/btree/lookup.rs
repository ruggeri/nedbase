use btree::BTree;
use locking::{LockSet, LockSetNodeReadGuard};
use node::TraversalDirection;

impl BTree {
  pub fn contains_key(lock_set: &mut LockSet, key: &str) -> bool {
    let guard = BTree::find_leaf_for_key(lock_set, key);
    let node = guard
      .unwrap_leaf_node_ref("find_leaf_for_key must return leaf node");

    node.contains_key(key)
  }

  pub fn find_leaf_for_key(
    lock_set: &mut LockSet,
    key: &str,
  ) -> LockSetNodeReadGuard {
    let mut current_identifier = {
      let root_identifier_guard = lock_set.temp_root_identifier_read_guard();
      let root_identifier = root_identifier_guard.identifier();
      root_identifier.clone()
    };

    loop {
      let guard = lock_set.temp_node_read_guard(&current_identifier);
      let direction = {
        guard.unwrap_node_ref().traverse_toward(key)
      };

      match direction {
        TraversalDirection::Arrived => break,
        TraversalDirection::MoveDown { child_node_identifier } => {
          current_identifier = child_node_identifier;
        }
        TraversalDirection::MoveRight { next_node_identifier } => {
          current_identifier = next_node_identifier;
        }
      }
    }

    loop {
      let guard = lock_set.node_read_guard(&current_identifier);
      let direction = {
        guard.unwrap_node_ref().traverse_toward(key)
      };

      match direction {
        TraversalDirection::Arrived => {
          lock_set.hold_node_read_guard(&guard);
          return guard;
        },

        TraversalDirection::MoveDown { child_node_identifier } => {
          current_identifier = child_node_identifier;
        }

        TraversalDirection::MoveRight { next_node_identifier } => {
          current_identifier = next_node_identifier;
        }
      }
    }
  }
}
