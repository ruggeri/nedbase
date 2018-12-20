#[allow(clippy::module_inception)]
mod btree;
// mod deletion;
mod insertion;
mod lookup;
mod storage;
mod validate;

pub use self::btree::BTree;
