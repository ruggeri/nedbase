## TODOs

* Review the methods in `nedbase::btree`.
* In `LockSetReadGuard` and `LockSetWriteGuard`, downcasting is
  ridiculous.
* Hunt down and figure out how to kill all `String::from`?
* Introduce a `Transaction` class. Allow aborts.
* And then introduce deadlock detection.
* Introduce `MergeInfo`: make it similar to `SplitInfo` for symmetry.
