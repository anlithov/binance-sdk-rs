use crate::model::EmptyResponse;
use crate::rest::endpoints::{Spot, API};
use crate::rest::inner_client::InnerClientRest;
use crate::rest::spot::general::model::{
  ExchangeInformationResponse, ServerTimeResponse, SymbolInformationResponse,
};
use anyhow::{bail, Result};

pub mod model;

#[derive(Clone)]
pub struct General {
  pub client: InnerClientRest,
}

impl General {
  /// Test connectivity
  pub async fn ping(&self) -> Result<String> {
    self
      .client
      .get::<EmptyResponse>(API::Spot(Spot::Ping), None)
      .await?;

    Ok("pong".into())
  }

  /// Check server time
  pub async fn server_time(&self) -> Result<ServerTimeResponse> {
    self.client.get(API::Spot(Spot::Time), None).await
  }

  /// Obtain exchange information.
  /// Current exchange trading rules and symbol information
  pub async fn exchange_info(&self) -> Result<ExchangeInformationResponse> {
    self.client.get(API::Spot(Spot::ExchangeInfo), None).await
  }

  /// Symbol Trade Rules & information
  pub async fn symbol_info<S>(&self, symbol: S) -> Result<SymbolInformationResponse>
  where
    S: Into<String>,
  {
    let upper_symbol = symbol.into().to_uppercase();
    match self.exchange_info().await {
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
