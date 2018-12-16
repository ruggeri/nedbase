// Represents the result of a DeletionAction. If the DeletionAction made
// a node deficient, we must keep bubbling up the DeletionPath, merging
// more nodes.
#[must_use]
#[derive(Clone, Copy)]
pub enum DeletionActionResult {
  ContinueBubbling,
  StopBubbling,
}
