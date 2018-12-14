#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TransactionMode {
  ReadOnly,
  ReadWrite,
}
