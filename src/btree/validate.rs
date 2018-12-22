use super::BTree;
use locking::LockSet;
use node::{InteriorNode, LeafNode, MaxValue, Node};

// This code tests that the BTree obeys all constraints.

const LOW_LIMIT: &str = "00000000";
const HIGH_LIMIT: &str = "zzzzzzzz";

impl BTree {
  pub fn validate(&self, lock_set: &mut LockSet) {
    let current_node_identifier = {
      let guard = lock_set.temp_root_identifier_read_guard();
      let guard_identifier = guard.identifier();
      guard_identifier.clone()
    };

    // Checking starts at the root.
    check_node(
      lock_set,
      &current_node_identifier,
      LOW_LIMIT,
      HIGH_LIMIT,
    );
  }
}

fn check_node(
  lock_set: &mut LockSet,
  node_identifier: &str,
  low_limit: &str,
  high_limit: &str,
) {
  let child_guard = lock_set.temp_node_read_guard(node_identifier);
  let child_node_ref = child_guard.unwrap_node_ref();

  match &(*child_node_ref) {
    Node::InteriorNode(inode) => {
      check_interior_node(lock_set, inode, low_limit, high_limit);
    }

    Node::LeafNode(lnode) => {
      check_leaf_node(lnode, low_limit, high_limit);
    }
  }
}

fn check_leaf_node(node: &LeafNode, low_limit: &str, high_limit: &str) {
  // All keys must be greater than the low limit.
  let mut prev_key = String::from(low_limit);
  for key in node.keys() {
    // Keys must be in ascending order (with no duplicates).
    if key.as_str() <= prev_key.as_str() {
      println!("{}", key);
      println!("{}", prev_key);
      panic!("Keys are out of order!");
    }

    // All values must be less than or equal to the high limit.
    if high_limit < key.as_str() {
      println!("{}", key);
      println!("{}", high_limit);
      panic!("High limit disobeyed");
    }

    prev_key = key.clone();
  }
}

fn check_interior_node(
  lock_set: &mut LockSet,
  node: &InteriorNode,
  low_limit: &str,
  high_limit: &str,
) {
  // All keys must be greater than the low limit.
  let mut prev_split_value = String::from(low_limit);

  // Choose the lower of the high limit (imposed by parent) or the max
  // value expressed by the node (which really should == the high limit
  // from the parent I think).
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
    // Keys must be in ascending order (with no duplicates).
    if split_value.as_str() <= prev_split_value.as_str() {
      println!("{}", split_value);
      println!("{}", prev_split_value);
      panic!("Keys are out of order!");
    }

    // All values must be less than or equal to the high limit.
    if high_limit.as_str() < split_value.as_str() {
      println!("{}", split_value);
      println!("{}", high_limit);
      panic!("High limit disobeyed");
    }

    // Recursively check each child node.
    let child_identifier = node.child_identifier_by_idx(idx);
    check_node(
      lock_set,
      child_identifier,
      &prev_split_value,
      &split_value,
    );

    prev_split_value = split_value.clone();
  }

  // There's one last child we didn't check!
  let child_identifier =
    node.child_identifier_by_idx(node.num_split_keys());
  check_node(
    lock_set,
    child_identifier,
    &prev_split_value,
    &high_limit,
  );
}
