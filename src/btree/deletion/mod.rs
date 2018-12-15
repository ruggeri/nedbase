mod acquire_parent_of_deepest_stable_node;
mod core;
mod deletion_path;
mod underflow_actions;

// Methods/classes to share amongst this submodule.
pub(self) use self::acquire_parent_of_deepest_stable_node::acquire_parent_of_deepest_stable_node;
pub(self) use self::deletion_path::{
  acquire_deletion_path, DeletionPathEntry,
};
pub(self) use self::underflow_actions::{
  UnderflowAction, UnderflowActionResult,
};

// Needed for the BTree#delete method.
use btree::BTree;
use locking::LockSet;
use std::sync::Arc;

impl BTree {
  pub fn delete(
    btree: &Arc<BTree>,
    lock_set: &mut LockSet,
    insert_key: &str,
  ) {
    core::delete(btree, lock_set, insert_key)
  }
}
