use node::{LeafNode, Node};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

type IdentifierToNodeArcLockMap = HashMap<String, Arc<RwLock<Node>>>;
pub struct BTree {
  // Keeps track of which node is the root node.
  pub(in btree) root_identifier_lock: RwLock<String>,
  // Associates node identifiers with the node.
  pub(in btree) identifier_to_node_arc_lock_map:
    RwLock<IdentifierToNodeArcLockMap>,
  // Used when creating new nodes.
  pub(in btree) max_key_capacity: usize,
}

impl BTree {
  pub fn new(max_key_capacity: usize) -> BTree {
    // First we make a BTree with a bogus root.
    let btree = BTree {
      // Default root identifier is "" which is bogus.
      root_identifier_lock: RwLock::default(),
      identifier_to_node_arc_lock_map: RwLock::default(),
      max_key_capacity,
    };

    // Then we do create an empty leaf node for the root.
    let root_identifier = LeafNode::store(&btree, vec![]);

    // Then we store this for the root node identifier.
    *(btree.root_identifier_lock.write()) = root_identifier;

    btree
  }

  pub fn get_node_arc_lock(
    &self,
    identifier: &str,
  ) -> Arc<RwLock<Node>> {
    let identifier_to_nodes_map =
      self.identifier_to_node_arc_lock_map.read();

    let node_lock_option = identifier_to_nodes_map.get(identifier);

    match node_lock_option {
      Some(node_lock) => Arc::clone(node_lock),
      None => panic!("Eventually should fetch from disk."),
    }
  }

  pub fn max_key_capacity(&self) -> usize {
    self.max_key_capacity
  }

  pub fn root_identifier_lock(&self) -> &RwLock<String> {
    &self.root_identifier_lock
  }
}
