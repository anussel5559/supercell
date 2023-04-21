use std::time::{Instant};

use super::run_fns::ChannelMessage;

pub fn _calc_extremes(res: &[ChannelMessage], start_time: &Instant) -> (f32, f32) {
  // x-axis is time, first value in our ret tuple
  let x_max = res.iter().fold(0_u128, |acm, msg| {
    let since_start = msg.request_at.duration_since(*start_time).as_millis();
    if since_start > acm {
      return since_start;
    }
    acm
  });

  let y_max = res.iter().fold(0_u128, |acm, msg| {
    if msg.response_time_ms > acm {
      return msg.response_time_ms;
    }
    acm
  });
  (x_max as f32, y_max as f32)
}

pub fn mean(numbers: &Vec<i32>) -> f32 {

  let sum: i32 = numbers.iter().sum();

  sum as f32 / numbers.len() as f32

}

pub fn median(numbers: &mut Vec<i32>) -> i32 {

  numbers.sort();

  let mid = numbers.len() / 2;
  if numbers.len() % 2 == 0 {
      mean(&vec![numbers[mid - 1], numbers[mid]]) as i32
  } else {
      numbers[mid]
  }

}

pub fn min_max(numbers: &[i32]) -> (f32, f64) {
  numbers.iter().fold((0_f32, std::f64::INFINITY), |mut acm, val| {
    if *val as f32 > acm.0 {
      acm.0 = *val as f32;
    }

    if (*val as f64) < acm.1 {
      acm.1 = *val as f64;
    }
    acm
  })
}