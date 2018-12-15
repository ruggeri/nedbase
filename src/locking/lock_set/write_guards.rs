use locking::{Guard, ReadGuard, WriteGuard};
use node::Node;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

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

  pub fn node(&self) -> Ref<Node> {
    Ref::map(self.guard.borrow(), |guard| {
      guard.unwrap_node_ref(
        "Guard ref in LockSetNodeWriteGuard doesn't hold Node?",
      )
    })
  }

  pub fn node_mut(&mut self) -> RefMut<Node> {
    RefMut::map(self.guard.borrow_mut(), |guard| {
      guard.unwrap_node_mut_ref(
        "Guard ref in LockSetNodeWriteGuard doesn't hold Node?",
      )
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
