use locking::Guard;
use node::Node;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct LockSetReadGuard {
  guard: Rc<RefCell<Guard>>,
}

pub struct LockSetNodeReadGuard {
  guard: Rc<RefCell<Guard>>,
}

pub struct LockSetNodeWriteGuard {
  guard: Rc<RefCell<Guard>>,
}

pub struct LockSetRootIdentifierReadGuard {
  guard: Rc<RefCell<Guard>>,
}

pub struct LockSetRootIdentifierWriteGuard {
  guard: Rc<RefCell<Guard>>,
}

impl LockSetReadGuard {
  pub fn from_guard(guard: Rc<RefCell<Guard>>) -> LockSetReadGuard {
    LockSetReadGuard { guard }
  }
}

impl LockSetNodeReadGuard {
  pub fn from_guard(guard: Rc<RefCell<Guard>>) -> LockSetNodeReadGuard {
    LockSetNodeReadGuard { guard }
  }

  pub fn node(&self) -> Ref<Node> {
    Ref::map(
      self.guard.borrow(),
      |guard| guard.unwrap_node_ref("Guard ref in LockSetNodeReadGuard doesn't hold Node?")
    )
  }

  pub fn upcast(self) -> LockSetReadGuard {
    LockSetReadGuard::from_guard(self.guard)
  }

  pub fn release(self) {}
}

impl LockSetNodeWriteGuard {
  pub fn from_guard(guard: Rc<RefCell<Guard>>) -> LockSetNodeWriteGuard {
    LockSetNodeWriteGuard { guard }
  }

  pub fn node(&self) -> Ref<Node> {
    Ref::map(
      self.guard.borrow(),
      |guard| guard.unwrap_node_ref("Guard ref in LockSetNodeWriteGuard doesn't hold Node?")
    )
  }

  pub fn node_mut(&mut self) -> RefMut<Node> {
    RefMut::map(
      self.guard.borrow_mut(),
      |guard| guard.unwrap_node_mut_ref("Guard ref in LockSetNodeWriteGuard doesn't hold Node?")
    )
  }
}

impl LockSetRootIdentifierReadGuard {
  pub fn from_guard(guard: Rc<RefCell<Guard>>) -> LockSetRootIdentifierReadGuard {
    LockSetRootIdentifierReadGuard { guard }
  }

  pub fn identifier(&self) -> Ref<String> {
    let msg = "Guard ref in LockSetRootIdentifierReadGuard doesn't hold RootIdentifier?";
    Ref::map(
      self.guard.borrow(),
      |guard| guard.unwrap_root_identifier_ref(msg)
    )
  }

  pub fn upcast(self) -> LockSetReadGuard {
    LockSetReadGuard::from_guard(self.guard)
  }
}

impl LockSetRootIdentifierWriteGuard {
  pub fn from_guard(guard: Rc<RefCell<Guard>>) -> LockSetRootIdentifierWriteGuard {
    LockSetRootIdentifierWriteGuard { guard }
  }
}
