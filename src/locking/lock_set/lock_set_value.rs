use locking::{Guard, LockMode};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// A LockSet holds Weak refs to Guards. This way the Guards are
// destroyed if no one is holding on to them.
//
// We also put the Guards in a RefCell. This is because, even though
// different queries in a transaction may store the Guard, when it comes
// to executing the transaction, only one person may be using a
// WriteGuard at a time.
//
// The LockMode tells us whether the Guard in the RefCell is a ReadGuard
// or a WriteGuard. In theory we can find this out by borrowing the
// RefCell, but that feels unnecessary.
pub(super) type RefCellGuard = RefCell<Guard>;
pub(super) type StrongRefCellGuard = Rc<RefCellGuard>;
pub(super) type WeakRefCellGuard = Weak<RefCellGuard>;

pub(super) struct LockSetValue {
  pub lock_mode: LockMode,
  pub guard: WeakRefCellGuard,
}
