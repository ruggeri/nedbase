use btree::BTree;
use node::{InteriorNode, LeafNode, Node, SplitInfo};
use parking_lot::RwLock;
use rand::{distributions::Alphanumeric, prelude::*};
use std::iter;
use std::sync::Arc;

const IDENTIFIER_LENGTH: usize = 8;

// TODO: Everything here is backward! There should only be a store_node
// method. Everything else should be handled elsewhere!
impl BTree {
  // Selects an identifier for a node. Relies on being sufficiently
  // random for no collision.
  pub fn get_new_identifier() -> String {
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
      .map(|()| rng.sample(Alphanumeric))
      .take(IDENTIFIER_LENGTH)
      .collect();

    chars
  }

  // Creates and stores a new InteriorNode.
  pub fn store_new_interior_node(
    &self,
    splits: Vec<String>,
    child_identifiers: Vec<String>,
  ) -> String {
    let identifier = BTree::get_new_identifier();

    // Create the node.
    let node = InteriorNode::new(
      identifier.clone(),
      splits,
      child_identifiers,
      self.max_key_capacity,
    );

    // Upcast and put it into Arc.
    let node = Arc::new(RwLock::new(node.upcast()));

    // Store the node in the map.
    self.store_node(identifier.clone(), node);

    identifier
  }

  pub fn store_new_leaf_node(&self, keys: Vec<String>) -> String {
    // Choose an identifier.
    let identifier = BTree::get_new_identifier();

    // Create the node.
    let node =
      LeafNode::new(identifier.clone(), keys, self.max_key_capacity);

    // Upcast and put it into Arc.
    let node = Arc::new(RwLock::new(node.upcast()));

    // Store the node in the map.
    self.store_node(identifier.clone(), node);

    identifier
  }

  pub fn store_new_root_node(&self, split_info: SplitInfo) -> String {
    // Choose an identifier.
    let identifier = BTree::get_new_identifier();

    // Create the node.
    let node = InteriorNode::new_root_from_split_info(
      identifier.clone(),
      split_info,
      self.max_key_capacity,
    );

    // Upcast and put it into Arc.
    let node = Arc::new(RwLock::new(node.upcast()));

    // Store the node.
    self.store_node(identifier.clone(), node);

    identifier
  }

  fn store_node(&self, identifier: String, node: Arc<RwLock<Node>>) {
    self
      .identifier_to_node_arc_lock_map
      .write()
      .insert(identifier, node);
  }
}
