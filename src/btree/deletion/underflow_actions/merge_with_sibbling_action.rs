use super::UnderflowActionResult;
use btree::deletion::WriteSet;
use btree::BTree;

pub struct MergeWithSibblingAction {
  pub(super) parent_node_identifier: String,
  pub(super) child_node_identifier: String,
  pub(super) sibbling_node_identifier: String,
}

// Helper struct that performs the merge operation.
impl MergeWithSibblingAction {
  pub fn execute(
    &self,
    btree: &BTree,
    write_set: &mut WriteSet,
  ) -> UnderflowActionResult {
    // Get the write locks you've acquired on everyone.
    let parent_node = write_set
      .get_node_mut_ref(&self.parent_node_identifier)
      .unwrap_interior_node_mut_ref("parents must be InteriorNodes");

    let child_node =
      write_set.get_node_mut_ref(&self.child_node_identifier);

    let sibbling_node_identifier =
      write_set.get_node_mut_ref(&self.sibbling_node_identifier);

    // And then have the parent perform the merge or rotation.
    parent_node.merge_or_rotate_children(
      btree,
      child_node,
      sibbling_node_identifier,
    );

    // If after merge our parent is fine, we can stop.
    if parent_node.is_deficient() {
      UnderflowActionResult::ContinueBubbling
    } else {
      UnderflowActionResult::StopBubbling
    }
  }
}
