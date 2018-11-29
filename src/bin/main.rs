extern crate nedbase;

use nedbase::BTree;

fn main() {
  let btree = BTree::new(8);

  let mut insertions = vec![];
  for _ in 0..100_000 {
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
