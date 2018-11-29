#[derive(Clone, Copy)]
pub enum LockTargetRef<'a> {
  RootIdentifierTarget,
  NodeTarget { identifier: &'a str }
}

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
