// An enum for distinguishing a ReadLock from a WriteLock.
//
// TODO: This maybe should be in locking itself.
pub(super) enum LockMode {
  Read,
  Write,
}
