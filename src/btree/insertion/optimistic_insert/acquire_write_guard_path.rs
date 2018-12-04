use btree::BTree;
use locking::{
  NodeWriteGuard,
  ReadGuard,
  RootIdentifierWriteGuard,
  WriteGuard,
  WriteGuardPath
};
use node::Node;
use std::sync::Arc;

pub enum WriteGuardPathAcquisitionResult {
  Success(WriteGuardPath),
  TopNodeWentUnstable,
}

pub fn acquire_write_guard_path(btree: &Arc<BTree>, parent_read_guard: Option<ReadGuard>, insert_key: &str) -> WriteGuardPathAcquisitionResult {
  let mut write_guards = WriteGuardPath::new();

  // Acquire top write guard.
  match parent_read_guard {
    None => {
      // There may be no parent_read_guard because we may have to split
      // all the way through the root.
      let root_identifier_guard = RootIdentifierWriteGuard::acquire(btree);
      let root_node_guard = WriteGuard::acquire_node_write_guard(btree, &(*root_identifier_guard));
      write_guards.push(WriteGuard::RootIdentifierWriteGuard(root_identifier_guard));
      write_guards.push(root_node_guard);
    }

    Some(parent_read_guard) => {
      // If there is a stable node, acquire a write lock on it. We have
      // a read lock on its "parent": either a parent node in the tree,
      // or if the lowest stable node is the root, then at least the
      // root identifier.
      //
      // Note that we take ownership here, so that the read guard will
      // be dropped after the write guard is acquired.
      let deepest_stable_parent = match parent_read_guard {
        ReadGuard::RootIdentifierReadGuard(root_identifier_read_guard) => {
          NodeWriteGuard::acquire(btree, &(*root_identifier_read_guard))
        }

        ReadGuard::NodeReadGuard(parent_node_read_guard) => {
          let interior_node = parent_node_read_guard
            .unwrap_interior_node_ref("a parent node must be an interior node");
          let child_identifier = interior_node.child_identifier_by_key(insert_key);
          NodeWriteGuard::acquire(btree, child_identifier)
        }
      };

      if !deepest_stable_parent.can_grow_without_split() {
        // It is possible that because of insertions, this node is
        // no longer stable! Then we must start all over again!
        return WriteGuardPathAcquisitionResult::TopNodeWentUnstable;
      }

      write_guards.push(WriteGuard::NodeWriteGuard(deepest_stable_parent));
    }
  }

  // Descend acquiring write guards.
  loop {
    let child_guard = {
      let last_node_guard = write_guards
        .peek_deepest_lock()
        .unwrap_node_write_guard_ref("we should have acquired a node write guard here");

      match &(**last_node_guard) {
        Node::LeafNode(_) => break,
        Node::InteriorNode(inode) => {
          let child_identifier = inode.child_identifier_by_key(insert_key);
          WriteGuard::acquire_node_write_guard(btree, child_identifier)
        }
      }
    };

    // TODO:: If the node we just locked is stable we can release the
    // prior locks. In that case we were just slightly overly
    // pessimistic.
    write_guards.push(child_guard);
  }

  WriteGuardPathAcquisitionResult::Success(write_guards)
}
