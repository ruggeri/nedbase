use locking::LockTarget;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum LockTargetRef<'a> {
  RootIdentifierTarget,
  NodeTarget(&'a str)
}

impl<'a> LockTargetRef<'a> {
  pub fn promote_to_val(&self) -> LockTarget {
    match self {
      LockTargetRef::RootIdentifierTarget => LockTarget::RootIdentifierTarget,
      LockTargetRef::NodeTarget(identifier) => LockTarget::NodeTarget(
        String::from(*identifier)
      )
    }
  }

  pub fn unwrap_identifier(&self, message: &'static str) -> &str {
    match self {
      LockTargetRef::RootIdentifierTarget => panic!(message),
      LockTargetRef::NodeTarget(identifier) => identifier,
    }
  }
}
