use btree::BTree;
use locking::{NodeWriteGuard, RootIdentifierWriteGuard, WriteGuard};
use std::collections::HashMap;
use std::sync::Arc;

pub struct WriteSet {
  map: HashMap<String, WriteGuard>,
}

impl WriteSet {
  pub fn new() -> WriteSet {
    WriteSet {
      map: HashMap::new(),
    }
  }

  pub fn acquire_node_guard(
    &mut self,
    btree: &Arc<BTree>,
    identifier: &str,
  ) -> &NodeWriteGuard {
    let node_guard = NodeWriteGuard::acquire(btree, &identifier);
    self
      .map
      .insert(String::from(identifier), node_guard.upcast());
    self.get_node_ref(identifier)
  }

  pub fn acquire_root_identifier(
    &mut self,
    btree: &Arc<BTree>,
  ) -> &RootIdentifierWriteGuard {
    let root_identifier_guard =
      RootIdentifierWriteGuard::acquire(btree);
    self
      .map
      .insert(String::from(""), root_identifier_guard.upcast());
    self.get_root_identifier_guard_ref()
  }

  pub fn drop_node_guard(&mut self, identifier: &str) {
    self.map.remove(identifier);
  }

  pub fn get_node_ref(&self, identifier: &str) -> &NodeWriteGuard {
    self
      .map
      .get(identifier)
      .expect("must acquire node before reading it")
      .unwrap_node_write_guard_ref(
        "only nodes should be stored under a proper identifier",
      )
  }

  pub fn get_node_mut_ref(
    &mut self,
    identifier: &str,
  ) -> &mut NodeWriteGuard {
    self
      .map
      .get_mut(identifier)
      .expect("must acquire node before reading it")
      .unwrap_node_write_guard_mut_ref(
        "only nodes should be stored under a proper identifier",
      )
  }

  pub fn get_root_identifier_guard_ref(
    &self,
  ) -> &RootIdentifierWriteGuard {
    self
      .map
      .get("")
      .expect("must acquire root identifier before reading it")
      .unwrap_root_identifier_write_guard_ref(
        "only root identifier should be stored under empty key",
      )
  }

  pub fn get_root_identifier_guard_mut_ref(
    &mut self,
  ) -> &mut RootIdentifierWriteGuard {
    self
      .map
      .get_mut("")
      .expect("must acquire root identifier before reading it")
      .unwrap_root_identifier_write_guard_mut_ref(
        "only root identifier should be stored under empty key",
      )
  }
}
