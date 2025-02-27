use super::SpotMarketV3Manager;
use crate::rest::endpoints::{SpotV3, API};
use crate::rest::spot::v3::market::enums::KlineInterval;
use crate::rest::spot::v3::market::requests::KlinesRequest;
use crate::rest::spot::v3::market::responses::KlineSummaryResponse;
use crate::util::build_query;
use anyhow::Result;

impl SpotMarketV3Manager {
  /// Returns klines for given symbol and interval ("1m", "5m", ...)
  /// from some time to now
  pub async fn list_klines_from_time<S, STR>(
    &self,
    symbol: S,
    interval: KlineInterval,
    start_time: STR,
  ) -> Result<Vec<KlineSummaryResponse>>
  where
    S: Into<String>,
    STR: Into<u64>,
  {
    let mut request = KlinesRequest::default();
    request.symbol = symbol.into();
    request.interval = interval;
    request.start_time = Some(start_time.into());
    request.limit = Some(1000);

    self.list_klines_custom(request).await
  }

  /// Returns klines for given symbol and interval ("1m", "5m", ...)
  /// use KlineInterval enum
  /// https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#klinecandlestick-data
  pub async fn list_klines_custom(
    &self,
    request: KlinesRequest,
  ) -> Result<Vec<KlineSummaryResponse>> {
    let params_btree = request.build_params_bree();
    let query = build_query(params_btree);

    self
      .client
      .get(API::SpotV3(SpotV3::Klines), Some(query))
      .await
  }
}
