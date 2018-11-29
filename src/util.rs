use std::thread;

pub fn thread_log(msg: &str) {
  let thread_id = thread::current().id();
  #[cfg(debug_assertions)]
  println!("{:?}: {}", thread_id, msg);
}
