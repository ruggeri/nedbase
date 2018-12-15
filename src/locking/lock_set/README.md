## `nedbase::locking::lock_set`

**LockSet**

The `LockSet` manages all the locks that a transaction will acquire.
Because two-phase locking requires us to hold all locks to the end of
the transaction, `LockSet` will make sure that (within a single
transaction) Query 2 doesn't try to "reacquire" locks that were already
taken and held by Query 1.

When in `ReadWrite` mode, `LockSet` will take write locks even when you
want to read. This is because a *later* query might want to take a write
lock on the node.

The exception is *temporary* locks. When descending a tree, you might
take some locks, but you don't need to hold those for two-phase locking
if you don't need to modify those interior nodes. Therefore `LockSet`
can take read locks for you. However, it is an error to try to take a
"held" lock on a node that you presently have a temporary lock on

The reason is that you can never safely "upgrade" locks from read to
write modes.

**Guards**

I introduce a higher level concept of guard for `LockSet`. The reason is
this: you may want to take a read lock, but the lock behind the scenes
may be a write lock if you are in `ReadWrite` mode. To abstract this, I
make `LockSetReadGuard` and `LockSetWriteGuard` classes. The
`LockSetReadGuard` may be backed by either a `ReadGuard` or a
`WriteGuard`.

To make sure that the `LockSet` will autodrop locks that are not held
onto, we put the locks into an `Rc`. The `LockSet` will only hold weak
references.
