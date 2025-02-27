use crate::serde_helpers::string_to_float;
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
