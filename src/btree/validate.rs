use super::BTree;
use locking::LockSet;
use node::{InteriorNode, LeafNode, MaxValue, Node};

impl BTree {
  pub fn check_leaf_node(node: &LeafNode, low_limit: &str, high_limit: &str) {
    // println!("{:?}", node);

    // prolly prev enough.
    let mut prev_key = String::from(low_limit);
    for key in node.keys() {
      if key.as_str() <= prev_key.as_str() {
        println!("{}", key);
        println!("{}", prev_key);
        panic!("Keys are out of order!");
      }
      if key.as_str() <= low_limit {
        println!("{}", key);
        println!("{}", low_limit);
        panic!("Low limit disobeyed");
      }
      if high_limit < key.as_str() {
        println!("{}", key);
        println!("{}", high_limit);
        panic!("High limit disobeyed");
      }

      prev_key = key.clone();
    }
  }

  pub fn check_interior_node(lock_set: &mut LockSet, node: &InteriorNode, low_limit: &str, high_limit: &str) {
    // println!("{:?}", node);

    // prolly prev enough.
    let mut prev_split_value = String::from(low_limit);

    let high_limit = match node.max_value() {
      MaxValue::Infinity => String::from(high_limit),
      MaxValue::DefiniteValue(max_value) => {
        if max_value.as_str() < high_limit {
          max_value.clone()
        } else {
          String::from(high_limit)
        }
      }
    };

    for (idx, split_value) in node.splits().iter().enumerate() {
      if split_value.as_str() <= prev_split_value.as_str() {
        println!("{}", split_value);
        println!("{}", prev_split_value);
        panic!("Keys are out of order!");
      }
      if split_value.as_str() <= low_limit {
        println!("{}", split_value);
        println!("{}", low_limit);
        panic!("Low limit disobeyed");
      }
      if high_limit.as_str() < split_value.as_str() {
        println!("{}", split_value);
        println!("{}", high_limit);
        panic!("High limit disobeyed");
      }

      let child_identifier = node.child_identifier_by_idx(idx);
      BTree::check_node(lock_set, child_identifier, &prev_split_value, &split_value);

      prev_split_value = split_value.clone();
    }

    let child_identifier = node.child_identifier_by_idx(node.num_split_keys());
    BTree::check_node(lock_set, child_identifier, &prev_split_value, &high_limit);
  }


  pub fn check_node(lock_set: &mut LockSet, node_identifier: &str, low_limit: &str, high_limit: &str) {
    let child_guard = lock_set.temp_node_read_guard(node_identifier);
    let child_node_ref = child_guard.unwrap_node_ref();

    match &(*child_node_ref) {
      Node::InteriorNode(inode) => {
        BTree::check_interior_node(
          lock_set,
          inode,
          low_limit,
          high_limit
        );
      }

      Node::LeafNode(lnode) => {
        BTree::check_leaf_node(
          lnode,
          low_limit,
          high_limit
        );
      }
    }
  }

  pub fn validate(&self, lock_set: &mut LockSet) {
    let current_node_identifier = {
      let guard = lock_set.temp_root_identifier_read_guard();
      let guard_identifier = guard.identifier();
      guard_identifier.clone()
    };

    BTree::check_node(lock_set, &current_node_identifier, "000000", "zzzzzzzzz");
  }
}
