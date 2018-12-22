#[allow(clippy::module_inception)]
mod btree;
pub(self) mod insertion;
mod lookup;
mod storage;
mod validate;

pub use self::btree::BTree;
