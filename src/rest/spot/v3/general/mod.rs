use crate::model::EmptyResponse;
use crate::rest::core::inner_client::InnerClient;
use crate::rest::endpoints::{SpotV3, API};
use crate::rest::spot::v3::general::responses::ServerTimeResponse;
use anyhow::Result;

pub mod responses;

#[derive(Clone)]
pub struct GeneralManagerV3 {
  pub(crate) client: InnerClient,
  pub(crate) recv_window: u64,
}

impl GeneralManagerV3 {
  /// Test connectivity
  pub async fn try_ping(&self) -> Result<String> {
    self
      .client
      .get::<EmptyResponse>(API::SpotV3(SpotV3::Ping), None)
      .await?;

    Ok("pong".into())
  }

  /// Check server time
  pub async fn fetch_server_time(&self) -> Result<ServerTimeResponse> {
    self.client.get(API::SpotV3(SpotV3::Time), None).await
  }
}
