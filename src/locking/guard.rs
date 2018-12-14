use locking::{ReadGuard, WriteGuard};
use node::Node;

pub enum Guard {
  Read(ReadGuard),
  Write(WriteGuard),
}

impl Guard {
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

  pub fn unwrap_root_identifier_ref(&self, msg: &'static str) -> &String {
    match self {
      Guard::Read(read_guard) => {
        read_guard.unwrap_root_identifier_read_guard_ref(msg)
      }

      Guard::Write(write_guard) => {
        write_guard.unwrap_root_identifier_write_guard_ref(msg)
      }
    }
  }
}
