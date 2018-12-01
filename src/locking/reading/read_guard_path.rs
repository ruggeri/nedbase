use locking::ReadGuard;

pub struct ReadGuardPath {
  read_guards: Vec<ReadGuard>,
}

impl ReadGuardPath {
  pub fn new() -> ReadGuardPath {
    ReadGuardPath { read_guards: Vec::new() }
  }

  pub fn len(&self) -> usize {
    self.read_guards.len()
  }

  pub fn pop(&mut self, msg: &'static str) -> ReadGuard {
    self.read_guards.pop().expect(msg)
  }

  pub fn push(&mut self, guard: ReadGuard) {
    self.read_guards.push(guard);
  }

  pub fn peek_head_lock(&self) -> &ReadGuard {
    self.read_guards.get(0).as_ref().expect("expected to hold at least one read guard")
  }

  pub fn peek_deepest_lock(&self) -> &ReadGuard {
    self.read_guards.last().expect("expected to hold at least one read guard")
  }

  pub fn truncate(&self, new_len: usize) {
    self.read_guards.truncate(new_len);
  }

  pub fn clear(&mut self) {
    self.read_guards.clear();
  }
}
