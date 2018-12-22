use node::SplitInfo;

pub enum UnwindingResult {
  FinishedUnwinding,
  MustContinueUnwinding(SplitInfo),
  MustRedescend(SplitInfo),
}
