#[must_use]
#[derive(Clone, Copy)]
pub enum DeletionActionResult {
  ContinueBubbling,
  StopBubbling,
}
