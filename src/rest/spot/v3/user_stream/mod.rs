use crate::rest::endpoints::{SpotV3, API};
use crate::rest::spot::v3::user_stream::model::{Success, UserDataStream};
use anyhow::Result;

use crate::rest::inner_client::InnerClient;

pub mod model;

#[derive(Clone)]
pub struct SpotUserStreamManagerV3 {
  pub(crate) client: InnerClient,
  pub(crate) recv_window: u64,
}

impl SpotUserStreamManagerV3 {
  // User Stream
  pub async fn start(&self) -> Result<UserDataStream> {
    self.client.post(API::SpotV3(SpotV3::UserDataStream)).await
  }

  // Current open orders on a symbol
  pub async fn keep_alive(&self, listen_key: &str) -> Result<Success> {
    self
      .client
      .put(API::SpotV3(SpotV3::UserDataStream), listen_key)
      .await
  }

  pub async fn close(&self, listen_key: &str) -> Result<Success> {
    self
      .client
      .delete(API::SpotV3(SpotV3::UserDataStream), listen_key)
      .await
  }
}
