use super::MergeInfo;

pub enum DeletionResult {
  DidDelete,
  DidDeleteWithMerge(MergeInfo),
  KeyWasNotPresent,
}
