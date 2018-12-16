## TODOs

* Write documentation for `nedbase::btree` modules.
* In `LockSetReadGuard` and `LockSetWriteGuard`, downcasting is
  ridiculous.
* With the exception of `nedbase::btree::deletion`, I have marked
  everywhere where I do `String::from`.
  * I always need `String::from` whern storing a node identifier or
    value.
  * I use it in LockSet, probably necessarily.
  * I did mark one place with TODO where it is unneeded in `lookup.rs`.
* Introduce a `Transaction` class. Allow aborts.
* And then introduce deadlock detection.
* Introduce `MergeInfo`: make it similar to `SplitInfo` for symmetry.
