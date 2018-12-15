## `LockAcquirer`

```
enum TransactionMode {
  ReadOnly,
  ReadWrite,
}

enum Guard {
  Read(ReadGuard),
  Write(WriteGuard),
}


impl Guard {
}

struct SharedGuard {
  guard: Rc<RefCell<Guard>>
}

impl SharedGuard {
  pub fn borrow(&self) -> {
    guard.borrow()
  }

  pub fn bottor_mut(&self) -> {
    guard.borrow_mut()
  }
}

struct LockSet {
  guards: HashMap<Weak<Guard>>,
  mode: ReadWrite
}

impl LockAcquirer {
  pub fn get_read_lock(&'a self) -> SharedGuard {

    // Checks guards and tries to promote.

    // If not, gets the lock in the default mode.

  }

  pub fn get_write_lock(&'a self) -> Ref<'a, Node> {

    // Checks guards and tries to promote. If not, checks mode and
    // returns lock.

  }

  pub fn get_temporary_read_lock(&'a self) -> Ref<'a, Node> {

  }

  pub fn 
}

```

* Want it to still drop the lock if no one wants it. Just like Guards
  currently do.
* But you can't actually know whether it's being held for someone else.
* That suggests that we need to use Rc to hold these guards.
* Maybe the `LockSet` can hold *weak* references.
    * And it will reacquire as needed, but if it *is* acquired will
      just give it to you.

* And if you have an `Rc<RefCell<Guard>>`, then it's easy if you want
  to borrow.
* And you can `borrow_mut` too.

* And I can add a wrapper type to hide the implementation.
