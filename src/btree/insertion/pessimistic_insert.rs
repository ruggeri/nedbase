use btree::BTree;
use locking::{LockSet, WriteGuard, WriteGuardPath};
use node::{InsertionResult, InteriorNode};
use std::sync::Arc;

pub fn pessimistic_insert(
  btree: &Arc<BTree>,
  lock_set: &mut LockSet,
  insert_key: &str,
) {
  let mut write_guards = WriteGuardPath::new();

  // Acquire write lock on root identifier, and then on the root node.
  {
    let identifier_guard = lock_set.root_identifier_write_guard();
    let current_node_guard = lock_set
      .node_write_guard(&identifier_guard.identifier());

    // If root node won't need to split, we can release the write
    // guard on the root identifier.
    if current_node_guard
      .unwrap_node_ref()
      .can_grow_without_split()
    {
      write_guards.push(current_node_guard.upcast());
    } else {
      write_guards.push(identifier_guard.upcast());
      write_guards.push(current_node_guard.upcast());
    }
  }

  // Now descend, taking write locks hand-over-hand. You can release all
  // prior write locks if you hit a stable node.
  loop {
    let current_node_guard = {
      let prev_node = {
        let deepest_lock = write_guards.peek_deepest_lock(
          "since we break at LeafNode, should not run out of locks",
        );
        deepest_lock.unwrap_node_ref(
          "final write guard in path should always be for a node",
        )
      };

      if prev_node.is_leaf_node() {
        break;
      }

      let node = prev_node.unwrap_interior_node_ref(
        "must not descend through interior node",
      );

      let child_identifier = node.child_identifier_by_key(insert_key);
      lock_set.node_write_guard(child_identifier)
    };

    // Whenever we encounter a stable node, we can clear all previously
    // acquired write locks.
    if current_node_guard
      .unwrap_node_ref()
      .can_grow_without_split()
    {
      write_guards.clear();
    }

    // Regardless, keep holding this lock.
    write_guards.push(current_node_guard.upcast());
  }

  // After descending all the way, perform the insert at the leaf.
  let mut insertion_result = {
    let mut last_guard = write_guards.pop(
      "should have acquired at least one write guard for insertion",
    );

    // For 2PL purposes, we must hold the write guard through the rest
    // of the transaction.
    lock_set.hold_write_guard(&last_guard);

    // Perform the insert.
    let mut last_node =
      last_guard.unwrap_node_mut_ref("should be inserting at a node");
    let insertion_result = last_node
      .unwrap_leaf_node_mut_ref("insertion should happen at leaf node")
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

      // Note: I believe we could give up the write lock on the old
      // leaf. But no one is going to see that anyway, because new
      // descents through the parent (which has a write lock on it
      // presently) will go to the new nodes.
    }

    insertion_result
  };

  // Bubble up. For as long as we are splitting children, insert the
  // split nodes into their parent.
  while let InsertionResult::DidInsertWithSplit(child_split_info) =
    insertion_result
  {
    let mut last_write_guard = write_guards
      .pop("should not run out of write guards while bubbling splits");

    match &mut (*last_write_guard.guard_mut()) {
      // Typical scenario: a child was split. We must update its
      // parent.
      WriteGuard::NodeWriteGuard(ref mut node_guard) => {
        insertion_result = node_guard
          .unwrap_interior_node_mut_ref(
            "parents of split nodes expected to be interior nodes",
          )
          .handle_split(btree, child_split_info);
      }

      // If we split all the way to the top, we have to create a new
      // root node.
      WriteGuard::RootIdentifierWriteGuard(
        ref mut root_identifier_guard,
      ) => {
        // First, create the new root node.
        let new_root_identifier =
          InteriorNode::store_new_root(btree, child_split_info);

        // Now update the BTree to use the new root node we created.
        **root_identifier_guard = new_root_identifier;

        break;
      }
    };
  }
}
