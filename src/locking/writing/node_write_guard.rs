use btree::BTree;
use locking::WriteGuard;
use node::{InteriorNode, Node};

rental! {
  mod rentals {
    use node::Node;
    use parking_lot::{RwLock, RwLockWriteGuard};
    use std::sync::Arc;

    #[rental(deref_mut_suffix)]
    pub struct NodeWriteGuard {
        lock: Arc<RwLock<Node>>,
        guard: RwLockWriteGuard<'lock, Node>,
    }
  }
}

pub use self::rentals::NodeWriteGuard;

impl NodeWriteGuard {
  pub fn acquire(btree: &BTree, identifier: &str) -> NodeWriteGuard {
    let lock = btree.get_node_arc_lock(&identifier);

    NodeWriteGuard::new(lock, |lock| {
      lock.write()
    })
  }

  pub fn node(&self) -> &Node {
    &(*self)
  }

  pub fn upcast(self) -> WriteGuard {
    WriteGuard::NodeWriteGuard(self)
  }
}

// This method is sort-of monkey-patched here because it's really about
// NodeWriteGuard much more than InteriorNode.
impl InteriorNode {
  pub fn acquire_write_guard_for_child_by_key(&self, btree: &BTree, key: &str) -> NodeWriteGuard {
    let child_identifier = self.child_identifier_by_key(key);
    NodeWriteGuard::acquire(btree, child_identifier)
  }
}
