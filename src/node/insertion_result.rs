use node::SplitInfo;

pub enum InsertionResult {
  DidInsert,
  KeyWasAlreadyInserted,
  DidInsertWithSplit(SplitInfo),
}
