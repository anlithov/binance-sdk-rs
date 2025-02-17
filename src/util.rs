use anyhow::bail;
use anyhow::Result;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn build_request(parameters: BTreeMap<String, String>) -> String {
  let mut request = String::new();
  for (key, value) in parameters {
    let param = format!("{}={}&", key, value);
    request.push_str(param.as_ref());
  }
  request.pop();
  request
}

pub fn build_signed_request(
  parameters: BTreeMap<String, String>,
  recv_window: u64,
) -> Result<String> {
  build_signed_request_custom(parameters, recv_window, SystemTime::now())
}

pub fn build_signed_request_custom(
  mut parameters: BTreeMap<String, String>,
  recv_window: u64,
  start: SystemTime,
) -> Result<String> {
  if recv_window > 0 {
    parameters.insert("recvWindow".into(), recv_window.to_string());
  }
  if let Ok(timestamp) = get_timestamp(start) {
    parameters.insert("timestamp".into(), timestamp.to_string());
    return Ok(build_request(parameters));
  }
  bail!("Failed to get timestamp")
}

fn get_timestamp(time: SystemTime) -> Result<u64> {
  Ok(
    time
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards")
      .as_millis() as u64,
  )
}

pub fn is_start_time_valid(start_time: &u64) -> bool {
  let current_time = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();

  if start_time > &current_time {
    false
  } else {
    true
  }
}

pub fn vec_to_string(vector: Vec<String>) -> String {
  format!(
    "[{}]",
    vector
      .iter()
      .map(|s| format!("\"{}\"", s))
      .collect::<Vec<_>>()
      .join(",")
  )
}
