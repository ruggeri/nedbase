## TODOs

**Material**
* In `LockSetReadGuard` and `LockSetWriteGuard`, downcasting is
  ridiculous.
* Introduce a `Transaction` class. Allow aborts.
* And then introduce deadlock detection.

**Nice to Haves**

* Write documentation for `nedbase::btree` modules.
* Eliminate `String::from`s in a few places. (Notably, in lookup for
  `LockSet`).
* Think about how I take root identifier lock sometimes unnecessarily..
* Introduce `MergeInfo`: make it similar to `SplitInfo` for symmetry.
