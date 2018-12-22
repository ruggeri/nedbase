use btree::insertion::{
  descend_toward_key, DescentDecision, InsertPathEntry,
};
use locking::LockSet;
use node::SplitInfo;

// Travel back down to the node where the last split occurred. We call
// this if, while unwinding, we run out of nodes to propagate up through
// AND notice that the root has split and is still higher up.
pub fn redescend_toward_last_split(
  lock_set: &mut LockSet,
  split_info: &SplitInfo,
) -> Vec<InsertPathEntry> {
  descend_toward_key(lock_set, &split_info.new_median, |node_ref| {
    let next_node_identifier = match node_ref.next_node_identifier() {
      None => return DescentDecision::ContinueDescending,
      Some(next_node_identifier) => next_node_identifier,
    };

    // We know we have found the parent at which to insert the new
    // right child when we have found where it's left sibbling lives.
    if next_node_identifier == &split_info.new_right_identifier {
      DescentDecision::StopEarly
    } else {
      DescentDecision::ContinueDescending
    }
  })
}
