use btree::BTree;
use node::Node;
use parking_lot::RwLock;
use rand::{distributions::Alphanumeric, prelude::*};
use std::iter;
use std::sync::Arc;

const IDENTIFIER_LENGTH: usize = 8;

// TODO: eventually this should become its own kind of storage class.
impl BTree {
  // Selects an identifier for a node. Relies on being sufficiently
  // random for no collision.
  pub fn get_new_identifier(&self) -> String {
    // Even though this doesn't need to be a method today, it *should*
    // so that we can eliminate the collision possibility.

    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
      .map(|()| rng.sample(Alphanumeric))
      .take(IDENTIFIER_LENGTH)
      .collect();

    chars
  }

  pub fn store_node(&self, node: Node) {
    // First put node in Arc and RwLock.
    let identifier = String::from(node.identifier());
    let node = Arc::new(RwLock::new(node));

    // And then store it.
    self
      .identifier_to_node_arc_lock_map
      .write()
      .insert(identifier, node);
  }
}
