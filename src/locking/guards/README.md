## `nedbase::locking::guards`

These are the most primitive form of guard.

What makes these guards dangerous is that they are not coordinated
through a `LockSet`. That means that if the same thread were to try to
acquire the same guard twice, it would deadlock itself.

For that reason, I've made the `acquire` methods private to
`nedbase:locking`.

The hierarchy is like so:

* `Guard`
  * `ReadGuard`: `NodeReadGuard`, `RootIdentifierReadGuard`
  * `WriteGuard`: `NodeWriteGuard`, `RootIdentifierWriteGuard`

There are many methods to cast from one kind of guard to another.
