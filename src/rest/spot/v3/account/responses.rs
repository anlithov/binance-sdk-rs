use crate::serde_helpers::string_to_float;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

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
  #[serde(with = "string_to_float")]
  pub maker: f64,
  #[serde(with = "string_to_float")]
  pub taker: f64,
  #[serde(with = "string_to_float")]
  pub seller: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetBalanceResponse {
  pub asset: String,
  #[serde(with = "string_to_float")]
  pub free: f64,
  #[serde(with = "string_to_float")]
  pub locked: f64,
}

/// Rate limit interval as defined by Binance API
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum AccountRateLimitIntervalResponse {
  #[serde(rename = "SECOND")]
  Second,
  #[serde(rename = "MINUTE")]
  Minute,
  #[serde(rename = "DAY")]
  Day,
}

impl Display for AccountRateLimitIntervalResponse {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountRateLimitResponse {
  // Potentially always "ORDERS"
  pub rate_limit_type: String,
  pub interval: AccountRateLimitIntervalResponse,
  pub interval_num: u64,
  pub limit: u64,
  pub count: u64,
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
  #[serde(with = "string_to_float")]
  pub maker: f64,
  #[serde(with = "string_to_float")]
  pub taker: f64,
  #[serde(with = "string_to_float")]
  pub buyer: f64,
  #[serde(with = "string_to_float")]
  pub seller: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountInfoResponse {
  pub enabled_for_account: bool,
  pub enabled_for_symbol: bool,
  pub discount_asset: String,
  #[serde(with = "string_to_float")]
  pub discount: f64,
}
