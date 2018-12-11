#[must_use]
#[derive(Clone, Copy)]
pub enum UnderflowActionResult {
  ContinueBubbling,
  StopBubbling,
}
