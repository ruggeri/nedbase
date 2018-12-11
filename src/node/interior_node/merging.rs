use super::InteriorNode;
use btree::BTree;
use node::Node;

impl InteriorNode {
  pub fn merge_or_rotate_children(
    &mut self,
    btree: &BTree,
    node1: &mut Node,
    node2: &mut Node,
  ) {
    // Sort out which node is left and which is right.
    let idx1 = self
      .child_identifiers
      .iter()
      .position(|identifier| identifier == node1.identifier())
      .expect("node1 is supposed to be a child...");

    let idx2 = self
      .child_identifiers
      .iter()
      .position(|identifier| identifier == node2.identifier())
      .expect("node2 is supposed to be a child...");

    let (left_idx, left_node, right_node) = if idx1 < idx2 {
      (idx1, node1, node2)
    } else {
      (idx2, node2, node1)
    };

    // Now perform the merging/rotating.
    Node::merge_or_rotate_sibblings(
      btree, self, left_node, right_node, left_idx,
    );
  }

  pub fn handle_child_merge(
    &mut self,
    left_idx: usize,
    merged_node_identifier: String,
  ) {
    // left_idx is what split the merged nodes from each other.
    self.splits.remove(left_idx);
    // Think of "merging in" from right to left. We remove the
    // right identifier.
    self.child_identifiers.remove(left_idx + 1);
    // And update the left one.
    self.child_identifiers[left_idx] = merged_node_identifier;
  }

  pub fn handle_leaf_child_rotate(
    &mut self,
    left_idx: usize,
    new_split_key: String,
  ) {
    self.splits[left_idx] = new_split_key;
  }

  pub fn merge_sibblings(
    btree: &BTree,
    parent_node: &mut InteriorNode,
    left_node: &InteriorNode,
    right_node: &InteriorNode,
    left_idx: usize,
  ) {
    // Merge splits
    let mut splits = left_node.splits.clone();
    splits.extend(right_node.splits.iter().cloned());

    // Merge child_identifiers
    let mut child_identifiers = left_node.child_identifiers.clone();
    child_identifiers
      .extend(right_node.child_identifiers.iter().cloned());

    // Create the new node.
    let merged_node_identifier =
      InteriorNode::store(btree, splits, child_identifiers);

    parent_node.handle_child_merge(left_idx, merged_node_identifier);
  }

  pub fn rotate_left_from_sibbling(
    parent_node: &mut InteriorNode,
    left_node: &mut InteriorNode,
    right_node: &mut InteriorNode,
    left_idx: usize,
  ) {
    assert!(left_node.num_split_keys() < right_node.num_split_keys());

    // Giving two names for the same quantity for clarity.
    let num_split_keys_to_move =
      (right_node.num_split_keys() - left_node.num_split_keys()) / 2;
    let num_children_to_move = num_split_keys_to_move;

    // drain is hella fancy. It removes from right_node.keys as it
    // copies to left_node.keys.
    let drain = right_node.splits.drain(..num_split_keys_to_move);
    left_node.splits.extend(drain);

    // Likewise for the children.
    let drain =
      right_node.child_identifiers.drain(..num_children_to_move);
    left_node.child_identifiers.extend(drain);

    // This effectively rotates the first of right keys up, and the
    // split key in the parent to the left.
    ::std::mem::swap(
      &mut parent_node.splits[left_idx],
      left_node
        .splits
        .last_mut()
        .expect("every InteriorNode must have at least one split"),
    );
  }

  pub fn rotate_right_from_sibbling(
    parent_node: &mut InteriorNode,
    left_node: &mut InteriorNode,
    right_node: &mut InteriorNode,
    left_idx: usize,
  ) {
    assert!(right_node.num_split_keys() < left_node.num_split_keys());

    // Giving two names for the same quantity for clarity.
    let num_split_keys_to_move =
      (left_node.num_split_keys() - right_node.num_split_keys()) / 2;
    let num_children_to_move = num_split_keys_to_move;

    // Rotating right is a little more annoying...
    let drain_start_idx =
      left_node.num_split_keys() - num_split_keys_to_move;
    let mut new_right_splits: Vec<_> =
      left_node.splits.drain(drain_start_idx..).collect();
    new_right_splits.append(&mut right_node.splits);
    right_node.splits = new_right_splits;

    // Likewise for the children.
    let drain_start_idx =
      left_node.num_children() - num_children_to_move;
    let mut new_right_child_identifiers: Vec<_> = left_node
      .child_identifiers
      .drain(drain_start_idx..)
      .collect();
    new_right_child_identifiers
      .append(&mut right_node.child_identifiers);
    right_node.child_identifiers = new_right_child_identifiers;

    // This effectively rotates the first of left keys up, and the
    // split key in the parent to the left.
    ::std::mem::swap(
      &mut parent_node.splits[left_idx],
      left_node
        .splits
        .first_mut()
        .expect("every InteriorNode must have at least one split"),
    );
  }
}
