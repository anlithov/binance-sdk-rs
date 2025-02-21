use crate::serde_helpers::{default_stop_price, string_to_float};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfoResponse {
  pub symbol: String,
  pub order_id: u64,
  pub order_list_id: i64,
  pub client_order_id: String,
  #[serde(with = "string_to_float")]
  pub price: f64,
  #[serde(with = "string_to_float")]
  pub orig_qty: f64,
  #[serde(with = "string_to_float")]
  pub executed_qty: f64,
  #[serde(with = "string_to_float")]
  pub cummulative_quote_qty: f64,
  pub status: String,
  pub time_in_force: String,
  #[serde(rename = "type")]
  pub type_name: String,
  pub side: String,
  #[serde(with = "string_to_float")]
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
  #[serde(with = "string_to_float")]
  pub price: f64,
  #[serde(with = "string_to_float")]
  pub orig_qty: f64,
  #[serde(with = "string_to_float")]
  pub executed_qty: f64,
  #[serde(with = "string_to_float")]
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
  #[serde(with = "string_to_float")]
  pub price: f64,
  #[serde(with = "string_to_float")]
  pub orig_qty: f64,
  #[serde(with = "string_to_float")]
  pub executed_qty: f64,
  #[serde(with = "string_to_float")]
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
  Order(OrderInfoResponse),
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
  #[serde(with = "string_to_float")]
  pub price: f64,
  #[serde(with = "string_to_float")]
  pub orig_qty: f64,
  #[serde(with = "string_to_float")]
  pub executed_qty: f64,
  #[serde(with = "string_to_float")]
  pub cummulative_quote_qty: f64,
  #[serde(with = "string_to_float", default = "default_stop_price")]
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
  #[serde(with = "string_to_float")]
  pub price: f64,
  #[serde(with = "string_to_float")]
  pub qty: f64,
  #[serde(with = "string_to_float")]
  pub commission: f64,
  pub commission_asset: String,
  pub trade_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeRecordResponse {
  pub id: u64,
  #[serde(with = "string_to_float")]
  pub price: f64,
  #[serde(with = "string_to_float")]
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
pub struct StpRecordResponse {
  pub symbol: String,
  pub id: u64,
  pub prevented_match_id: u64,
  pub taker_order_id: u64,
  pub maker_order_id: u64,
  pub trade_group_id: u64,
  pub self_trade_prevention_mode: String,
  #[serde(with = "string_to_float")]
  pub price: f64,
  #[serde(with = "string_to_float")]
  pub maker_prevented_quantity: f64,
  pub transact_time: u64,
}
