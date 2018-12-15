// Opaque type: only use for client is to pass to the
// `InteriorNode#handle_split` method (or possibly
// `InteriorNode#store_new_root`).
pub struct SplitInfo {
  pub(in node) old_identifier: String,
  pub(in node) new_left_identifier: String,
  pub(in node) new_right_identifier: String,
  pub(in node) new_median: String,
}
