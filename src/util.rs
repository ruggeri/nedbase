use std::thread;

pub fn log_node_map_locking(msg: &str) {
  let thread_id = thread::current().id();
  #[cfg(debug_assertions)]
  println!("{:?}: [node_map] {}", thread_id, msg);
}

pub fn log_method_entry(msg: &str) {
  let thread_id = thread::current().id();
  #[cfg(debug_assertions)]
  println!("{:?}: [log_method_entry] {}", thread_id, msg);
}

pub fn log_node_locking(msg: &str) {
  let thread_id = thread::current().id();
  #[cfg(debug_assertions)]
  println!("{:?}: [log_node_locking] {}", thread_id, msg);
}

pub fn log_root_locking(msg: &str) {
  let thread_id = thread::current().id();
  #[cfg(debug_assertions)]
  println!("{:?}: [log_root_locking] {}", thread_id, msg);
}

pub fn _thread_log(msg: &str) {
  let thread_id = thread::current().id();
  #[cfg(debug_assertions)]
  println!("{:?}: [junk] {}", thread_id, msg);
}
