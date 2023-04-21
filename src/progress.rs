use indicatif::{ProgressBar, ProgressStyle, HumanDuration};
use std::{time::Duration};
use console::{style};

pub fn setup_spinner(progess_msg: String) -> ProgressBar {
  let pb = ProgressBar::new_spinner();
  setup_existing_spinner(progess_msg, &pb);
  pb
}

pub fn setup_existing_spinner(progress_msg: String, existing_pb: &ProgressBar) {
  let spinner_style = ProgressStyle::with_template("  {prefix:.bold.dim} {msg}{spinner}")
    .unwrap()
    .tick_strings(&[
      ".   ",
      " .  ",
      "  . ",
      "   .",
      ""
    ]);
  
  existing_pb.set_style(spinner_style);
  existing_pb.set_message(progress_msg);
  existing_pb.enable_steady_tick(Duration::from_millis(200));
}

pub fn finish_spinner(pb: &ProgressBar, complete_msg: String) {
  pb.set_prefix("\u{2705}");
  let duration_str = style(format!("({})", HumanDuration(pb.elapsed()))).bold().dim();
  pb.finish_with_message(format!("{} {}", complete_msg, duration_str));
}