use super::{KlineInterval, Market};
use crate::rest::endpoints::{Spot, API};
use crate::rest::spot::market::model::{KlineSummariesResponse, KlineSummaryResponse};
use crate::util::build_request;
use anyhow::Result;
use serde_json::Value;
use std::collections::BTreeMap;

impl Market {
  /// Returns klines for given symbol and interval ("1m", "5m", ...)
  /// from some time to now
  pub async fn fetch_klines_from_time<S, STR>(
    &self,
    symbol: S,
    interval: KlineInterval,
    start_time: STR,
  ) -> Result<KlineSummariesResponse>
  where
    S: Into<String>,
    STR: Into<u64>,
  {
    self
      .fetch_klines_custom(symbol.into(), interval, start_time.into(), None, 1000)
      .await
  }

  /// Returns klines for given symbol and interval ("1m", "5m", ...)
  /// use KlineInterval enum
  /// https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#klinecandlestick-data
  pub async fn fetch_klines_custom<S, L, STR, END>(
    &self,
    symbol: S,
    interval: KlineInterval,
    start_time: STR,
    end_time: END,
    limit: L,
  ) -> Result<KlineSummariesResponse>
  where
    S: Into<String>,
    STR: Into<Option<u64>>,
    END: Into<Option<u64>>,
    L: Into<Option<u16>>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    parameters.insert("symbol".into(), symbol.into());
    parameters.insert("interval".into(), interval.to_string());

    // Add three optional parameters
    if let Some(lt) = limit.into() {
      parameters.insert("limit".into(), format!("{}", lt));
    }
    if let Some(st) = start_time.into() {
      parameters.insert("startTime".into(), format!("{}", st));
    }
    if let Some(et) = end_time.into() {
      parameters.insert("endTime".into(), format!("{}", et));
    }

    let request = build_request(parameters);
    let data: Vec<Vec<Value>> = self
      .client
      .get(API::Spot(Spot::Klines), Some(request))
      .await?;

    let klines = KlineSummariesResponse::AllKlineSummaries(
      data
        .iter()
        .map(|row| row.try_into())
        .collect::<Result<Vec<KlineSummaryResponse>>>()?,
    );

    Ok(klines)
  }
}
