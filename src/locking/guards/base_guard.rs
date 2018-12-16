use super::{ReadGuard, WriteGuard};
use locking::LockTarget;
use node::Node;

pub enum Guard {
  Read(ReadGuard),
  Write(WriteGuard),
}

impl Guard {
  pub fn target(&self) -> LockTarget {
    match self {
      Guard::Read(read_guard) => read_guard.target(),
      Guard::Write(write_guard) => write_guard.target(),
    }
  }

  pub fn unwrap_node_mut_ref(
    &mut self,
    msg: &'static str,
  ) -> &mut Node {
    match self {
      Guard::Read(_) => {
        panic!("Cannot unwrap a mutable reference to a ReadGuard!")
      }

      Guard::Write(write_guard) => {
        write_guard.unwrap_node_write_guard_mut_ref(msg)
      }
    }
  }

  pub fn unwrap_node_ref(&self, msg: &'static str) -> &Node {
    match self {
      Guard::Read(read_guard) => {
        read_guard.unwrap_node_read_guard_ref(msg)
      }

      Guard::Write(write_guard) => {
        write_guard.unwrap_node_write_guard_ref(msg)
      }
    }
  }

  pub fn unwrap_root_identifier_mut_ref(
    &mut self,
    msg: &'static str,
  ) -> &mut String {
    match self {
      Guard::Read(_) => {
        panic!("Cannot unwrap a mutable reference to a ReadGuard!")
      }

      Guard::Write(write_guard) => {
        write_guard.unwrap_root_identifier_write_guard_mut_ref(msg)
      }
    }
  }

  pub fn unwrap_root_identifier_ref(
    &self,
    msg: &'static str,
  ) -> &String {
    match self {
      Guard::Read(read_guard) => {
        read_guard.unwrap_root_identifier_read_guard_ref(msg)
      }

      Guard::Write(write_guard) => {
        write_guard.unwrap_root_identifier_write_guard_ref(msg)
      }
    }
  }

  pub fn unwrap_write_guard_ref(
    &self,
    msg: &'static str,
  ) -> &WriteGuard {
    match self {
      Guard::Read(_) => panic!(msg),

      Guard::Write(write_guard) => write_guard,
    }
  }

  pub fn unwrap_write_guard_mut_ref(
    &mut self,
    msg: &'static str,
  ) -> &mut WriteGuard {
    match self {
      Guard::Read(_) => panic!(msg),

      Guard::Write(write_guard) => write_guard,
    }
  }
}
