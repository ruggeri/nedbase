extern crate nedbase;
extern crate rand;

use nedbase::{BTree, LockSet, TransactionMode};
use std::sync::Arc;
use std::thread;

const MAX_KEYS_PER_NODE: usize = 32;
const NUM_KEYS: usize =
  MAX_KEYS_PER_NODE * MAX_KEYS_PER_NODE * MAX_KEYS_PER_NODE;
const NUM_THREADS: u32 = 32;

fn main() {
  // Make the BTree.
  let btree = Arc::new(BTree::new(MAX_KEYS_PER_NODE));

  // Make the work.
  let keyset = {
    let mut keyset = vec![];
    for _ in 0..NUM_KEYS {
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

  // Check no keys were lost.
  let keyset = (*keyset).clone();
  for (key1, key2) in keyset.into_iter() {
    let mut lock_set = LockSet::new(&btree, TransactionMode::ReadOnly);
    let key1_present = BTree::contains_key(&mut lock_set, &key1);
    let key2_present = BTree::contains_key(&mut lock_set, &key2);

    if !key1_present || !key2_present {
      println!("We lost a key?!");
    }
  }

  // Finally, validate the structure of the tree.
  let mut lock_set = LockSet::new(&btree, TransactionMode::ReadOnly);
  btree.validate(&mut lock_set)
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

  let third_of_keyset = keyset.len() / 3;

  for idx in 0..keyset.len() {
    {
      // Do a transaction of two insertions.
      let (key1, key2) = keyset[idx].clone();
      let mut lock_set =
        LockSet::new(btree, TransactionMode::ReadWrite);

      BTree::insert(btree, &mut lock_set, &key1);
      let key1_present = BTree::contains_key(&mut lock_set, &key1);
      if !key1_present {
        println!("Where did key1 go? {}", key1);
      }

      BTree::insert(btree, &mut lock_set, &key2);
      let key2_present = BTree::contains_key(&mut lock_set, &key2);
      if !key2_present {
        println!("Where did key2 go? {}", key2);
      }
    }

    {
      // Do a transaction of two reads.
      let idx = (idx + third_of_keyset) % keyset.len();
      let (key1, key2) = keyset[idx].clone();
      let mut lock_set = LockSet::new(btree, TransactionMode::ReadOnly);
      let key1_present = BTree::contains_key(&mut lock_set, &key1);
      let key2_present = BTree::contains_key(&mut lock_set, &key2);

      if key1_present != key2_present {
        println!("Read transaction isolation violated!");
      }
    }

    {
      // let idx = (idx + 2*third_of_keyset) % keyset.len();
      // let (key1, key2) = keyset[idx].clone();

      // let mut lock_set = LockSet::new(btree, TransactionMode::ReadWrite);
      // BTree::delete(btree, &mut lock_set, &key1);
      // BTree::delete(btree, &mut lock_set, &key2);

      // let key1_present = BTree::contains_key(&mut lock_set, &key1);
      // let key2_present = BTree::contains_key(&mut lock_set, &key2);
      // if key1_present || key2_present {
      //   println!("A key wasn't deleted?");
      // }
    }
  }
}
