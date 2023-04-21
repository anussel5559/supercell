#![allow(clippy::upper_case_acronyms)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TestConfig {
  pub request: RequestConfig,
  pub test_parameters: LoadTestConfig
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum Methods {
  GET,
  POST,
  PATCH,
  DELETE
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RequestConfig {
  pub method: Methods,
  pub host: String,
  pub path: String,
  pub header: RequestHeader,
  pub body: Option<HashMap<String, Value>>,
  // pathRefs is an optional array of variable tuples
  pub path_refs: Option<Vec<HashMap<String, String>>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RequestHeader {
  pub authorization: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoadTestConfig {
  pub threads: i32,
  pub requests: i32,
  pub timeout_ms: i32
}