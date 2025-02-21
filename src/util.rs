use anyhow::bail;
use anyhow::Result;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn build_query(parameters: BTreeMap<String, String>) -> String {
  let mut request = String::new();
  for (key, value) in parameters {
    let param = format!("{}={}&", key, value);
    request.push_str(param.as_ref());
  }
  request.pop();
  request
}

pub fn build_signed_query(
  parameters: BTreeMap<String, String>,
  recv_window: u64,
) -> Result<String> {
  build_signed_query_custom(parameters, recv_window, SystemTime::now())
}

pub fn build_signed_query_custom(
  mut parameters: BTreeMap<String, String>,
  recv_window: u64,
  timestamp: SystemTime,
) -> Result<String> {
  if recv_window > 0 {
    parameters.insert("recvWindow".into(), recv_window.to_string());
  }
  if let Ok(timestamp) = get_timestamp(timestamp) {
    parameters.insert("timestamp".into(), timestamp.to_string());
    return Ok(build_query(parameters));
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

  start_time <= &current_time
}

pub fn vec_to_string_array(vector: Vec<String>) -> String {
  format!(
    "[{}]",
    vector
      .iter()
      .map(|s| format!("\"{}\"", s))
      .collect::<Vec<_>>()
      .join(",")
  )
}

#[macro_export]
macro_rules! create_enum_with_fmt {
  // We handle the first variant separately, then the remaining variants in a list.
  // This way, we can easily assign the first variant as the enum's default.
  (
    $name:ident, {
      $first_variant:ident => $first_display:expr
      $(, $variant:ident => $display:expr)* $(,)?
    }
  ) => {
    #[derive(Debug, Clone, Copy)]
    pub enum $name {
      $first_variant,
      $($variant),*
    }

    /// `Default` returns the **first** variant from the macro's list.
    impl Default for $name {
      fn default() -> Self {
        $name::$first_variant
      }
    }

    impl $name {
      /// Convert a 1-based index to the corresponding variant.
      /// Returns `None` if index is out of range.
      pub fn from_int(value: i32) -> Option<Self> {
        // Check the first variant
        let mut index = 1;
        if index == 1 {
          return Some($name::$first_variant);
        }
        // Start indexing for the rest of the variants at 2
        $(
        index += 1;
        if value == index {
          return Some($name::$variant);
        }
        )*
        None
      }
    }

    impl std::fmt::Display for $name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          $name::$first_variant => write!(f, "{}", $first_display),
          $(
          $name::$variant => write!(f, "{}", $display),
          )*
        }
      }
    }
  }
}
