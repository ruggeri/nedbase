use btree::BTree;
use locking::{NodeReadGuard, ReadGuard, ReadGuardPath, RootIdentifierReadGuard};
use node::Node;
use std::sync::Arc;

// Finds highest lock target that may need to be mutated by an
// insertion.
pub fn acquire_parent_of_stable_node(btree: &Arc<BTree>, insert_key: &str) -> Option<ReadGuard> {
  let mut read_guards = ReadGuardPath::new();

  // Acquire read lock on root identifier, and then on the root node.
  {
    let identifier_guard = RootIdentifierReadGuard::acquire(btree);
    let current_node_guard = NodeReadGuard::acquire(btree, &(*identifier_guard));

    read_guards.push(ReadGuard::RootIdentifierReadGuard(identifier_guard));
    read_guards.push(ReadGuard::NodeReadGuard(current_node_guard));
  }

  // Now descend, taking read locks hand-over-hand.
  loop {
    let current_node_guard = {
      let node_read_guard = read_guards
        .peek_deepest_lock()
        .unwrap_node_read_guard_ref("final read guard in path should always be for a node");

      match &(**node_read_guard) {
        Node::LeafNode(_) => break,
        Node::InteriorNode(inode) => {
          let child_identifier = inode.child_identifier_by_key(insert_key);
          NodeReadGuard::acquire(btree, child_identifier)
        }
      }
    };

    // Whenever we encounter a stable node, we can clear all but the
    // last two locks.
    //
    // Why all but the last two? Eventually we will temporarily release
    // the read lock on the target stable node, so that we can try to
    // reacquire a write lock.
    //
    // Holding a read lock on its parent means that the target of the
    // write lock is still where the value should live.
    if current_node_guard.can_grow_without_split() {
      let last_guard = read_guards.pop("should never run out of read locks");
      read_guards.clear();
      read_guards.push(last_guard);
    }

    // Regardless, we will continue to hold this lock.
    read_guards.push(ReadGuard::NodeReadGuard(current_node_guard));
  };

  // By the end, we have >= than two locks. We need at most one lock at
  // the end.
  //
  // * If there is any stable node encountered, we want to hold the
  //   parent (so the stable node doesn't move on us), and reacquire the
  //   write lock on the stable node. Note: the "parent" may be the root
  //   identifier lock.
  //
  // * If there are no stable nodes, we'll have to acquire the root
  //   identifier lock. There is no need to hold any read lock anymore.

  // First, drop all but the top two locks. Unpack those.
  read_guards.truncate(2);
  let node_guard = read_guards
    .pop("expected at least root node guard here...")
    .unwrap_node_read_guard("second lock should never be a root identifier lock");
  let parent_guard = read_guards.pop("should always have had at least two guards");

  if node_guard.can_grow_without_split() {
    Some(parent_guard)
  } else {
    None
  }
}
