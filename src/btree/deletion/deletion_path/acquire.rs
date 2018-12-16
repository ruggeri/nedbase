use super::{DeletionPath, DeletionPathBuilder};
use btree::deletion::acquire_parent_of_deepest_stable_node;
use locking::LockSet;

// Acquires a DeletionPath. The DeletionPath is the series of actions to
// take to perform the delete.
pub fn acquire_deletion_path(
  lock_set: &mut LockSet,
  key_to_delete: &str,
) -> DeletionPath {
  // Start building the path. Starting a builder acquires the first
  // write lock.
  let mut builder = loop {
    let builder_option =
      new_deletion_path_builder(lock_set, key_to_delete);

    // If the deepest node stayed stable, then the builder will be
    // constructed. We can begin to descend.
    if builder_option.is_some() {
      break builder_option.unwrap();
    }

    // The deepest stable node may go unstable due to simultaneous
    // deletions. In that case, we must try everything again, starting
    // from the root.
  };

  // We descend, taking write locks all the way down.
  while !builder.hit_leaf_node() {
    builder.extend_deletion_path(lock_set, key_to_delete);
  }

  // And now we pull out the DeletionPath as the builder's job is done.
  builder.finish(key_to_delete)
}

// Creates a builder and starts the deletion path by acquiring the
// topmost write guard.
#[allow(clippy::let_and_return)]
fn new_deletion_path_builder(
  lock_set: &mut LockSet,
  key_to_delete: &str,
) -> Option<DeletionPathBuilder> {
  // The first step is to acquire a read guard of the parent above our
  // target stable node.
  let parent_guard = match acquire_parent_of_deepest_stable_node(
    lock_set,
    key_to_delete,
  ) {
    None => {
      // However, the root itself may be unstable for deletion. In that
      // case, we must acquire a write lock on the root identifier.
      return Some(DeletionPathBuilder::new_from_unstable_root(
        lock_set,
      ));
    }

    // Normally, there is *some* stable node. We unwrap its parent.
    Some(parent_guard) => parent_guard,
  };

  // If there was a stable node, we need to start our
  // DeletionPathBuilder off by locking it.
  let builder_option = match parent_guard.downcast() {
    // If the deepest stable node is the root, then parent_guard will be
    // a read guard on the root_identifier. We want the builder to write
    // lock the root node.
    (Some(root_identifier), None) => {
      DeletionPathBuilder::new_from_stable_parent(
        lock_set,
        &root_identifier,
      )
    }

    // Typically, the deepest stable node is not the root. In which
    // case, we have read locked the node's parent. The builder will
    // take a write lock on the child, and we release the read lock on
    // the parent (because it goes out of scope).
    (None, Some(parent_node)) => {
      let child_identifier = parent_node
        .unwrap_interior_node_ref("a parent must be an interior node")
        .child_identifier_by_key(key_to_delete);

      DeletionPathBuilder::new_from_stable_parent(
        lock_set,
        child_identifier,
      )
    }

    _ => {
      panic!("Every guard must be for a RootIdentifier or a Node...")
    }
  };

  // For some reason if I return directly from the match I have a
  // lifetime problem...
  builder_option
}
