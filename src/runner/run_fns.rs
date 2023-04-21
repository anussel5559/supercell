use color_eyre::eyre::{Result};
use reqwest::StatusCode;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Instant, Duration};
use std::cmp::Ordering;
use indicatif::{MultiProgress, ProgressBar};
use reqwest::blocking::Client;

use crate::structs::TestConfig;
use crate::progress;

use super::helpers;
use super::chart;
use super::request;

pub struct ChannelMessage {
  pub thread_id: i32,
  pub req_number: i32,
  pub response_time_ms: u128,
  pub request_at: Instant,
  pub resp_status: StatusCode
}

fn req_consumer(config: &TestConfig, thread_id: &i32, count: &Arc<Mutex<i32>>, limit: &i32, tx: mpsc::Sender<ChannelMessage>) {
  let client = Client::builder()
    .timeout(Duration::from_millis(config.test_parameters.timeout_ms as u64))
    .tcp_keepalive(Duration::from_secs(30))
    .build().unwrap();

  while let Some(req_number) = helpers::pull_request_to_work(count, limit) {
    let req_timer = Instant::now();
  
    let resp_status = request::make_request(&client, &config.request);
    tx.send(ChannelMessage { 
      thread_id: *thread_id,
      response_time_ms: req_timer.elapsed().as_millis(),
      req_number,
      request_at: req_timer,
      resp_status
    }).unwrap();
    // work the next one
  }
}

fn maybe_pb(ord: &Ordering, m: &MultiProgress, init_msg: String) -> Option<ProgressBar> {
  let pb = match ord {
    Ordering::Less => { Some(m.add(ProgressBar::new_spinner())) },
    Ordering::Equal => { Some(m.add(ProgressBar::new_spinner())) },
    Ordering::Greater => { None }
  };

  if pb.is_some() {
    let pb = pb.clone().unwrap();
    progress::setup_existing_spinner(init_msg, &pb);
  }

  pb
}

pub fn run(config: TestConfig) -> Result<()> {
  println!("\n\n  ==== Run ====");
  let run_start = Instant::now();
  let req_count = Arc::new(Mutex::new(0));
  let m = MultiProgress::new();
  let (tx, rx) = mpsc::channel();

  // if the threads is large, setup a single spinner to let user know we're working
  let overall_pb = maybe_pb(&(11).cmp(&config.test_parameters.threads), &m, "Threads running".into());

  // go and make our threads, store their handles so we can ensure all are complete later
  let mut handles = vec![];
  for i in 1..=config.test_parameters.threads {
    let config = config.clone();
    let my_pb = maybe_pb(&config.test_parameters.threads.cmp(&10), &m, format!("Thread {} running", i));
    let tx = tx.clone();
    let shared_count = Arc::clone(&req_count);

    handles.push(thread::spawn(move || {
      req_consumer(&config, &i, &shared_count, &config.test_parameters.requests, tx);
      if let Some(my_pb) = my_pb {
        progress::finish_spinner(&my_pb, format!("Thread {} complete", i));
      }
    }));
  }
  drop(tx);

  // push individual request results in to an array here as our threads make the requests
  let mut results = vec![];
  while let Ok(msg) = rx.recv() {
    results.push(msg);
  }

  // make sure all the threads are done before continuing
  for h in handles {
    let _ = h.join();
  }
  // finish up the "single spinner" for large thread runs
  if let Some(overall_pb) = overall_pb {
    progress::finish_spinner(&overall_pb, "Threads complete".into());
  }

  println!("  ==== Total requests made: {} ====", req_count.lock().unwrap());
  if config.test_parameters.threads <= 20 && config.test_parameters.requests < 100000 {
    chart::plot_results(results, run_start);
  }

  Ok(())
}