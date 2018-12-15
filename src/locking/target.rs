// This represents a target for lock acquisition. There are two kinds of
// locks: RootIdentifier and Node locks. In the case of a Node lock, we
// specify the identifier, which is a String.
#[derive(Clone, Eq, Hash, PartialEq)]
pub enum LockTarget {
  RootIdentifier,
  Node(String),
}
