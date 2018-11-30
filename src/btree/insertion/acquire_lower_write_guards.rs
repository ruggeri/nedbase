fn finish_building_insertion_guards(btree: &Arc<BTree>, mut read_guards: ReadGuardPath, key: &str) -> Option<InsertionGuards> {
  let mut write_guards = Vec::new();

  let top_write_lock_location = {
    let top_read_guard = read_guards.pop().unwrap();
    top_read_guard.location().promote_to_val()
  };

  match top_write_lock_location {
    LockTarget::RootIdentifierTarget => {
      let root_identifier_guard = RootIdentifierWriteGuard::acquire(btree);
      let root_guard = WriteGuard::acquire_node_write_guard(btree, &root_identifier_guard);

      write_guards.push(WriteGuard::RootIdentifierWriteGuard(root_identifier_guard));
      write_guards.push(root_guard);
    },
    LockTarget::NodeTarget(identifier) => {
      let node_guard = NodeWriteGuard::acquire(btree, &identifier);
      if !node_guard.can_grow_without_split() {
        // We failed; this is no longer stable.
        return None;
      }

      write_guards.push(WriteGuard::NodeWriteGuard(node_guard));
    }
  }

  // Descend acquiring write guards.
  loop {
    let child_guard = {
      let node_guard = write_guards.last().unwrap().unwrap_node_write_guard_ref("expected node");
      match &(**node_guard) {
        Node::LeafNode(_) => break,
        Node::InteriorNode(inode) => {
          let child_identifier = inode.child_identifier_by_key(key);
          WriteGuard::acquire_node_write_guard(btree, child_identifier)
        }
      }
    };

    write_guards.push(child_guard);
  };

  Some(InsertionGuards {
    read_guards,
    write_guards
  })
}
