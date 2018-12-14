use btree::BTree;
use locking::{LockTarget, ReadGuard, TransactionMode, WriteGuard};
use node::Node;
use std::sync::Arc;

pub enum Guard {
  Read(ReadGuard),
  Write(WriteGuard),
}

impl Guard {
  pub fn acquire_guard(btree: &Arc<BTree>, tx_mode: TransactionMode, lock_target: &LockTarget) -> Guard {
    match tx_mode {
      TransactionMode::ReadOnly => {
        // First acquire the guard.
        let guard = ReadGuard::acquire_read_guard(btree, lock_target);
        // Next, make it generic over Read/Write.
        Guard::Read(guard)
      }

      TransactionMode::ReadWrite => {
        // First acquire the guard.
        let guard = WriteGuard::acquire_write_guard(btree, lock_target);
        // Next, make it generic over Read/Write.
        Guard::Write(guard)
      }
    }
  }

  pub fn acquire_node_guard(btree: &Arc<BTree>, tx_mode: TransactionMode, identifier: &str) -> Guard {
    let lock_target = LockTarget::Node(String::from(identifier));
    Guard::acquire_guard(btree, tx_mode, &lock_target)
  }

  pub fn acquire_root_identifier_guard(btree: &Arc<BTree>, tx_mode: TransactionMode, identifier: &str) -> Guard {
    let lock_target = LockTarget::Node(String::from(identifier));
    Guard::acquire_guard(btree, tx_mode, &lock_target)
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
