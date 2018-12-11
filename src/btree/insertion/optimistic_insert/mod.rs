mod acquire_parent_of_deepest_stable_node;
mod acquire_write_guard_path;
mod core;

pub(self) use self::acquire_parent_of_deepest_stable_node::acquire_parent_of_deepest_stable_node;
pub(self) use self::acquire_write_guard_path::acquire_write_guard_path;

pub use self::core::optimistic_insert;
