extern crate nedbase;

use nedbase::BTree;
use std::sync::Arc;
use std::thread;

const MAX_KEYS_PER_NODE: usize = 1024;
const NUM_INSERTIONS_PER_THREAD: u32 = 10_000;
const NUM_THREADS: u32 = 32;

fn main() {
  let btree = Arc::new(BTree::new(MAX_KEYS_PER_NODE));

  // Spawn a bunch of threads to hammer the BTree.
  let mut join_handles = vec![];
  for _ in 0..NUM_THREADS {
    join_handles.push({
      let btree = Arc::clone(&btree);
      thread::spawn(move || perform_insertions(&btree))
    });
  }

  // Wait for them to all finish.
  for handle in join_handles {
    handle.join().expect("no threads should panic");
  }
}

// A thread's work.
fn perform_insertions(btree: &Arc<BTree>) {
  // Make lots and lots of insertions.
  let mut insertions = vec![];
  for _ in 0..NUM_INSERTIONS_PER_THREAD {
    let insertion = BTree::get_new_identifier();
    BTree::optimistic_insert(btree, &insertion);
    insertions.push(insertion.clone());
  }

  // Next, check that we can properly find what we have added.
  for insertion in insertions {
    if !BTree::contains_key(btree, &insertion) {
      println!("Dropped key: {}", insertion);
      continue;
    }

    // And interleave deletions.
    BTree::delete(btree, &insertion);
  }
}
