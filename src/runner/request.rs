use reqwest::StatusCode;
use reqwest::blocking::Client;

use crate::structs;

fn url_builder(req_template: &structs::RequestConfig) -> String {
  format!("{}/{}", req_template.host, req_template.path)
}

pub fn make_request(req_client: &Client, req_template: &structs::RequestConfig) -> StatusCode {
  let url = url_builder(req_template);
  let req_builder = match req_template.method {
    structs::Methods::GET => req_client.get(&url),
    structs::Methods::POST => req_client.post(&url).json(&req_template.body),
    structs::Methods::DELETE => req_client.delete(&url),
    structs::Methods::PATCH => req_client.patch(&url).json(&req_template.body)
  };
  
  let req_builder = req_builder.header("Authorization", &req_template.header.authorization);

  let resp = req_builder.send();
  match resp {
    Ok(resp) => resp.status(),
    Err(resp) => {
      match resp.status() {
        Some(code) => code,
        None => StatusCode::OK
      }
    }
  }
}