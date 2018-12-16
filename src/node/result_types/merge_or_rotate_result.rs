#[derive(Clone, Eq, PartialEq)]
pub enum MergeOrRotateResult {
  DidMerge {
    merge_node_identifier: String
  },
  DidRotate,
}
