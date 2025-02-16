use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BinanceContentError {
  pub code: i16,
  pub msg: String,
}
