use super::SpotTradeV3Manager;
use crate::rest::endpoints::{SpotV3, API};
use crate::rest::spot::trade_v3::responses::StpRecordResponse;
use crate::util::build_signed_query;
use anyhow::Result;
use std::collections::BTreeMap;

impl SpotTradeV3Manager {
  /// Find order that was expired because of Self Trade Prevention (STP).
  pub async fn fetch_stp_order_record<S, O>(
    &self,
    symbol: S,
    order_id: O,
  ) -> Result<Option<StpRecordResponse>>
  where
    S: Into<String>,
    O: Into<u64>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    parameters.insert("orderId".into(), order_id.into().to_string());

    self.fetch_stp_order_record_by_params(parameters).await
  }

  /// Find order that was expired because of Self Trade Prevention (STP).
  pub async fn fetch_stp_order_record_by_prevented_match_id<S, P>(
    &self,
    symbol: S,
    prevented_match_id: P,
  ) -> Result<Option<StpRecordResponse>>
  where
    S: Into<String>,
    P: Into<u64>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    parameters.insert(
      "preventedMatchId".into(),
      prevented_match_id.into().to_string(),
    );

    self.fetch_stp_order_record_by_params(parameters).await
  }

  async fn fetch_stp_order_record_by_params(
    &self,
    parameters: BTreeMap<String, String>,
  ) -> Result<Option<StpRecordResponse>> {
    let request = build_signed_query(parameters, self.recv_window)?;
    match self
      .client
      .get_signed::<Vec<StpRecordResponse>>(API::SpotV3(SpotV3::MyPreventedMatches), Some(request))
      .await
    {
      Ok(res) => {
        if res.is_empty() {
          Ok(None)
        } else {
          Ok(Some(res[0].clone()))
        }
      }
      Err(e) => Err(e),
    }
  }
}
