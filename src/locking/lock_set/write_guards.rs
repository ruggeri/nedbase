use locking::{Guard, WriteGuard};
use node::{InteriorNode, Node};
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

// LockSetWriteGuard is simpler than LockSetReadGuard because there is
// only one kind of primitive `Guard` backing the LockSetWriteGuard: a
// `WriteGuard`.
//
// Note that we borrow and borrow_mut from a `RefCell`. Why? That's
// because if the same transaction contains multiple queries which
// overlap in locks they acquire, they will hold the same locks. That is
// fine, but they must not *use* the same locks simultaneously.

#[derive(Clone)]
pub struct LockSetWriteGuard {
  guard: Rc<RefCell<Guard>>,
}

#[derive(Clone)]
pub struct LockSetNodeWriteGuard {
  guard: Rc<RefCell<Guard>>,
}

#[derive(Clone)]
pub struct LockSetRootIdentifierWriteGuard {
  guard: Rc<RefCell<Guard>>,
}

impl LockSetWriteGuard {
  pub fn from_guard(guard: Rc<RefCell<Guard>>) -> LockSetWriteGuard {
    LockSetWriteGuard { guard }
  }

  pub(in super) fn clone_ref_cell_guard(&self) -> Rc<RefCell<Guard>> {
    Rc::clone(&self.guard)
  }

  // TODO: I don't love that we are handing out the primitive
  // WriteGuards...
  pub fn guard(&self) -> Ref<WriteGuard> {
    Ref::map(self.guard.borrow(), |guard| {
      guard.unwrap_write_guard_ref(
        "LockSetWriteGuard must hold WriteGuard",
      )
    })
  }

  pub fn guard_mut(&mut self) -> RefMut<WriteGuard> {
    RefMut::map(self.guard.borrow_mut(), |guard| {
      guard.unwrap_write_guard_mut_ref(
        "LockSetWriteGuard must hold WriteGuard",
      )
    })
  }

  pub fn unwrap_node_ref(&self, msg: &'static str) -> Ref<Node> {
    Ref::map(self.guard.borrow(), |guard| guard.unwrap_node_ref(msg))
  }

  pub fn unwrap_node_mut_ref(
    &mut self,
    msg: &'static str,
  ) -> RefMut<Node> {
    RefMut::map(self.guard.borrow_mut(), |guard| {
      guard.unwrap_node_mut_ref(msg)
    })
  }
}

impl LockSetNodeWriteGuard {
  pub fn from_guard(
    guard: Rc<RefCell<Guard>>,
  ) -> LockSetNodeWriteGuard {
    LockSetNodeWriteGuard { guard }
  }

  pub(in super) fn clone_ref_cell_guard(&self) -> Rc<RefCell<Guard>> {
    Rc::clone(&self.guard)
  }

  pub fn is_leaf_node(&self) -> bool {
    let guard = self.guard.borrow();
    guard.unwrap_node_ref(
      "Guard ref in LockSetNodeWriteGuard doesn't hold Node?"
    ).is_leaf_node()
  }

  pub fn unwrap_node_ref(&self) -> Ref<Node> {
    Ref::map(self.guard.borrow(), |guard| {
      guard.unwrap_node_ref(
        "Guard ref in LockSetNodeWriteGuard doesn't hold Node?",
      )
    })
  }

  pub fn unwrap_node_mut_ref(&mut self) -> RefMut<Node> {
    RefMut::map(self.guard.borrow_mut(), |guard| {
      guard.unwrap_node_mut_ref(
        "Guard ref in LockSetNodeWriteGuard doesn't hold Node?",
      )
    })
  }

  pub fn unwrap_interior_node_ref(
    &self,
    msg: &'static str,
  ) -> Ref<InteriorNode> {
    Ref::map(self.guard.borrow(), |guard| {
      guard
        .unwrap_node_ref(
          "Guard ref in LockSetNodeWriteGuard doesn't hold Node?",
        )
        .unwrap_interior_node_ref(msg)
    })
  }

  pub fn upcast(self) -> LockSetWriteGuard {
    LockSetWriteGuard::from_guard(self.guard)
  }
}

impl LockSetRootIdentifierWriteGuard {
  pub fn from_guard(
    guard: Rc<RefCell<Guard>>,
  ) -> LockSetRootIdentifierWriteGuard {
    LockSetRootIdentifierWriteGuard { guard }
  }

  pub fn identifier(&self) -> Ref<String> {
    let msg = "Guard ref in LockSetRootIdentifierWriteGuard doesn't hold RootIdentifier?";
    Ref::map(self.guard.borrow(), |guard| {
      guard.unwrap_root_identifier_ref(msg)
    })
  }

  pub fn identifier_mut(&self) -> RefMut<String> {
    let msg = "Guard ref in LockSetRootIdentifierWriteGuard doesn't hold RootIdentifier?";
    RefMut::map(self.guard.borrow_mut(), |guard| {
      guard.unwrap_root_identifier_mut_ref(msg)
    })
  }

  pub fn upcast(self) -> LockSetWriteGuard {
    LockSetWriteGuard::from_guard(self.guard)
  }
}
