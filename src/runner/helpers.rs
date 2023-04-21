use std::sync::{Arc, Mutex};

pub fn pull_request_to_work(req_count: &Arc<Mutex<i32>>, limit: &i32) -> Option<i32> {
  let mut cur_count = req_count.lock().unwrap();
  if *cur_count >= *limit {
    return None;
  }

  *cur_count += 1;
  let req_number = *cur_count;
  drop(cur_count);
  Some(req_number)
}