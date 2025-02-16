use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
  pub server_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformationResponse {
  pub timezone: String,
  pub server_time: u64,
  pub rate_limits: Vec<SymbolRateLimitIntervalResponse>,
  pub symbols: Vec<SymbolInformationResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SymbolRateLimitIntervalResponse {
  pub rate_limit_type: String,
  pub interval: String,
  pub interval_num: u64,
  pub limit: u64,
  pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInformationResponse {
  pub symbol: String,
  pub status: String,
  pub base_asset: String,
  pub base_asset_precision: u64,
  pub quote_asset: String,
  pub quote_precision: u64,
  pub order_types: Vec<String>,
  pub iceberg_allowed: bool,
  pub oco_allowed: bool,
  pub oto_allowed: bool,
  pub quote_order_qty_market_allowed: bool,
  pub allow_trailing_stop: bool,
  pub cancel_replace_allowed: bool,
  pub is_spot_trading_allowed: bool,
  pub is_margin_trading_allowed: bool,
  pub filters: Vec<Filters>,
  pub permissions: Vec<String>,
  pub permission_sets: Vec<Vec<String>>,
  pub default_self_trade_prevention_mode: String,
  pub allowed_self_trade_prevention_modes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType")]
pub enum Filters {
  #[serde(rename = "PRICE_FILTER")]
  #[serde(rename_all = "camelCase")]
  PriceFilter {
    min_price: String,
    max_price: String,
    tick_size: String,
  },
  #[serde(rename = "PERCENT_PRICE")]
  #[serde(rename_all = "camelCase")]
  PercentPrice {
    multiplier_up: String,
    multiplier_down: String,
    avg_price_mins: Option<f64>,
  },
  #[serde(rename = "PERCENT_PRICE_BY_SIDE")]
  #[serde(rename_all = "camelCase")]
  PercentPriceBySide {
    bid_multiplier_up: String,
    bid_multiplier_down: String,
    ask_multiplier_up: String,
    ask_multiplier_down: String,
    avg_price_mins: Option<f64>,
  },
  #[serde(rename = "LOT_SIZE")]
  #[serde(rename_all = "camelCase")]
  LotSize {
    min_qty: String,
    max_qty: String,
    step_size: String,
  },
  #[serde(rename = "MIN_NOTIONAL")]
  #[serde(rename_all = "camelCase")]
  MinNotional {
    notional: Option<String>,
    min_notional: Option<String>,
    apply_to_market: Option<bool>,
    avg_price_mins: Option<f64>,
  },
  #[serde(rename = "NOTIONAL")]
  #[serde(rename_all = "camelCase")]
  Notional {
    notional: Option<String>,
    min_notional: Option<String>,
    apply_to_market: Option<bool>,
    avg_price_mins: Option<f64>,
  },
  #[serde(rename = "ICEBERG_PARTS")]
  #[serde(rename_all = "camelCase")]
  IcebergParts { limit: Option<u16> },
  #[serde(rename = "MAX_NUM_ORDERS")]
  #[serde(rename_all = "camelCase")]
  MaxNumOrders { max_num_orders: Option<u16> },
  #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
  #[serde(rename_all = "camelCase")]
  MaxNumAlgoOrders { max_num_algo_orders: Option<u16> },
  #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
  #[serde(rename_all = "camelCase")]
  MaxNumIcebergOrders { max_num_iceberg_orders: u16 },
  #[serde(rename = "MAX_POSITION")]
  #[serde(rename_all = "camelCase")]
  MaxPosition { max_position: String },
  #[serde(rename = "MARKET_LOT_SIZE")]
  #[serde(rename_all = "camelCase")]
  MarketLotSize {
    min_qty: String,
    max_qty: String,
    step_size: String,
  },
  #[serde(rename = "TRAILING_DELTA")]
  #[serde(rename_all = "camelCase")]
  TrailingData {
    min_trailing_above_delta: Option<u16>,
    max_trailing_above_delta: Option<u16>,
    min_trailing_below_delta: Option<u16>,
    max_trailing_below_delta: Option<u16>,
  },
}
