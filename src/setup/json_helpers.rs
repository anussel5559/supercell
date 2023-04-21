use color_eyre::{eyre::{WrapErr, Result, eyre}};

use crate::structs::{TestConfig, Methods};

pub fn parse_test_config(input_path: &std::path::PathBuf) -> Result<TestConfig> {
  let file_contents = std::fs::read_to_string(input_path)
    .wrap_err_with(|| format!("Could not read file {}", input_path.display()))?;

  let config: TestConfig = serde_json::from_str(&file_contents)
    .wrap_err_with(|| "Failed to parse json in to expected schema!".to_string())?;

  Ok(config)
}

pub fn validate_test_config(config: &TestConfig) -> Result<()> {
  match &config.request.path_refs {
    None => (),
    Some(path_refs) => {
      if path_refs.len() != config.test_parameters.requests as usize {
        return Err(eyre!("The length of path_refs provided does not match the request amount in test_parameters"));
      }
    }
  }

  if config.test_parameters.threads > config.test_parameters.requests {
    return Err(eyre!("The number of threads requested is larger than requests to be made"));
  }

  if (config.request.method == Methods::POST || config.request.method == Methods::PATCH) && config.request.body.is_none() {
    return Err(eyre!("A POST or PATCH method requires a body, but no body was provided"));
  }
  Ok(())
}