use locking::LockTargetRef;

#[derive(Clone, Eq, PartialEq)]
pub enum LockTarget {
  RootIdentifierTarget,
  NodeTarget(String),
}

impl LockTarget {
  pub fn as_ref(&self) -> LockTargetRef {
    match self {
      LockTarget::RootIdentifierTarget => LockTargetRef::RootIdentifierTarget,
      LockTarget::NodeTarget(identifier) => LockTargetRef::NodeTarget(
        &identifier
      )
    }
  }

  pub fn unwrap_identifier(&self, message: &'static str) -> &str {
    match self {
      LockTarget::RootIdentifierTarget => panic!(message),
      LockTarget::NodeTarget(identifier) => identifier
    }
  }
}
