#[allow(clippy::module_inception)]
mod btree;
pub(in self) mod insertion;
mod lookup;
mod storage;
mod validate;

pub use self::btree::BTree;
