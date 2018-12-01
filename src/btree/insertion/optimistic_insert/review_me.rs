pub fn insert(btree: &Arc<BTree>, key: String) {
  let mut insertion_guards = loop {
    let read_guards = BTree::build_read_path_for_insert(btree, &key);
    match BTree::finish_building_insertion_guards(btree, read_guards, &key) {
      None => continue,
      Some(insertion_guards) => break insertion_guards
    }
  };

  ::util::log_method_entry("beginning insertion process");
  let mut insertion_result = {
    let last_write_guard = insertion_guards.write_guards.pop().expect("should acquire at least one write guard");
    let mut current_node_guard = last_write_guard
      .unwrap_node_write_guard("last write_guard should be a node guard");
    current_node_guard
      .unwrap_leaf_node_mut_ref("Expected leaf node to insert into at bottom")
      .insert(btree, key)
  };

  while let InsertionResult::DidInsertWithSplit(child_split_info) = insertion_result {
    let mut last_write_guard = insertion_guards.write_guards.pop().expect("should not run out of write guards");

    match last_write_guard {
      WriteGuard::RootIdentifierWriteGuard(mut identifier_guard) => {
        ::util::log_method_entry("trying to split root");

        let new_root_identifier = btree.store_new_root_node(
          child_split_info
        );

        *identifier_guard = new_root_identifier;

        break
      },

      WriteGuard::NodeWriteGuard(mut node_guard) => {
        insertion_result = node_guard
          .unwrap_interior_node_mut_ref("expected interior node")
          .handle_split(btree, child_split_info);
      }
    };
  }

  ::util::log_method_entry("insert completed");
}
