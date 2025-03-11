use super::enums::{OrderSide, OrderType, TimeInForce};
use crate::util::is_start_time_valid;
use anyhow::{bail, Result};
use std::collections::BTreeMap;

#[derive(Default, Clone)]
pub struct PlaceOrderRequest {
  pub symbol: String,
  pub order_side: OrderSide,
  pub order_type: OrderType,
  pub time_in_force: Option<TimeInForce>,
  pub qty: Option<f64>,
  pub quote_order_qty: Option<f64>,
  pub price: Option<f64>,
  pub new_client_order_id: Option<String>,
  pub strategy_id: Option<u64>,
  pub strategy_type: Option<String>,
  pub stop_price: Option<f64>,
  pub trailing_delta: Option<f64>,
  pub iceberg_qty: Option<f64>,
  pub new_order_resp_time: Option<String>,
  pub self_trade_prevention_mode: Option<String>,
}

impl PlaceOrderRequest {
  pub(crate) fn build_params_tree(self: PlaceOrderRequest) -> BTreeMap<String, String> {
    let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

    order_parameters.insert("symbol".into(), self.symbol.to_uppercase());
    order_parameters.insert("side".into(), self.order_side.to_string());
    order_parameters.insert("type".into(), self.order_type.to_string());

    if let Some(time_in_force) = self.time_in_force {
      order_parameters.insert("timeInForce".into(), time_in_force.to_string());
    }
    if let Some(qty) = self.qty {
      order_parameters.insert("quantity".into(), qty.to_string());
    }
    if let Some(quote_order_qty) = self.quote_order_qty {
      order_parameters.insert("quoteOrderQty".into(), quote_order_qty.to_string());
    }
    if let Some(price) = self.price {
      order_parameters.insert("price".into(), price.to_string());
    }
    if let Some(new_client_order_id) = self.new_client_order_id {
      order_parameters.insert("newClientOrderId".into(), new_client_order_id);
    }
    if let Some(strategy_id) = self.strategy_id {
      order_parameters.insert("strategyId".into(), strategy_id.to_string());
    }
    if let Some(strategy_type) = self.strategy_type {
      order_parameters.insert("strategyType".into(), strategy_type);
    }
    if let Some(stop_price) = self.stop_price {
      order_parameters.insert("stopPrice".into(), stop_price.to_string());
    }
    if let Some(trailing_delta) = self.trailing_delta {
      order_parameters.insert("trailingDelta".into(), trailing_delta.to_string());
    }
    if let Some(iceberg_qty) = self.iceberg_qty {
      order_parameters.insert("icebergQty".into(), iceberg_qty.to_string());
    }
    if let Some(new_order_resp_time) = self.new_order_resp_time {
      order_parameters.insert("newOrderRespType".into(), new_order_resp_time.to_string());
    }
    if let Some(self_trade_prevention_mode) = self.self_trade_prevention_mode {
      order_parameters.insert("selfTradePreventionMode".into(), self_trade_prevention_mode);
    }

    order_parameters
  }
}

#[derive(Default, Clone)]
pub struct TradeHistoryRequest {
  pub symbol: String,
  pub order_id: Option<u64>,
  pub from_id: Option<u64>,
  pub start_time: Option<u64>,
  pub end_time: Option<u64>,
  pub limit: Option<u16>,
}

impl TradeHistoryRequest {
  pub(crate) fn build_params_tree(self) -> Result<BTreeMap<String, String>> {
    let start_time = self.start_time;
    let end_time = self.end_time;
    if start_time.is_some() && end_time.is_some() && (&start_time > &end_time) {
      bail!("End time should be greater than start time");
    }
    if let Some(st) = &start_time {
      if !is_start_time_valid(st) {
        bail!("Start time should be less than the current time");
      }
    }

    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    parameters.insert("symbol".into(), self.symbol);
    if let Some(lt) = self.limit {
      parameters.insert("limit".into(), format!("{}", lt));
    }
    if let Some(ord) = self.order_id {
      parameters.insert("orderId".into(), format!("{}", ord));
    }
    if let Some(st) = start_time {
      parameters.insert("startTime".into(), format!("{}", st));
    }
    if let Some(et) = end_time {
      parameters.insert("endTime".into(), format!("{}", et));
    }
    if let Some(fi) = self.from_id {
      parameters.insert("fromId".into(), format!("{}", fi));
    }

    Ok(parameters)
  }
}
