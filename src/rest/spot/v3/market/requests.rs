use crate::rest::spot::v3::market::enums::KlineInterval;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct KlinesRequest {
  pub symbol: String,
  pub interval: KlineInterval,
  pub start_time: Option<u64>,
  pub end_time: Option<u64>,
  pub limit: Option<u16>,
}

impl KlinesRequest {
  pub(crate) fn build_params_bree(self) -> BTreeMap<String, String> {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    parameters.insert("symbol".into(), self.symbol.into());
    parameters.insert("interval".into(), self.interval.to_string());

    // Add three optional parameters
    if let Some(lt) = self.limit {
      parameters.insert("limit".into(), format!("{}", lt));
    }
    if let Some(st) = self.start_time {
      parameters.insert("startTime".into(), format!("{}", st));
    }
    if let Some(et) = self.end_time {
      parameters.insert("endTime".into(), format!("{}", et));
    }

    parameters
  }
}
