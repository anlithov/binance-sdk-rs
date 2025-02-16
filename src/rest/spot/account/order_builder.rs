use std::collections::BTreeMap;
use std::fmt::Display;

pub(crate) fn build_order(order: OrderRequest) -> BTreeMap<String, String> {
  let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

  order_parameters.insert("symbol".into(), order.symbol);
  order_parameters.insert("side".into(), order.order_side.to_string());
  order_parameters.insert("type".into(), order.order_type.to_string());
  order_parameters.insert("quantity".into(), order.qty.to_string());

  if let Some(stop_price) = order.stop_price {
    order_parameters.insert("stopPrice".into(), stop_price.to_string());
  }

  if order.price != 0.0 {
    order_parameters.insert("price".into(), order.price.to_string());
    order_parameters.insert("timeInForce".into(), order.time_in_force.to_string());
  }

  if let Some(client_order_id) = order.new_client_order_id {
    order_parameters.insert("newClientOrderId".into(), client_order_id);
  }

  order_parameters
}

pub(crate) fn build_quote_quantity_order(
  order: OrderQuoteQuantityRequest,
) -> BTreeMap<String, String> {
  let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

  order_parameters.insert("symbol".into(), order.symbol);
  order_parameters.insert("side".into(), order.order_side.to_string());
  order_parameters.insert("type".into(), order.order_type.to_string());
  order_parameters.insert("quoteOrderQty".into(), order.quote_order_qty.to_string());

  if order.price != 0.0 {
    order_parameters.insert("price".into(), order.price.to_string());
    order_parameters.insert("timeInForce".into(), order.time_in_force.to_string());
  }

  if let Some(client_order_id) = order.new_client_order_id {
    order_parameters.insert("newClientOrderId".into(), client_order_id);
  }

  order_parameters
}

pub(crate) struct OrderRequest {
  pub symbol: String,
  pub qty: f64,
  pub price: f64,
  pub stop_price: Option<f64>,
  pub order_side: OrderSide,
  pub order_type: OrderType,
  pub time_in_force: TimeInForce,
  pub new_client_order_id: Option<String>,
}

pub(crate) struct OrderQuoteQuantityRequest {
  pub symbol: String,
  pub quote_order_qty: f64,
  pub price: f64,
  pub order_side: OrderSide,
  pub order_type: OrderType,
  pub time_in_force: TimeInForce,
  pub new_client_order_id: Option<String>,
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
