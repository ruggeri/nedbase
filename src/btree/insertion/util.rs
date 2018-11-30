use locking::{LockTarget, ReadGuard, WriteGuard, WriteGuardPath};

enum WriteLockAcquisitionResult {
  TopWriteLockVerificationFailed,
  Succeeded(Vec<WriteGuard>),
}

type LockVerificationPath = Vec<LockTarget>;
pub type ReadGuardPath = Vec<ReadGuard>;

struct InsertionGuards {
  read_guards: ReadGuardPath,
  write_guards: WriteGuardPath,
}
