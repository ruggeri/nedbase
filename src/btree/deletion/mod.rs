mod acquire_deletion_path;
mod acquire_parent_of_stable_node;
mod core;
mod deletion_path;
mod write_set;

use btree::BTree;
use std::sync::Arc;

pub(self) use self::acquire_deletion_path::acquire_deletion_path;
pub(self) use self::acquire_parent_of_stable_node::acquire_parent_of_stable_node;
pub(self) use self::deletion_path::{
  DeletionPath, DeletionPathEntry, UnderflowAction,
};
pub(self) use self::write_set::WriteSet;

impl BTree {
  pub fn delete(btree: &Arc<BTree>, insert_key: &str) {
    core::delete(btree, insert_key)
  }
}
