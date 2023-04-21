use color_eyre::eyre::{Result};
use std::time::{Instant};
use indicatif::{HumanDuration};
use console::{style};

mod progress;
mod runner;
mod setup;
mod structs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    println!("\n\n---- Beginning Load Test Run ----");
    let started = Instant::now();

    let test_config = setup::setup_load_test()?;

    runner::run(test_config)?;
    println!("\n\n\u{26c8}\u{1f32a}  Complete! {}", style(format!("(time: {})", HumanDuration(started.elapsed()))).bold().dim());
    Ok(())
}
