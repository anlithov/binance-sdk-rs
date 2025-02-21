use super::responses::{GeneralExchangeInfoResponse, InstrumentInfoResponse};
use crate::rest::endpoints::{SpotV3, API};
use crate::rest::spot::market_v3::SpotMarketV3Manager;
use anyhow::{bail, Result};

impl SpotMarketV3Manager {
  /// Obtain exchange information.
  /// Current exchange trading rules and symbol information
  pub async fn fetch_general_exchange_info(&self) -> Result<GeneralExchangeInfoResponse> {
    self
      .client
      .get(API::SpotV3(SpotV3::ExchangeInfo), None)
      .await
  }

  /// Obtain exchange information.
  /// Current exchange trading rules and symbol information
  pub async fn list_instruments_info(&self) -> Result<Vec<InstrumentInfoResponse>> {
    self
      .client
      .get::<GeneralExchangeInfoResponse>(API::SpotV3(SpotV3::ExchangeInfo), None)
      .await
      .map(|r| r.symbols)
  }

  /// Symbol Trade Rules & information
  pub async fn fetch_instrument_info<S>(&self, symbol: S) -> Result<InstrumentInfoResponse>
  where
    S: Into<String>,
  {
    let upper_symbol = symbol.into().to_uppercase();
    match self.list_instruments_info().await {
      Ok(symbols) => {
        for item in symbols {
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
