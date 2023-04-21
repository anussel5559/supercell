use std::time::{Instant};
use comfy_table::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;

use super::run_fns::ChannelMessage;
use super::math;

fn pivot_res(res: &[ChannelMessage], start_time: Instant) -> Vec<(i32, Vec<(f32, f32)>)> {
  let mut reduced = res.iter().fold(vec![], |mut acmm, msg| {
    let threads_index = match acmm.iter().position(|&(id, _)| id == msg.thread_id) {
      None => {
        acmm.push((msg.thread_id, vec![] as Vec<(f32, f32)>));
        acmm.len() - 1
      },
      Some(index) => index
    };
    acmm[threads_index].1.push((
      msg.request_at.duration_since(start_time).as_millis() as f32,
      msg.response_time_ms as f32
    ));
    acmm
  });
  reduced.sort_by(|a, b| a.0.cmp(&b.0));
  reduced
}

pub fn plot_results(res: Vec<ChannelMessage>, start_time: Instant) {
  let data_pieces = pivot_res(&res, start_time);
  
  let mut table = Table::new();
  table
    .load_preset(UTF8_FULL)
    .apply_modifier(UTF8_ROUND_CORNERS)
    .set_content_arrangement(ContentArrangement::Dynamic)
    .set_header(vec!["Thread", "Min", "Median", "Mean", "Max", "Request Count"]);

  for thread_data in data_pieces {
    let mut raw_response_times: Vec<i32> = thread_data.1.iter().map(|(_, val)| {
      *val as i32
    }).collect();
    let thread_min_max = math::min_max(&raw_response_times);
    table.add_row(vec![
      Cell::new(thread_data.0),
      Cell::new(thread_min_max.1),
      Cell::new(math::median(&mut raw_response_times)),
      Cell::new(math::mean(&raw_response_times)),
      Cell::new(thread_min_max.0),
      Cell::new(thread_data.1.len())
    ]);
  }
  println!("{table}");
}