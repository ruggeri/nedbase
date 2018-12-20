use btree::BTree;
use node::{InteriorNode, LeafNode, MergeOrRotateResult};

impl LeafNode {
  pub fn merge_sibblings(
    btree: &BTree,
    parent_node: &mut InteriorNode,
    left_node: &mut LeafNode,
    right_node: &mut LeafNode,
    left_idx: usize,
  ) -> MergeOrRotateResult {
    unimplemented!();
    // let mut keys = left_node.keys.clone();
    // keys.extend(right_node.keys.iter().cloned());

    // let merge_node_identifier = LeafNode::store(btree, keys);

    // parent_node.handle_child_merge(left_idx, merge_node_identifier.clone());

    // MergeOrRotateResult::DidMerge {
    //   merge_node_identifier
    // }
  }

  pub fn rotate_left_from_sibbling(
    parent_node: &mut InteriorNode,
    left_node: &mut LeafNode,
    right_node: &mut LeafNode,
    left_idx: usize,
  ) -> MergeOrRotateResult {
    unimplemented!();
    // assert!(left_node.num_keys() < right_node.num_keys());

    // let num_keys_to_move =
    //   (right_node.num_keys() - left_node.num_keys()) / 2;

    // // drain is hella fancy. It removes from right_node.keys as it
    // // copies to left_node.keys.
    // let drain = right_node.keys.drain(..num_keys_to_move);
    // left_node.keys.extend(drain);

    // let new_split_key = left_node
    //   .keys
    //   .last()
    //   .expect("just moved at least one element")
    //   .clone();

    // parent_node.handle_leaf_child_rotate(left_idx, new_split_key);

    // MergeOrRotateResult::DidRotate
  }

  pub fn rotate_right_from_sibbling(
    parent_node: &mut InteriorNode,
    left_node: &mut LeafNode,
    right_node: &mut LeafNode,
    left_idx: usize,
  ) -> MergeOrRotateResult {
    unimplemented!();
    // assert!(right_node.num_keys() < left_node.num_keys());

    // let num_keys_to_move =
    //   (left_node.num_keys() - right_node.num_keys()) / 2;

    // // First, take the last `num_keys_to_move` from left_node.keys.
    // let drain_start_idx = left_node.num_keys() - num_keys_to_move;
    // let mut new_right_keys: Vec<_> =
    //   left_node.keys.drain(drain_start_idx..).collect();

    // // Then add on all the right keys. This clears out
    // // `right_node.keys`.
    // new_right_keys.append(&mut right_node.keys);

    // // Then replace the empty right keys with the new ones.
    // right_node.keys = new_right_keys;

    // let new_split_key = left_node
    //   .keys
    //   .last()
    //   .expect("should have left at least one element")
    //   .clone();

    // parent_node.handle_leaf_child_rotate(left_idx, new_split_key);

    // MergeOrRotateResult::DidRotate
  }
}
