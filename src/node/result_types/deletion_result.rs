// TODO: I would like it if there were a MergeInfo result type just like
// InsertionResult. I would like to make merging more opaque to callers
// (if possible).
pub enum DeletionResult {
  DidDelete,
  KeyWasNotPresent,
}
