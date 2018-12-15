// TransactionMode determines whether you are allowed to acquire write
// locks at all. Also, if you *read* a node, in ReadWrite mode we'll
// have you acquire a *write* lock, because of the possibility that
// another query in the transaction wants to write that node.
//
// TODO: I think this should eventually live in a submodule dedicated to
// transactions.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TransactionMode {
  ReadOnly,
  ReadWrite,
}
