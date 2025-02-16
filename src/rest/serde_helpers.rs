pub(crate) mod string_or_float {
  use std::fmt;

  use serde::{de, Deserialize, Deserializer, Serializer};

  #[allow(unused)]
  pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
  where
    T: fmt::Display,
    S: Serializer,
  {
    serializer.collect_str(value)
  }

  #[allow(unused)]
  pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrFloat {
      String(String),
      Float(f64),
    }

    match StringOrFloat::deserialize(deserializer)? {
      StringOrFloat::String(s) => {
        if s == "INF" {
          Ok(f64::INFINITY)
        } else {
          s.parse().map_err(de::Error::custom)
        }
      }
      StringOrFloat::Float(i) => Ok(i),
    }
  }
}

pub(crate) mod string_or_float_opt {
  use serde::{Deserialize, Deserializer, Serializer};
  use std::fmt;

  #[allow(unused)]
  pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
  where
    T: fmt::Display,
    S: Serializer,
  {
    match value {
      Some(v) => crate::rest::serde_helpers::string_or_float::serialize(v, serializer),
      None => serializer.serialize_none(),
    }
  }

  #[allow(unused)]
  pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    #[allow(dead_code)]
    #[serde(untagged)]
    enum StringOrFloat {
      String(String),
      Float(f64),
    }

    Ok(Some(
      crate::rest::serde_helpers::string_or_float::deserialize(deserializer)?,
    ))
  }
}

pub(crate) mod string_or_bool {
  use std::fmt;

  use serde::{de, Deserialize, Deserializer, Serializer};

  #[allow(unused)]
  pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
  where
    T: fmt::Display,
    S: Serializer,
  {
    serializer.collect_str(value)
  }

  #[allow(unused)]
  pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    #[allow(dead_code)]
    #[serde(untagged)]
    enum StringOrFloat {
      String(String),
      Bool(bool),
    }

    match StringOrFloat::deserialize(deserializer)? {
      StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
      StringOrFloat::Bool(i) => Ok(i),
    }
  }
}

pub(crate) fn default_stop_price() -> f64 {
  0.0
}
