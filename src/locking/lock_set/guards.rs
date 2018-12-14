use locking::Guard;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

rental! {
  mod rentals {
    use locking::Guard;
    use node::Node;
    use std::cell::{Ref, RefCell};
    use std::rc::Rc;

    #[rental(deref_suffix)]
    pub struct LockSetNodeReadGuard {
      lock: Rc<RefCell<Guard>>,
      guard: Ref<'lock, Node>,
    }

    #[rental(deref_suffix)]
    pub struct LockSetRootIdentifierReadGuard {
      lock: Rc<RefCell<Guard>>,
      guard: Ref<'lock, String>,
    }
  }
}

pub use self::rentals::{LockSetNodeReadGuard, LockSetRootIdentifierReadGuard};

impl LockSetNodeReadGuard {
  pub fn from_guard(guard: Rc<RefCell<Guard>>, unwrap_msg: &'static str) -> LockSetNodeReadGuard {
    LockSetNodeReadGuard::new(guard, |guard| {
      Ref::map(
        guard.borrow(),
        |guard| {
          guard.unwrap_node_ref(unwrap_msg)
        }
      )
    })
  }
}

impl LockSetRootIdentifierReadGuard {
  pub fn from_guard(guard: Rc<RefCell<Guard>>, unwrap_msg: &'static str) -> LockSetRootIdentifierReadGuard {
    LockSetRootIdentifierReadGuard::new(guard, |guard| {
      Ref::map(
        guard.borrow(),
        |guard| {
          guard.unwrap_root_identifier_ref(unwrap_msg)
        }
      )
    })
  }
}
