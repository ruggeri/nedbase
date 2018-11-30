use btree::BTree;
use node::{
  InteriorNode,
  LeafNode,
  Node,
};
use parking_lot::RwLock;
use rand::{
  distributions::Alphanumeric,
  prelude::*
};
use std::iter;
use std::sync::Arc;

const IDENTIFIER_LENGTH: usize = 8;

impl BTree {
  fn store_node(&self, identifier: String, node: Arc<RwLock<Node>>) {
    ::util::log_node_map_locking("trying to acquire write lock of node map");
    let mut identifier_to_nodes_map = self.identifier_to_node_arc_lock_map.write();
    ::util::log_node_map_locking("acquired write lock of node map");
    identifier_to_nodes_map.insert(String::from(identifier), node);
    ::util::log_node_map_locking("released write lock of node map");
  }

  pub fn store_new_leaf_node(&self, keys: Vec<String>) -> String {
    // Choose an identifier.
    let identifier = BTree::get_new_identifier();

    // Create the node and put it into Arc.
    let node = LeafNode::new(identifier.clone(), keys, self.max_key_capacity);
    let node = Arc::new(RwLock::new(Node::LeafNode(node)));

    // Store the node.
    self.store_node(identifier.clone(), node);

    identifier
  }

  pub fn store_new_interior_node(&self, splits: Vec<String>, child_identifiers: Vec<String>) -> String {
    let identifier = BTree::get_new_identifier();
    let interior_node = InteriorNode {
      identifier: identifier.clone(),
      splits,
      child_identifiers,
      max_key_capacity: self.max_key_capacity,
    };

    let node = Arc::new(RwLock::new(Node::InteriorNode(interior_node)));

    {
      ::util::log_node_map_locking("trying to acquire write lock of node map");
      let mut identifier_to_nodes_map = self.identifier_to_node_arc_lock_map.write();
      ::util::log_node_map_locking("acquired write lock of node map");
      identifier_to_nodes_map.insert(identifier.clone(), Arc::clone(&node));
    }
    ::util::log_node_map_locking("released write lock of node map");

    identifier
  }

  pub fn get_new_identifier() -> String {
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(IDENTIFIER_LENGTH)
        .collect();

    chars
  }
}
