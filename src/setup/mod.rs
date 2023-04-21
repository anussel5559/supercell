use color_eyre::eyre::{WrapErr, Result};
use clap::Parser;

use crate::progress;
use crate::structs::TestConfig;

mod json_helpers;

#[derive(Parser)]
struct Cli {
    file_path: std::path::PathBuf
}

pub fn setup_load_test() -> Result<TestConfig> {
  let args = Cli::parse();
  let tab = "  ";
  
  println!("\n{tab}==== Setup ====");
  // Bring in the file and then attempt to parse it to our TestConfig struct so we
  // know what to do with it
  let parse_pb = progress::setup_spinner("  Parsing input file JSON".into());
  let test_config = json_helpers::parse_test_config(&args.file_path)?;
  progress::finish_spinner(&parse_pb, "Parsed input file JSON".into());

  let validate_pb = progress::setup_spinner("  Validating input configuration".into());
  json_helpers::validate_test_config(&test_config)
      .wrap_err_with(|| "Failed to validate test configuration!".to_string())?;
  progress::finish_spinner(&validate_pb, "Validated load test configuration".into());

  Ok(test_config)
}