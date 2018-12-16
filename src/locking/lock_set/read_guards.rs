use locking::{Guard, ReadGuard, WriteGuard};
use node::{InteriorNode, LeafNode, Node};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

// The idea of the LockSetReadGuards is that when some code asks for a
// read lock, you *might* give it a write lock instead, if either:
//
// 1. They will do a read to the node, but you are in ReadWrite mode,
// 2. They want to do a temporary read to the node, but you already have
//    a write lock on the node.
//
// Therefore, LockSetReadGuard abstracts over the more primitive notion
// of `Guard`.
//
// Note that we borrow from a `RefCell`. Why? That's because if the same
// transaction contains multiple queries which overlap in locks they
// acquire, they will hold the same locks. That is fine, but they must
// not *use* the same locks simultaneously.

#[derive(Clone)]
pub struct LockSetReadGuard {
  guard: Rc<RefCell<Guard>>,
}

#[derive(Clone)]
pub struct LockSetNodeReadGuard {
  guard: Rc<RefCell<Guard>>,
}

#[derive(Clone)]
pub struct LockSetRootIdentifierReadGuard {
  guard: Rc<RefCell<Guard>>,
}

impl LockSetReadGuard {
  pub fn from_guard(guard: Rc<RefCell<Guard>>) -> LockSetReadGuard {
    LockSetReadGuard { guard }
  }

  // This lets them drop the lock early if they way, without having to
  // use std::mem::drop.
  //
  // Note that it won't necessarily release the lock! For instance, if
  // another query in the transaction holds the lock, it will not be
  // released!
  pub fn release(self) {}

  pub fn unwrap_node_ref(&self, msg: &'static str) -> Ref<Node> {
    Ref::map(self.guard.borrow(), |guard| guard.unwrap_node_ref(msg))
  }

  pub fn unwrap_root_identifier_ref(
    &self,
    msg: &'static str,
  ) -> Ref<String> {
    Ref::map(self.guard.borrow(), |guard| {
      guard.unwrap_root_identifier_ref(msg)
    })
  }

  // TODO: This method is a disaster.
  pub fn downcast(&self) -> (Option<Ref<String>>, Option<Ref<Node>>) {
    match &(*self.guard.borrow()) {
      Guard::Read(read_guard) => match read_guard {
        ReadGuard::NodeReadGuard(_) => (
          None,
          Some(self.unwrap_node_ref(
            "We just verified we're a node read guard...",
          )),
        ),

        ReadGuard::RootIdentifierReadGuard(_) => (
          Some(self.unwrap_root_identifier_ref(
            "We just verified we're a RootIdentifierReadGuard...",
          )),
          None,
        ),
      },

      Guard::Write(write_guard) => match write_guard {
        WriteGuard::NodeWriteGuard(_) => (
          None,
          Some(self.unwrap_node_ref(
            "We just verified we're a node read guard...",
          )),
        ),

        WriteGuard::RootIdentifierWriteGuard(_) => (
          Some(self.unwrap_root_identifier_ref(
            "We just verified we're a RootIdentifierReadGuard...",
          )),
          None,
        ),
      },
    }
  }
}

impl LockSetNodeReadGuard {
  pub fn from_guard(guard: Rc<RefCell<Guard>>) -> LockSetNodeReadGuard {
    LockSetNodeReadGuard { guard }
  }

  pub(in super) fn clone_ref_cell_guard(&self) -> Rc<RefCell<Guard>> {
    Rc::clone(&self.guard)
  }

  pub fn is_leaf_node(&self) -> bool {
    let guard = self.guard.borrow();
    guard.unwrap_node_ref(
      "Guard ref in LockSetNodeReadGuard doesn't hold Node?"
    ).is_leaf_node()
  }

  // This lets them drop the lock early if they way, without having to
  // use std::mem::drop.
  //
  // Note that it won't necessarily release the lock! For instance, if
  // another query in the transaction holds the lock, it will not be
  // released!
  pub fn release(self) {}

  pub fn unwrap_interior_node_ref(
    &self,
    msg: &'static str,
  ) -> Ref<InteriorNode> {
    Ref::map(self.guard.borrow(), |guard| {
      guard
        .unwrap_node_ref(
          "Guard ref in LockSetNodeReadGuard doesn't hold Node?",
        )
        .unwrap_interior_node_ref(msg)
    })
  }

  pub fn unwrap_leaf_node_ref(
    &self,
    msg: &'static str,
  ) -> Ref<LeafNode> {
    Ref::map(self.guard.borrow(), |guard| {
      guard
        .unwrap_node_ref(
          "Guard ref in LockSetNodeReadGuard doesn't hold Node?",
        )
        .unwrap_leaf_node_ref(msg)
    })
  }

  pub fn unwrap_node_ref(&self) -> Ref<Node> {
    Ref::map(self.guard.borrow(), |guard| {
      guard.unwrap_node_ref(
        "Guard ref in LockSetNodeReadGuard doesn't hold Node?",
      )
    })
  }

  pub fn upcast(self) -> LockSetReadGuard {
    LockSetReadGuard::from_guard(self.guard)
  }
}

impl LockSetRootIdentifierReadGuard {
  pub fn from_guard(
    guard: Rc<RefCell<Guard>>,
  ) -> LockSetRootIdentifierReadGuard {
    LockSetRootIdentifierReadGuard { guard }
  }

  pub fn identifier(&self) -> Ref<String> {
    let msg = "Guard ref in LockSetRootIdentifierReadGuard doesn't hold RootIdentifier?";
    Ref::map(self.guard.borrow(), |guard| {
      guard.unwrap_root_identifier_ref(msg)
    })
  }

  pub fn upcast(self) -> LockSetReadGuard {
    LockSetReadGuard::from_guard(self.guard)
  }
}
