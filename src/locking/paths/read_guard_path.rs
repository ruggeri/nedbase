use locking::LockSetReadGuard;

// A ReadGuardPath is a path of `LockSetReadGuard`, typically acquired
// from the root down to a `LeafNode`. This class is helpful if you are
// holding many read locks as you search for the deepest stable ancestor
// of a LeafNode.
pub struct ReadGuardPath {
  read_guards: Vec<LockSetReadGuard>,
}

#[allow(
  clippy::len_without_is_empty,
  clippy::new_without_default,
  clippy::new_without_default_derive
)]
impl ReadGuardPath {
  pub fn new() -> ReadGuardPath {
    ReadGuardPath {
      read_guards: Vec::new(),
    }
  }

  pub fn clear(&mut self) {
    self.read_guards.clear();
  }

  pub fn peek_deepest_lock(
    &self,
    msg: &'static str,
  ) -> &LockSetReadGuard {
    self.read_guards.last().expect(msg)
  }

  pub fn pop(&mut self, msg: &'static str) -> LockSetReadGuard {
    self.read_guards.pop().expect(msg)
  }

  pub fn push(&mut self, guard: LockSetReadGuard) {
    self.read_guards.push(guard);
  }

  pub fn truncate(&mut self, new_len: usize) {
    self.read_guards.truncate(new_len);
  }
}
