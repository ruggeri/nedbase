extern crate nedbase;

use nedbase::BTree;
use std::sync::Arc;
use std::thread;

const MAX_KEYS_PER_NODE: usize = 128;
const NUM_INSERTIONS_PER_THREAD: u32 = 10_000;
const NUM_THREADS: u32 = 4;

fn perform_insertions(btree: &BTree) {
  let mut insertions = vec![];
  for _ in 0..NUM_INSERTIONS_PER_THREAD {
    let insertion = BTree::get_new_identifier();
    insertions.push(insertion.clone());
    btree.insert(insertion.clone());
  }

  for insertion in insertions {
    if !btree.contains_key(&insertion) {
      println!("Dropped key: {}", insertion);
    }
  }
}

fn main() {
  let btree = Arc::new(BTree::new(MAX_KEYS_PER_NODE));
  let mut join_handles = vec![];

  for _ in 0..NUM_THREADS {
    join_handles.push({
      let btree = Arc::clone(&btree);
      thread::spawn(move || perform_insertions(&btree))
    });
  }

  for handle in join_handles {
    handle.join().expect("no threads should have problems");
  }
}
