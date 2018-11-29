use node::{
  InteriorNode,
  LeafNode,
  Node,
};

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

type IdentifierToNodeArcLockMap = HashMap<String, Arc<RwLock<Node>>>;
pub struct BTree {
  pub root_identifier_lock: RwLock<String>,
  pub identifier_to_node_arc_lock_map: RwLock<IdentifierToNodeArcLockMap>,
  pub max_key_capacity: usize,
}

impl BTree {
  pub fn get_node_arc_lock(&self, identifier: &str) -> Arc<RwLock<Node>> {
    let identifier_to_nodes_map = self.identifier_to_node_arc_lock_map.read().expect("No thread should panic with map lock");
    match identifier_to_nodes_map.get(identifier) {
      Some(node_lock) => Arc::clone(node_lock),
      None => panic!("Eventually should fetch from disk."),
    }
  }

  pub fn store_new_leaf_node(&self, keys: Vec<String>) -> String {
    let identifier = self.get_new_identifier();
    let ln = LeafNode {
      identifier: identifier.clone(),
      keys,
      max_key_capacity: self.max_key_capacity,
    };

    let node = Arc::new(RwLock::new(Node::LeafNode(ln)));
    let mut identifier_to_nodes_map = self.identifier_to_node_arc_lock_map.write().expect("No thread should panic with map lock");
    identifier_to_nodes_map.insert(identifier.clone(), Arc::clone(&node));

    identifier
  }

  pub fn store_new_interior_node(&self, splits: Vec<String>, child_identifiers: Vec<String>) -> String {
    let identifier = self.get_new_identifier();
    let interior_node = InteriorNode {
      identifier: identifier.clone(),
      splits,
      child_identifiers,
      max_key_capacity: self.max_key_capacity,
    };

    let node = Arc::new(RwLock::new(Node::InteriorNode(interior_node)));
    let mut identifier_to_nodes_map = self.identifier_to_node_arc_lock_map.write().expect("No thread should panic with map lock");
    identifier_to_nodes_map.insert(identifier.clone(), Arc::clone(&node));

    identifier
  }

  fn get_new_identifier(&self) -> String {
    unimplemented!()
  }
}
