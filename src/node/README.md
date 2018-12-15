## `nedbase::node`

The main classes here are:

* `LeafNode`
* `InteriorNode`
* `Node`

`LeafNode` and `InteriorNode` are just what they sound like.

For the `LeafNode`, presently there are no values stored: only keys. The
keys are `String`s.

The `InteriorNode` does not directly have access to its children. It has
a vector of `child_identifiers`. In part for this reason, `insert` and
`delete` methods are *not* written for `InteriorNode`; it would not be
easily possible to write them recursively.

We leave traversal of the `BTree` for the user of the `Node` classes.
The user will have to bubble up splits caused by an insert. Likewise,
when a delete forces a merge, the user will have to handle this and
bubble up the merge to the parent.

The `Node` enum is a wrapper that has a variants for `LeafNode` and
`InteriorNode`. It provides a number of methods common to both, as well
as methods to unwrap (AKA, *downcast*) the more specific types.
