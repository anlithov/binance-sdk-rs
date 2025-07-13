#![deny(
  unstable_features,
  unused_must_use,
  unused_mut,
  unused_imports,
  unused_import_braces
)]
#![allow(dead_code)]
pub mod client;
pub mod config;
mod errors;
pub mod model;
pub mod rest;
mod result;
pub(crate) mod serde_helpers;
mod util;
pub mod websocket_stream;
