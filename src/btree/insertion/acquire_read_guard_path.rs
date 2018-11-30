// Finds highest lock target that may need to be mutated by an
// insertion.
fn build_read_path_for_insert(btree: &Arc<BTree>, key: &str) -> ReadGuardPath {
  ::util::log_method_entry("build_read_path_for_insert starting");

  let mut current_path = ReadGuardPath::new();
  {
    let root_identifier_guard = RootIdentifierReadGuard::acquire(btree);
    let root_node_guard = NodeReadGuard::acquire(btree, &root_identifier_guard);

    current_path.push(ReadGuard::RootIdentifierReadGuard(root_identifier_guard));
    current_path.push(ReadGuard::NodeReadGuard(root_node_guard));
  }

  loop {
    let child_guard = {
      let node_guard = current_path.last().unwrap().unwrap_node_read_guard_ref("expected node");
      match &(**node_guard) {
        Node::LeafNode(_) => break,
        Node::InteriorNode(inode) => {
          let child_identifier = inode.child_identifier_by_key(key);
          ReadGuard::acquire_node_read_guard(btree, child_identifier)
        }
      }
    };

    current_path.push(child_guard);
  };

  while current_path.len() > 1 {
    {
      let read_guard = current_path.last().unwrap();
      let read_guard = read_guard.unwrap_node_read_guard_ref("expected node");

      if read_guard.can_grow_without_split() {
        break;
      }
    }

    current_path.pop();
  }

  current_path
}
