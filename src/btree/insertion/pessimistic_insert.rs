use btree::BTree;
use locking::{
  NodeWriteGuard,
  RootIdentifierWriteGuard,
  WriteGuard,
  WriteGuardPath,
};
use node::{InsertionResult, Node};
use std::sync::Arc;

pub fn pessimistic_insert(btree: &Arc<BTree>, insert_key: &str) {
  let mut write_guards = WriteGuardPath::new();

  // Acquire write lock on root identifier, and then on the root node.
  {
    let identifier_guard = RootIdentifierWriteGuard::acquire(btree);
    let current_node_guard = NodeWriteGuard::acquire(btree, &(*identifier_guard));

    // If root node won't need to split, we can release the write
    // guard on the root identifier.
    if current_node_guard.can_grow_without_split() {
      write_guards.push(WriteGuard::NodeWriteGuard(current_node_guard));
    } else {
      write_guards.push(WriteGuard::RootIdentifierWriteGuard(identifier_guard));
      write_guards.push(WriteGuard::NodeWriteGuard(current_node_guard));
    }
  }

  // Now descend, taking write locks hand-over-hand. You can release all
  // prior write locks if you hit a stable node.
  loop {
    let current_node_guard = {
      let node_write_guard = write_guards
        .peek_deepest_lock()
        .unwrap_node_write_guard_ref("final write guard in path should always be for a node");

      match &(**node_write_guard) {
        Node::LeafNode(_) => break,
        Node::InteriorNode(inode) => {
          let child_identifier = inode.child_identifier_by_key(insert_key);
          NodeWriteGuard::acquire(btree, child_identifier)
        }
      }
    };

    // Whenever we encounter a stable node, we can clear all previously
    // acquired write locks.
    if current_node_guard.can_grow_without_split() {
      write_guards.clear();
    }

    // Regardless, hold this lock.
    write_guards.push(WriteGuard::NodeWriteGuard(current_node_guard));
  };

  // Perform the insert at the leaf.
  let mut insertion_result = write_guards
    .pop("should have acquired at least one write guard for insertion")
    .unwrap_node_write_guard("should be inserting at a node")
    .unwrap_leaf_node_mut_ref("insertion should happen at leaf node")
    .insert(btree, String::from(insert_key));

  // For as long as we are splitting, insert the split nodes into their
  // parent.
  while let InsertionResult::DidInsertWithSplit(child_split_info) = insertion_result {
    let mut last_write_guard = write_guards
      .pop("should not run out of write guards while bubbling splits");

    match last_write_guard {
      WriteGuard::RootIdentifierWriteGuard(mut root_identifier_guard) => {
        // We have split all the way to the top!

        // First, create the new root node.
        let new_root_identifier = BTree::store_new_root_node(
          btree,
          child_split_info,
        );

        // Now update the BTree to use the new root node we created.
        *root_identifier_guard = new_root_identifier;

        break
      },

      WriteGuard::NodeWriteGuard(mut node_guard) => {
        insertion_result = node_guard
          .unwrap_interior_node_mut_ref("parents of split nodes expected to be interior nodes")
          .handle_split(btree, child_split_info);
      }
    };
  }
}
