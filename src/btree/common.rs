use node::{
  InteriorNode,
  LeafNode,
  Node,
};

use rand::{
  distributions::Alphanumeric,
  prelude::*
};
use std::collections::HashMap;
use std::iter;
use parking_lot::RwLock;
use std::sync::{Arc};

type IdentifierToNodeArcLockMap = HashMap<String, Arc<RwLock<Node>>>;
pub struct BTree {
  pub root_identifier_lock: RwLock<String>,
  pub identifier_to_node_arc_lock_map: RwLock<IdentifierToNodeArcLockMap>,
  pub max_key_capacity: usize,
}

impl BTree {
  pub fn new(max_key_capacity: usize) -> BTree {
    let root_identifier: String = BTree::get_new_identifier();
    let root_node = Node::LeafNode(LeafNode {
      identifier: root_identifier.clone(),
      keys: vec![],
      max_key_capacity,
    });

    let mut identifier_to_node_arc_lock_map = HashMap::new();
    identifier_to_node_arc_lock_map.insert(root_identifier.clone(), Arc::new(RwLock::new(root_node)));

    BTree {
      root_identifier_lock: RwLock::new(root_identifier.clone()),
      identifier_to_node_arc_lock_map: RwLock::new(identifier_to_node_arc_lock_map),
      max_key_capacity,
    }
  }


  pub fn get_node_arc_lock(&self, identifier: &str) -> Arc<RwLock<Node>> {
    let identifier_to_nodes_map = self.identifier_to_node_arc_lock_map.read();
    match identifier_to_nodes_map.get(identifier) {
      Some(node_lock) => Arc::clone(node_lock),
      None => panic!("Eventually should fetch from disk."),
    }
  }

  pub fn store_new_leaf_node(&self, keys: Vec<String>) -> String {
    let identifier = BTree::get_new_identifier();
    let ln = LeafNode {
      identifier: identifier.clone(),
      keys,
      max_key_capacity: self.max_key_capacity,
    };

    let node = Arc::new(RwLock::new(Node::LeafNode(ln)));
    let mut identifier_to_nodes_map = self.identifier_to_node_arc_lock_map.write();
    identifier_to_nodes_map.insert(identifier.clone(), Arc::clone(&node));

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
    let mut identifier_to_nodes_map = self.identifier_to_node_arc_lock_map.write();
    identifier_to_nodes_map.insert(identifier.clone(), Arc::clone(&node));

    identifier
  }

  pub fn get_new_identifier() -> String {
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(16)
        .collect();

    chars
  }
}
