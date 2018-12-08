extern crate nedbase;

use nedbase::BTree;
use std::sync::Arc;
use std::thread;

const MAX_KEYS_PER_NODE: usize = 1024;
const NUM_INSERTIONS_PER_THREAD: u32 = 100_000;
const NUM_THREADS: u32 = 32;

struct PanicChecker {}

impl Drop for PanicChecker {
  fn drop(&mut self) {
    if ::std::thread::panicking() {
      println!("SOMEONE HAS PANICKED?");
    } else {
      // println!("thread has completed")
    }
  }
}

fn perform_insertions(btree: &Arc<BTree>) {
  let panic_checker = PanicChecker {};

  let mut insertions = vec![];
  for _ in 0..NUM_INSERTIONS_PER_THREAD {
    let insertion = BTree::get_new_identifier();
    BTree::optimistic_insert(btree, &insertion);
    insertions.push(insertion.clone());
  }

  for insertion in insertions {
    if !BTree::contains_key(btree, &insertion) {
      println!("Dropped key: {}", insertion);
    }
  }

  nedbase::util::_thread_log("thread_terminated");
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
