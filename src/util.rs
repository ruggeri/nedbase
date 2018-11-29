use std::thread;

pub fn thread_log(msg: &str) {
  let thread_id = thread::current().id();
  println!("{:?}: {}", thread_id, msg);
}
