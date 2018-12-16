// Represents info about a split. This used to be an opaque type, but
// then I realized the user may need to know about the new nodes. The
// user may want to lock the new nodes for 2PL purposes.
pub struct SplitInfo {
  pub(in node) old_identifier: String,
  pub new_left_identifier: String,
  pub new_right_identifier: String,
  pub(in node) new_median: String,
}
