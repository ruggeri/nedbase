use locking::WriteGuard;

pub struct WriteGuardPath {
  write_guards: Vec<WriteGuard>,
}

#[allow(new_without_default, new_without_default_derive)]
impl WriteGuardPath {
  pub fn new() -> WriteGuardPath {
    WriteGuardPath {
      write_guards: Vec::new(),
    }
  }

  pub fn clear(&mut self) {
    self.write_guards.clear();
  }

  pub fn peek_deepest_lock(&self) -> &WriteGuard {
    self
      .write_guards
      .last()
      .expect("expected to hold at least one write guard")
  }

  pub fn pop(&mut self, msg: &'static str) -> WriteGuard {
    self.write_guards.pop().expect(msg)
  }

  pub fn push(&mut self, guard: WriteGuard) {
    self.write_guards.push(guard);
  }
}
