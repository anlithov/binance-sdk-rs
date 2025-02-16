#![deny(
  unstable_features,
  unused_must_use,
  unused_mut,
  unused_imports,
  unused_import_braces,
  clippy::all
)]
#![allow(clippy::needless_doctest_main)]
#![warn(
  clippy::wildcard_imports,
  clippy::manual_string_new,
  clippy::single_match_else,
  clippy::implicit_clone,
  clippy::semicolon_if_nothing_returned
)]

pub mod client;
mod config;
mod errors;
pub mod model;
pub mod rest;
mod util;
pub mod websocket;
