use locking::{LockTarget, ReadGuard, ReadGuardPath, WriteGuard, WriteGuardPath};

enum WriteLockAcquisitionResult {
  TopWriteLockVerificationFailed,
  Succeeded(Vec<WriteGuard>),
}

type LockVerificationPath = Vec<LockTarget>;

struct InsertionGuards {
  read_guards: ReadGuardPath,
  write_guards: WriteGuardPath,
}
