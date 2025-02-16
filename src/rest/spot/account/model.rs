use crate::rest::serde_helpers::{default_stop_price, string_or_float};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformationResponse {
  pub maker_commission: f32,
  pub taker_commission: f32,
  pub buyer_commission: f32,
  pub seller_commission: f32,
  pub commission_rates: CommissionRatesShortResponse,
  pub can_trade: bool,
  pub can_withdraw: bool,
  pub can_deposit: bool,
  pub require_self_trade_prevention: bool,
  pub prevent_sor: bool,
  pub update_time: u64,
  pub account_type: String,
  pub balances: Vec<AssetBalanceResponse>,
  pub uid: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRatesShortResponse {
  #[serde(with = "string_or_float")]
  pub maker: f64,
  #[serde(with = "string_or_float")]
  pub taker: f64,
  #[serde(with = "string_or_float")]
  pub seller: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetBalanceResponse {
  pub asset: String,
  #[serde(with = "string_or_float")]
  pub free: f64,
  #[serde(with = "string_or_float")]
  pub locked: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
  pub symbol: String,
  pub order_id: u64,
  pub order_list_id: i64,
  pub client_order_id: String,
  #[serde(with = "string_or_float")]
  pub price: f64,
  #[serde(with = "string_or_float")]
  pub orig_qty: f64,
  #[serde(with = "string_or_float")]
  pub executed_qty: f64,
  #[serde(with = "string_or_float")]
  pub cummulative_quote_qty: f64,
  pub status: String,
  pub time_in_force: String,
  #[serde(rename = "type")]
  pub type_name: String,
  pub side: String,
  #[serde(with = "string_or_float")]
  pub stop_price: f64,
  pub iceberg_qty: String,
  pub time: u64,
  pub update_time: u64,
  pub is_working: bool,
  pub working_time: u64,
  pub orig_quote_order_qty: String,
  pub self_trade_prevention_mode: String,
  pub prevented_match_id: Option<i64>,
  pub prevented_quantity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderCanceledResponse {
  pub symbol: String,
  pub orig_client_order_id: String,
  pub order_id: u64,
  pub order_list_id: i64,
  pub client_order_id: String,
  pub transact_time: i64,
  #[serde(with = "string_or_float")]
  pub price: f64,
  #[serde(with = "string_or_float")]
  pub orig_qty: f64,
  #[serde(with = "string_or_float")]
  pub executed_qty: f64,
  #[serde(with = "string_or_float")]
  pub cummulative_quote_qty: f64,
  pub status: String,
  pub time_in_force: String,
  #[serde(rename = "type")]
  pub type_name: String,
  pub side: String,
  pub self_trade_prevention_mode: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TriggerCanceledResponse {
  pub symbol: String,
  pub order_list_id: i64,
  pub contingency_type: String,
  pub list_status_type: String,
  pub list_order_status: String,
  pub list_client_order_id: String,
  pub transact_time: i64,
  pub orders: Vec<TriggerOrderResponse>,
  pub order_reports: Vec<TriggerOrderResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TriggerOrderResponse {
  pub symbol: String,
  pub order_id: u64,
  pub client_order_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TriggerOrderReportResponse {
  pub symbol: String,
  pub orig_client_order_id: String,
  pub order_id: u64,
  pub order_list_id: i64,
  pub client_order_id: String,
  pub transact_time: i64,
  #[serde(with = "string_or_float")]
  pub price: f64,
  #[serde(with = "string_or_float")]
  pub orig_qty: f64,
  #[serde(with = "string_or_float")]
  pub executed_qty: f64,
  #[serde(with = "string_or_float")]
  pub cummulative_quote_qty: f64,
  pub status: String,
  pub time_in_force: String,
  #[serde(rename = "type")]
  pub type_name: String,
  pub side: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum OrderOrTriggerCanceledResponse {
  Order(OrderResponse),
  Trigger(TriggerCanceledResponse),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreatedResponse {
  pub symbol: String,
  pub order_id: u64,
  pub order_list_id: Option<i64>,
  pub client_order_id: String,
  pub transact_time: u64,
  #[serde(with = "string_or_float")]
  pub price: f64,
  #[serde(with = "string_or_float")]
  pub orig_qty: f64,
  #[serde(with = "string_or_float")]
  pub executed_qty: f64,
  #[serde(with = "string_or_float")]
  pub cummulative_quote_qty: f64,
  #[serde(with = "string_or_float", default = "default_stop_price")]
  pub stop_price: f64,
  pub status: String,
  pub time_in_force: String,
  #[serde(rename = "type")]
  pub type_name: String,
  pub side: String,
  pub fills: Option<Vec<OrderFillsResponse>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderFillsResponse {
  #[serde(with = "string_or_float")]
  pub price: f64,
  #[serde(with = "string_or_float")]
  pub qty: f64,
  #[serde(with = "string_or_float")]
  pub commission: f64,
  pub commission_asset: String,
  pub trade_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeRecordResponse {
  pub id: u64,
  #[serde(with = "string_or_float")]
  pub price: f64,
  #[serde(with = "string_or_float")]
  pub qty: f64,
  pub commission: String,
  pub commission_asset: String,
  pub time: u64,
  pub is_buyer: bool,
  pub is_maker: bool,
  pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitIntervalResponse {
  pub rate_limit_type: String,
  pub interval: String,
  pub interval_num: u64,
  pub limit: u64,
  pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StpRecordResponse {
  pub symbol: String,
  pub id: u64,
  pub prevented_match_id: u64,
  pub taker_order_id: u64,
  pub maker_order_id: u64,
  pub trade_group_id: u64,
  pub self_trade_prevention_mode: String,
  #[serde(with = "string_or_float")]
  pub price: f64,
  #[serde(with = "string_or_float")]
  pub maker_prevented_quantity: f64,
  pub transact_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountCommissionRatesResponse {
  pub symbol: String,
  pub standard_commission: CommissionRatesResponse,
  pub tax_commission: CommissionRatesResponse,
  pub discount: DiscountInfoResponse,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRatesResponse {
  #[serde(with = "string_or_float")]
  pub maker: f64,
  #[serde(with = "string_or_float")]
  pub taker: f64,
  #[serde(with = "string_or_float")]
  pub buyer: f64,
  #[serde(with = "string_or_float")]
  pub seller: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountInfoResponse {
  pub enabled_for_account: bool,
  pub enabled_for_symbol: bool,
  pub discount_asset: String,
  #[serde(with = "string_or_float")]
  pub discount: f64,
}
