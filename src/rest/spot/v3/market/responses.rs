use crate::serde_helpers::string_to_float;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
  pub last_update_id: u64,
  pub bids: Vec<BidResponse>,
  pub asks: Vec<AskResponse>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct BidResponse {
  #[serde(with = "string_to_float")]
  pub price: f64,
  #[serde(with = "string_to_float")]
  pub qty: f64,
}

impl BidResponse {
  pub fn new(price: f64, qty: f64) -> BidResponse {
    BidResponse { price, qty }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AskResponse {
  #[serde(with = "string_to_float")]
  pub price: f64,
  #[serde(with = "string_to_float")]
  pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AggregatedTradeResponse {
  #[serde(rename = "T")]
  pub time: u64,
  #[serde(rename = "a")]
  pub agg_id: u64,
  #[serde(rename = "f")]
  pub first_id: u64,
  #[serde(rename = "l")]
  pub last_id: u64,
  #[serde(rename = "m")]
  pub maker: bool,
  #[serde(rename = "M")]
  pub best_match: bool,
  #[serde(rename = "p", with = "string_to_float")]
  pub price: f64,
  #[serde(rename = "q", with = "string_to_float")]
  pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KlineSummaryResponse {
  pub open_time: u64,
  pub open_price: String,
  pub high_price: String,
  pub low_price: String,
  pub close_price: String,
  pub volume: String,
  pub close_time: u64,
  pub quote_asset_volume: String,
  pub number_of_trades: i64,
  pub taker_buy_base_asset_volume: String,
  pub taker_buy_quote_asset_volume: String,
  pub taker: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TickerPriceResponse {
  pub symbol: String,
  #[serde(with = "string_to_float")]
  pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AveragePrice {
  pub mins: u64,
  #[serde(with = "string_to_float")]
  pub price: f64,
  pub close_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum BookTickersMultiResponse {
  AllBookTickers(Vec<BookTickerResponse>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerResponse {
  pub symbol: String,
  #[serde(with = "string_to_float")]
  pub bid_price: f64,
  #[serde(with = "string_to_float")]
  pub bid_qty: f64,
  #[serde(with = "string_to_float")]
  pub ask_price: f64,
  #[serde(with = "string_to_float")]
  pub ask_qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TickerDaySummaryResponse {
  pub symbol: String,
  pub price_change: String,
  pub price_change_percent: String,
  pub weighted_avg_price: String,
  #[serde(with = "string_to_float")]
  pub prev_close_price: f64,
  #[serde(with = "string_to_float")]
  pub last_price: f64,
  #[serde(with = "string_to_float")]
  pub bid_price: f64,
  #[serde(with = "string_to_float")]
  pub ask_price: f64,
  #[serde(with = "string_to_float")]
  pub open_price: f64,
  #[serde(with = "string_to_float")]
  pub high_price: f64,
  #[serde(with = "string_to_float")]
  pub low_price: f64,
  #[serde(with = "string_to_float")]
  pub volume: f64,
  pub open_time: u64,
  pub close_time: u64,
  pub first_id: i64,
  pub last_id: i64,
  pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeneralExchangeInfoResponse {
  pub timezone: String,
  pub server_time: u64,
  pub rate_limits: Vec<InstrumentRateLimitIntervalResponse>,
  pub symbols: Vec<InstrumentInfoResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentRateLimitIntervalResponse {
  pub rate_limit_type: String,
  pub interval: String,
  pub interval_num: u64,
  pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentInfoResponse {
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
  pub filters: Vec<InstrumentFilters>,
  pub permissions: Vec<String>,
  pub permission_sets: Vec<Vec<String>>,
  pub default_self_trade_prevention_mode: String,
  pub allowed_self_trade_prevention_modes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType")]
pub enum InstrumentFilters {
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
