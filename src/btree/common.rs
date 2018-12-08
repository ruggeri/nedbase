use node::{LeafNode, Node};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

type IdentifierToNodeArcLockMap = HashMap<String, Arc<RwLock<Node>>>;
pub struct BTree {
  pub(in btree) root_identifier_lock: RwLock<String>,
  pub(in btree) identifier_to_node_arc_lock_map:
    RwLock<IdentifierToNodeArcLockMap>,
  pub(in btree) max_key_capacity: usize,
}

impl BTree {
  pub fn new(max_key_capacity: usize) -> BTree {
    let root_identifier: String = BTree::get_new_identifier();
    let root_node = Node::LeafNode(LeafNode::new(
      root_identifier.clone(),
      vec![],
      max_key_capacity,
    ));

    let mut identifier_to_node_arc_lock_map = HashMap::new();
    identifier_to_node_arc_lock_map.insert(
      root_identifier.clone(),
      Arc::new(RwLock::new(root_node)),
    );

    BTree {
      root_identifier_lock: RwLock::new(root_identifier.clone()),
      identifier_to_node_arc_lock_map: RwLock::new(
        identifier_to_node_arc_lock_map,
      ),
      max_key_capacity,
    }
  }

  pub fn root_identifier_lock(&self) -> &RwLock<String> {
    &self.root_identifier_lock
  }

  pub fn get_node_arc_lock(
    &self,
    identifier: &str,
  ) -> Arc<RwLock<Node>> {
    let node_lock = {
      ::util::log_node_map_locking(
        "trying to acquire read lock of node map",
      );
      let identifier_to_nodes_map =
        self.identifier_to_node_arc_lock_map.read();
      ::util::log_node_map_locking("acquired read lock of node map");

      let node_lock_option = identifier_to_nodes_map.get(identifier);

      match node_lock_option {
        Some(node_lock) => Arc::clone(node_lock),
        None => panic!("Eventually should fetch from disk."),
      }
    };
    ::util::log_node_map_locking("released read lock of node map");

    node_lock
  }
}
