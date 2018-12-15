#[allow(clippy::module_inception)]
mod btree;
mod deletion;
mod insertion;
mod lookup;
mod storage;

// util is helpful in submodules too.
pub(in btree) mod util;

pub use self::btree::BTree;
