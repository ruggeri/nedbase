extern crate parking_lot;
extern crate rand;

// Allow submodules to access the public contents of other submodules.
pub(self) mod btree;
pub(self) mod constants;
pub(self) mod locking;
pub(self) mod node;

pub use btree::BTree;
// TODO: I would rather present something named more like a
// `Transaction` object. The `Transaction` would presumably manage the
// `LockSet`.
pub use locking::LockSet;
