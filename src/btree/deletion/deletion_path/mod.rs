mod acquire;
mod path;
mod path_entry;

pub use self::acquire::acquire_deletion_path;
pub use self::path::DeletionPath;
pub use self::path_entry::DeletionPathEntry;
