use locking::LockSetWriteGuard;

pub struct WriteGuardPath {
  write_guards: Vec<LockSetWriteGuard>,
}

#[allow(clippy::new_without_default, clippy::new_without_default_derive)]
impl WriteGuardPath {
  pub fn new() -> WriteGuardPath {
    WriteGuardPath {
      write_guards: Vec::new(),
    }
  }

  pub fn clear(&mut self) {
    self.write_guards.clear();
  }

  pub fn peek_deepest_lock(&self, msg: &'static str) -> &LockSetWriteGuard {
    self.write_guards.last().expect(msg)
  }

  pub fn pop(&mut self, msg: &'static str) -> LockSetWriteGuard {
    self.write_guards.pop().expect(msg)
  }

  pub fn push(&mut self, guard: LockSetWriteGuard) {
    self.write_guards.push(guard);
  }
}
