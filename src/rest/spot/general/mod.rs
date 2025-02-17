use crate::model::EmptyResponse;
use crate::rest::endpoints::{Spot, API};
use crate::rest::inner_client::InnerClient;
use crate::rest::spot::general::model::{
  ExchangeInformationResponse, ServerTimeResponse, SymbolInformationResponse,
};
use anyhow::{bail, Result};

pub mod model;

#[derive(Clone)]
pub struct General {
  pub client: InnerClient,
}

impl General {
  /// Test connectivity
  pub async fn try_ping(&self) -> Result<String> {
    self
      .client
      .get::<EmptyResponse>(API::Spot(Spot::Ping), None)
      .await?;

    Ok("pong".into())
  }

  /// Check server time
  pub async fn fetch_server_time(&self) -> Result<ServerTimeResponse> {
    self.client.get(API::Spot(Spot::Time), None).await
  }

  /// Obtain exchange information.
  /// Current exchange trading rules and symbol information
  pub async fn fetch_exchange_info(&self) -> Result<ExchangeInformationResponse> {
    self.client.get(API::Spot(Spot::ExchangeInfo), None).await
  }

  /// Obtain exchange information.
  /// Current exchange trading rules and symbol information
  pub async fn list_symbols_info(&self) -> Result<Vec<SymbolInformationResponse>> {
    self
      .client
      .get::<ExchangeInformationResponse>(API::Spot(Spot::ExchangeInfo), None)
      .await
      .map(|r| r.symbols)
  }

  /// Symbol Trade Rules & information
  pub async fn fetch_symbol_info<S>(&self, symbol: S) -> Result<SymbolInformationResponse>
  where
    S: Into<String>,
  {
    let upper_symbol = symbol.into().to_uppercase();
    match self.fetch_exchange_info().await {
      Ok(info) => {
        for item in info.symbols {
          if item.symbol == upper_symbol {
            return Ok(item);
          }
        }
        bail!("Symbol not found")
      }
      Err(e) => Err(e),
    }
  }
}
