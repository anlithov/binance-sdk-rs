use std::collections::BTreeMap;
use std::fmt::Display;

pub struct OrderRequest {
  pub symbol: String,
  pub order_side: OrderSide,
  pub order_type: OrderType,
  pub time_in_force: TimeInForce,
  pub qty: f64,
  pub price: f64,
  pub new_client_order_id: Option<String>,
  pub stop_price: Option<f64>,
}

impl OrderRequest {
  pub(crate) fn build_order(self: OrderRequest) -> BTreeMap<String, String> {
    let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

    order_parameters.insert("symbol".into(), self.symbol);
    order_parameters.insert("side".into(), self.order_side.to_string());
    order_parameters.insert("type".into(), self.order_type.to_string());
    order_parameters.insert("quantity".into(), self.qty.to_string());

    if let Some(stop_price) = self.stop_price {
      order_parameters.insert("stopPrice".into(), stop_price.to_string());
    }

    if self.price != 0.0 {
      order_parameters.insert("price".into(), self.price.to_string());
      order_parameters.insert("timeInForce".into(), self.time_in_force.to_string());
    }

    if let Some(client_order_id) = self.new_client_order_id {
      order_parameters.insert("newClientOrderId".into(), client_order_id);
    }

    order_parameters
  }
}

pub struct OrderQuoteQuantityRequest {
  pub symbol: String,
  pub order_side: OrderSide,
  pub order_type: OrderType,
  pub time_in_force: TimeInForce,
  pub quote_order_qty: f64,
  pub price: f64,
  pub new_client_order_id: Option<String>,
}

impl OrderQuoteQuantityRequest {
  pub(crate) fn build_quote(self: OrderQuoteQuantityRequest) -> BTreeMap<String, String> {
    let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

    order_parameters.insert("symbol".into(), self.symbol);
    order_parameters.insert("side".into(), self.order_side.to_string());
    order_parameters.insert("type".into(), self.order_type.to_string());
    order_parameters.insert("quoteOrderQty".into(), self.quote_order_qty.to_string());

    if self.price != 0.0 {
      order_parameters.insert("price".into(), self.price.to_string());
      order_parameters.insert("timeInForce".into(), self.time_in_force.to_string());
    }

    if let Some(client_order_id) = self.new_client_order_id {
      order_parameters.insert("newClientOrderId".into(), client_order_id);
    }

    order_parameters
  }
}

pub struct OrderCustomRequest {
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

impl OrderCustomRequest {
  pub(crate) fn build_order(self: OrderCustomRequest) -> BTreeMap<String, String> {
    let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

    order_parameters.insert("symbol".into(), self.symbol);
    order_parameters.insert("side".into(), self.order_side.to_string());
    order_parameters.insert("type".into(), self.order_type.to_string());

    if let Some(time_in_force) = self.time_in_force {
      order_parameters.insert("timeInForce".into(), time_in_force.to_string());
    }
    if let Some(qty) = self.qty {
      order_parameters.insert("quantity".into(), qty.to_string());
    }
    if let Some(quote_order_qty) = self.quote_order_qty {
      order_parameters.insert("quantityOrderQty".into(), quote_order_qty.to_string());
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

pub enum OrderType {
  Limit,
  Market,
  StopLoss,
  StopLossLimit,
  TakeProfit,
  TakeProfitLimit,
  LimitMaker,
}

impl OrderType {
  pub fn from_int(value: i32) -> Option<Self> {
    match value {
      1 => Some(OrderType::Limit),
      2 => Some(OrderType::Market),
      3 => Some(OrderType::StopLoss),
      4 => Some(OrderType::StopLossLimit),
      5 => Some(OrderType::TakeProfit),
      6 => Some(OrderType::TakeProfitLimit),
      7 => Some(OrderType::LimitMaker),
      _ => None,
    }
  }
}

impl Display for OrderType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Limit => write!(f, "LIMIT"),
      Self::Market => write!(f, "MARKET"),
      Self::StopLoss => write!(f, "STOP_LOSS"),
      Self::StopLossLimit => write!(f, "STOP_LOSS_LIMIT"),
      Self::TakeProfit => write!(f, "TAKE_PROFIT"),
      Self::TakeProfitLimit => write!(f, "TAKE_PROFIT_LIMIT"),
      Self::LimitMaker => write!(f, "LIMIT_MAKER"),
    }
  }
}

pub enum OrderSide {
  Buy,
  Sell,
}

impl OrderSide {
  pub fn from_int(value: i32) -> Option<Self> {
    match value {
      1 => Some(OrderSide::Buy),
      2 => Some(OrderSide::Sell),
      _ => None,
    }
  }
}

impl Display for OrderSide {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Buy => write!(f, "BUY"),
      Self::Sell => write!(f, "SELL"),
    }
  }
}

#[allow(clippy::all)]
pub enum TimeInForce {
  GTC,
  IOC,
  FOK,
}

impl TimeInForce {
  pub fn from_int(value: i32) -> Option<Self> {
    match value {
      1 => Some(TimeInForce::GTC),
      2 => Some(TimeInForce::IOC),
      3 => Some(TimeInForce::FOK),
      _ => None,
    }
  }
}

impl Display for TimeInForce {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::GTC => write!(f, "GTC"),
      Self::IOC => write!(f, "IOC"),
      Self::FOK => write!(f, "FOK"),
    }
  }
}
