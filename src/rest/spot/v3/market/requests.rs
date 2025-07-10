use crate::rest::spot::v3::market::enums::{
  ExchangeSymbolPermission, ExchangeSymbolStatus, KlineInterval,
};
use crate::util::vec_to_string_array;
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

#[derive(Default)]
pub struct ExchangeInfoRequest {
  pub symbol: Option<String>,
  pub symbols: Option<Vec<String>>,
  pub permissions: Option<Vec<ExchangeSymbolPermission>>,
  pub show_permission_sets: Option<bool>,
  pub symbol_status: Option<ExchangeSymbolStatus>,
}

impl ExchangeInfoRequest {
  pub(crate) fn build_params_bree(self) -> BTreeMap<String, String> {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    if let Some(smb) = self.symbol {
      parameters.insert("symbol".into(), format!("{}", smb));
    }
    if let Some(smbs) = self.symbols {
      let str_symbols = vec_to_string_array(smbs);
      parameters.insert("symbols".into(), format!("{}", str_symbols));
    }
    if let Some(perms) = self.permissions {
      let str_symbols = vec_to_string_array(perms.iter().map(|p| p.to_string()).collect());
      parameters.insert("permissions".into(), format!("{}", str_symbols));
    }
    if let Some(sps) = self.show_permission_sets {
      parameters.insert("showPermissionSets".into(), format!("{}", sps));
    }
    if let Some(ss) = self.symbol_status {
      parameters.insert("symbolStatus".into(), format!("{}", ss.to_string()));
    }

    parameters
  }
}
