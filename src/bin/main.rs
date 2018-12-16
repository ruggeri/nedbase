extern crate nedbase;
extern crate rand;

use nedbase::{BTree, LockSet, TransactionMode};
use std::sync::Arc;
use std::thread;

const MAX_KEYS_PER_NODE: usize = 1024;
const NUM_KEYS_PER_THREAD: u32 = 10_000;
const NUM_THREADS: u32 = 32;

fn main() {
  // Make the BTree.
  let btree = Arc::new(BTree::new(MAX_KEYS_PER_NODE));

  // Make the work.
  let keyset = {
    let mut keyset = vec![];
    for _ in 0..(NUM_THREADS * NUM_KEYS_PER_THREAD) {
      let key1 = btree.get_new_identifier();
      let key2 = btree.get_new_identifier();

      let pair = if key1 < key2 {
        (key1, key2)
      } else {
        (key2, key1)
      };

      keyset.push(pair);
    }

    keyset
  };

  let keyset = Arc::new(keyset);

  // Spawn a bunch of threads to hammer the BTree.
  let mut join_handles = vec![];
  for _ in 0..NUM_THREADS {
    join_handles.push({
      let btree = Arc::clone(&btree);
      let keyset = Arc::clone(&keyset);
      thread::spawn(move || run_thread(&btree, keyset))
    });
  }

  // Wait for them to all finish.
  for handle in join_handles {
    handle.join().expect("no threads should panic");
  }
}

// A thread's work.
fn run_thread(btree: &Arc<BTree>, keyset: Arc<Vec<(String, String)>>) {
  // First, shuffle the keys.
  let keyset = {
    let mut rng = thread_rng();
    let mut keyset = (*keyset).clone();
    keyset.shuffle(&mut rng);
    use rand::prelude::*;

    keyset
  };

  for (idx, (key1, key2)) in keyset.into_iter().enumerate() {
    if idx % 3 == 0 {
      let mut lock_set = LockSet::new(btree, TransactionMode::ReadWrite);
      BTree::optimistic_insert(btree, &mut lock_set, &key1);
      BTree::optimistic_insert(btree, &mut lock_set, &key2);

      let key1_present = BTree::contains_key(&mut lock_set, &key1);
      let key2_present = BTree::contains_key(&mut lock_set, &key2);
      if !key1_present || !key2_present {
        println!("A key wasn't inserted?");
      }
    } else if idx % 3 == 1 {
      let mut lock_set = LockSet::new(btree, TransactionMode::ReadWrite);
      BTree::delete(btree, &mut lock_set, &key1);
      BTree::delete(btree, &mut lock_set, &key2);

      let key1_present = BTree::contains_key(&mut lock_set, &key1);
      let key2_present = BTree::contains_key(&mut lock_set, &key2);
      if key1_present || key2_present {
        println!("A key wasn't deleted?");
      }
    } else {
      let mut lock_set = LockSet::new(btree, TransactionMode::ReadOnly);
      let key1_present = BTree::contains_key(&mut lock_set, &key1);
      let key2_present = BTree::contains_key(&mut lock_set, &key2);

      if key1_present != key2_present {
        println!("Read transaction isolation violated!");
      }
    }
  }
}
