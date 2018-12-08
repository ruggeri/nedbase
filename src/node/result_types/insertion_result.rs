use node::SplitInfo;

pub enum InsertionResult {
  DidInsert,
  DidInsertWithSplit(SplitInfo),
  KeyWasAlreadyInserted,
}
