#[derive(Clone, Copy, Eq, PartialEq)]
pub enum LockTargetRef<'a> {
  RootIdentifierTarget,
  NodeTarget { identifier: &'a str }
}

#[derive(Clone, Eq, PartialEq)]
pub enum LockTarget {
  RootIdentifierTarget,
  NodeTarget { identifier: String }
}

impl LockTarget {
  pub fn as_ref(&self) -> LockTargetRef {
    match self {
      LockTarget::RootIdentifierTarget => LockTargetRef::RootIdentifierTarget,
      LockTarget::NodeTarget { identifier } => LockTargetRef::NodeTarget {
        identifier: &identifier
      }
    }
  }
}
