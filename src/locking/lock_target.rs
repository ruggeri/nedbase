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

impl<'a> LockTargetRef<'a> {
  pub fn as_val(&self) -> LockTarget {
    match self {
      LockTargetRef::RootIdentifierTarget => LockTarget::RootIdentifierTarget,
      LockTargetRef::NodeTarget { identifier } => LockTarget::NodeTarget {
        identifier: String::from(*identifier)
      }
    }
  }
}
