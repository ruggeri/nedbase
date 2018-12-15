use locking::{Guard, ReadGuard, WriteGuard};
use node::Node;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct LockSetReadGuard {
  guard: Rc<RefCell<Guard>>,
}

pub struct LockSetWriteGuard {
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

  pub fn unwrap_node_ref(&self, msg: &'static str) -> Ref<Node> {
    Ref::map(
      self.guard.borrow(),
      |guard| guard.unwrap_node_ref(msg)
    )
  }

  pub fn unwrap_root_identifier_ref(&self, msg: &'static str) -> Ref<String> {
    Ref::map(
      self.guard.borrow(),
      |guard| guard.unwrap_root_identifier_ref(msg)
    )
  }

  pub fn downcast(&self) -> (Option<Ref<String>>, Option<Ref<Node>>) {
    match &(*self.guard.borrow()) {
      Guard::Read(read_guard) => {
        match read_guard {
          ReadGuard::NodeReadGuard(node_guard) => {
            (None, Some(self.unwrap_node_ref("We just verified we're a node read guard...")))
          }

          ReadGuard::RootIdentifierReadGuard(root_identifier_guard) => {
            (Some(self.unwrap_root_identifier_ref("We just verified we're a RootIdentifierReadGuard...")), None)
          }
        }
      }

      Guard::Write(write_guard) => {
        match write_guard {
          WriteGuard::NodeWriteGuard(node_guard) => {
            (None, Some(self.unwrap_node_ref("We just verified we're a node read guard...")))
          }

          WriteGuard::RootIdentifierWriteGuard(root_identifier_guard) => {
            (Some(self.unwrap_root_identifier_ref("We just verified we're a RootIdentifierReadGuard...")), None)
          }
        }
      }
    }
  }
}

impl LockSetWriteGuard {
  pub fn from_guard(guard: Rc<RefCell<Guard>>) -> LockSetWriteGuard {
    LockSetWriteGuard { guard }
  }

  pub fn guard(&self) -> Ref<WriteGuard> {
    Ref::map(
      self.guard.borrow(),
      |guard| guard.unwrap_write_guard("LockSetWriteGuard must hold WriteGuard")
    )
  }

  pub fn guard_mut(&mut self) -> RefMut<WriteGuard> {
    RefMut::map(
      self.guard.borrow_mut(),
      |guard| guard.unwrap_write_guard_mut("LockSetWriteGuard must hold WriteGuard")
    )
  }

  pub fn unwrap_node_ref(&self, msg: &'static str) -> Ref<Node> {
    Ref::map(
      self.guard.borrow(),
      |guard| guard.unwrap_node_ref(msg)
    )
  }

  pub fn unwrap_node_mut_ref(&mut self, msg: &'static str) -> RefMut<Node> {
    RefMut::map(
      self.guard.borrow_mut(),
      |guard| guard.unwrap_node_mut_ref(msg)
    )
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

  pub fn upcast(self) -> LockSetWriteGuard {
    LockSetWriteGuard::from_guard(self.guard)
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

  pub fn identifier(&self) -> Ref<String> {
    let msg = "Guard ref in LockSetRootIdentifierWriteGuard doesn't hold RootIdentifier?";
    Ref::map(
      self.guard.borrow(),
      |guard| guard.unwrap_root_identifier_ref(msg)
    )
  }

  pub fn upcast(self) -> LockSetWriteGuard {
    LockSetWriteGuard::from_guard(self.guard)
  }
}
