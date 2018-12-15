use locking::LockSetReadGuard;

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

  pub fn len(&self) -> usize {
    self.read_guards.len()
  }

  pub fn peek_deepest_lock(&self, msg: &'static str) -> &LockSetReadGuard {
    self.read_guards.last().expect(msg)
  }

  pub fn peek_head_lock(&self, msg: &'static str) -> &LockSetReadGuard {
    self.read_guards.get(0).as_ref().expect(msg)
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