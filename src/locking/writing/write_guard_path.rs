use locking::WriteGuard;

pub struct WriteGuardPath {
  write_guards: Vec<WriteGuard>,
}

impl WriteGuardPath {
  pub fn new() -> WriteGuardPath {
    WriteGuardPath { write_guards: Vec::new() }
  }

  pub fn pop(&mut self, msg: &'static str) -> WriteGuard {
    self.write_guards.pop().expect(msg)
  }

  pub fn push(&mut self, guard: WriteGuard) {
    self.write_guards.push(guard);
  }

  pub fn peek_deepest_lock(&self) -> &WriteGuard {
    self.write_guards.last().expect("expected to hold at least one write guard")
  }

  pub fn clear(&mut self) {
    self.write_guards.clear();
  }
}
