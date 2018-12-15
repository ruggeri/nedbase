## `nedbase::locking`

There are a number of submodules.

### `nedbase::locking::guards`

These are primitive guards. They are not coordinated by any `LockSet`.
You can read more about them in the README contained in the submodule.

### `nedbase::locking::lock_set`

See the extensive README in the submodule.

### `nedbase::locking::paths`

These are helper classes that allow you to hold a sequence of read or
write guards. `ReadGuardPath` is useful when descending the tree and
acquiring a series of read locks when searching for the deepest stable
ancestor. Likewise `WriteGuardPath` is useful when descending from the
deepest stable ancestor and acquiring all needed write locks.

## Other `nedbase::locking::*`

* `LockTarget` is an enum that helps you generically specify whether you
  want to acquire a RootIdentifier lock or a Node lock.
* `TransactionMode` is an enum for specifying whether we are running a
  ReadOnly or ReadWrite transaction. It determines what kinds of locks
  the `LockSet` will try to acquire.
