use super::responses::{GeneralExchangeInfoResponse, InstrumentInfoResponse};
use crate::rest::endpoints::{SpotV3, API};
use crate::rest::spot::v3::market::SpotMarketV3Manager;
use crate::util::build_query;
use anyhow::Result;
use std::collections::BTreeMap;

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
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    parameters.insert("symbol".into(), symbol.into());

    let request = build_query(parameters);

    self
      .client
      .get::<GeneralExchangeInfoResponse>(API::SpotV3(SpotV3::ExchangeInfo), Some(request))
      .await
      .map(|r| r.symbols[0].clone())
  }
}
