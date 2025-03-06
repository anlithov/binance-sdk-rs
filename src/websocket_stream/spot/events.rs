use crate::serde_helpers::string_to_float;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WebsocketSpotEvent {
  AccountUpdate(AccountUpdateEvent),
  BalanceUpdate(BalanceUpdateEvent),
  OrderTrade(OrderTradeEvent),
  AggTrades(AggTradesEvent),
  Trade(TradeEvent),
  OrderBook(OrderBook),
  DayTicker(DayTickerEvent),
  DayTickerAll(Vec<DayTickerEvent>),
  WindowTicker(WindowTickerEvent),
  WindowTickerAll(Vec<WindowTickerEvent>),
  Kline(KlineEvent),
  DepthOrderBook(DepthOrderBookEvent),
  BookTicker(BookTickerEvent),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
  pub last_update_id: u64,
  pub bids: Vec<Bids>,
  pub asks: Vec<Asks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepthOrderBookEvent {
  #[serde(rename = "e")]
  pub event_type: String,
  #[serde(rename = "E")]
  pub event_time: u64,
  #[serde(rename = "s")]
  pub symbol: String,
  #[serde(rename = "U")]
  pub first_update_id: u64,
  #[serde(rename = "u")]
  pub final_update_id: u64,
  #[serde(rename = "pu")]
  #[serde(default)]
  pub previous_final_update_id: Option<u64>,
  #[serde(rename = "b")]
  pub bids: Vec<Bids>,
  #[serde(rename = "a")]
  pub asks: Vec<Asks>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Bids {
  #[serde(with = "string_to_float")]
  pub price: f64,
  #[serde(with = "string_to_float")]
  pub qty: f64,
}

impl Bids {
  pub fn new(price: f64, qty: f64) -> Bids {
    Bids { price, qty }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asks {
  #[serde(with = "string_to_float")]
  pub price: f64,
  #[serde(with = "string_to_float")]
  pub qty: f64,
}

/// The Trade Streams push raw trade information; each trade has a unique buyer and seller.
///
/// Stream Name: \<symbol\>@trade
///
/// Update Speed: Real-time
///
/// <https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#trade-streams>
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeEvent {
  #[serde(rename = "e")]
  pub event_type: String,
  #[serde(rename = "E")]
  pub event_time: u64,
  #[serde(rename = "s")]
  pub symbol: String,
  #[serde(rename = "t")]
  pub trade_id: u64,
  #[serde(rename = "p")]
  pub price: String,
  #[serde(rename = "q")]
  pub qty: String,
  #[serde(rename = "T")]
  pub trade_order_time: u64,
  #[serde(rename = "m")]
  pub is_buyer_maker: bool,
  #[serde(skip, rename = "M")]
  pub m_ignore: bool,
}

/// The Aggregate Trade Streams push trade information that is aggregated for a single taker order.
///
/// Stream Name: \<symbol\>@aggTrade
///
/// Update Speed: Real-time
///
/// <https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#aggregate-trade-streams>
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AggTradesEvent {
  #[serde(rename = "e")]
  pub event_type: String,
  #[serde(rename = "E")]
  pub event_time: u64,
  #[serde(rename = "s")]
  pub symbol: String,
  #[serde(rename = "a")]
  pub aggregated_trade_id: u64,
  #[serde(rename = "p")]
  pub price: String,
  #[serde(rename = "q")]
  pub qty: String,
  #[serde(rename = "f")]
  pub first_break_trade_id: u64,
  #[serde(rename = "l")]
  pub last_break_trade_id: u64,
  #[serde(rename = "T")]
  pub trade_order_time: u64,
  #[serde(rename = "m")]
  pub is_buyer_maker: bool,
  #[serde(skip, rename = "M")]
  pub m_ignore: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerEvent {
  #[serde(rename = "u")]
  pub update_id: u64,
  #[serde(rename = "s")]
  pub symbol: String,
  #[serde(rename = "b")]
  pub best_bid: String,
  #[serde(rename = "B")]
  pub best_bid_qty: String,
  #[serde(rename = "a")]
  pub best_ask: String,
  #[serde(rename = "A")]
  pub best_ask_qty: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateEvent {
  #[serde(rename = "e")]
  pub event_type: String,
  #[serde(rename = "E")]
  pub event_time: u64,
  #[serde(rename = "a")]
  pub data: AccountUpdateDataEvent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateDataEvent {
  #[serde(rename = "m")]
  pub reason: String,

  #[serde(rename = "B")]
  pub balances: Vec<EventBalance>,

  #[serde(rename = "P")]
  pub positions: Vec<EventPosition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventPosition {
  #[serde(rename = "s")]
  pub symbol: String,
  #[serde(rename = "pa")]
  pub position_amount: String,
  #[serde(rename = "ep")]
  pub entry_price: String,
  #[serde(rename = "cr")]
  pub accumulated_realized: String, // (Pre-fee) Accumulated Realized
  #[serde(rename = "up")]
  pub unrealized_pnl: String,
  #[serde(rename = "mt")]
  pub margin_type: String,
  #[serde(rename = "iw")]
  pub isolated_wallet: String,
  #[serde(rename = "ps")]
  pub position_side: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BalanceUpdateEvent {
  #[serde(rename = "B")]
  pub balance: Vec<EventBalance>,
  #[serde(rename = "e")]
  pub event_type: String,
  #[serde(rename = "E")]
  pub event_time: u64,
  #[serde(rename = "u")]
  pub last_account_update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventBalance {
  #[serde(rename = "a")]
  pub asset: String,
  #[serde(rename = "wb")]
  pub wallet_balance: String,
  #[serde(rename = "cw")]
  pub cross_wallet_balance: String,
  #[serde(rename = "bc")]
  pub balance_change: String, // Balance Change except PnL and Commission
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderTradeEvent {
  #[serde(rename = "e")]
  pub event_type: String,
  #[serde(rename = "E")]
  pub event_time: u64,
  #[serde(rename = "s")]
  pub symbol: String,
  #[serde(rename = "c")]
  pub new_client_order_id: String,
  #[serde(rename = "S")]
  pub side: String,
  #[serde(rename = "o")]
  pub order_type: String,
  #[serde(rename = "f")]
  pub time_in_force: String,
  #[serde(rename = "q")]
  pub qty: String,
  #[serde(rename = "p")]
  pub price: String,
  #[serde(skip, rename = "P")]
  pub p_ignore: String,
  #[serde(skip, rename = "F")]
  pub f_ignore: String,
  #[serde(skip)]
  pub g: i32,
  #[serde(skip, rename = "C")]
  pub c_ignore: Option<String>,
  #[serde(rename = "x")]
  pub execution_type: String,
  #[serde(rename = "X")]
  pub order_status: String,
  #[serde(rename = "r")]
  pub order_reject_reason: String,
  #[serde(rename = "i")]
  pub order_id: u64,
  #[serde(rename = "l")]
  pub qty_last_filled_trade: String,
  #[serde(rename = "z")]
  pub accumulated_qty_filled_trades: String,
  #[serde(rename = "L")]
  pub price_last_filled_trade: String,
  #[serde(rename = "n")]
  pub commission: String,
  #[serde(skip, rename = "N")]
  pub asset_commisioned: Option<String>,
  #[serde(rename = "T")]
  pub trade_order_time: u64,
  #[serde(rename = "t")]
  pub trade_id: i64,
  #[serde(skip, rename = "I")]
  pub i_ignore: u64,
  #[serde(skip)]
  pub w: bool,
  #[serde(rename = "m")]
  pub is_buyer_maker: bool,
  #[serde(skip, rename = "M")]
  pub m_ignore: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DayTickerEvent {
  #[serde(rename = "e")]
  pub event_type: String,
  #[serde(rename = "E")]
  pub event_time: u64,
  #[serde(rename = "s")]
  pub symbol: String,
  #[serde(rename = "p")]
  pub price_change: String,
  #[serde(rename = "P")]
  pub price_change_percent: String,
  #[serde(rename = "w")]
  pub average_price: String,
  #[serde(rename = "x")]
  pub prev_close: String,
  #[serde(rename = "c")]
  pub current_close: String,
  #[serde(rename = "Q")]
  pub current_close_qty: String,
  #[serde(rename = "b")]
  pub best_bid: String,
  #[serde(rename = "B")]
  pub best_bid_qty: String,
  #[serde(rename = "a")]
  pub best_ask: String,
  #[serde(rename = "A")]
  pub best_ask_qty: String,
  #[serde(rename = "o")]
  pub open: String,
  #[serde(rename = "h")]
  pub high: String,
  #[serde(rename = "l")]
  pub low: String,
  #[serde(rename = "v")]
  pub volume: String,
  #[serde(rename = "q")]
  pub quote_volume: String,
  #[serde(rename = "O")]
  pub open_time: u64,
  #[serde(rename = "C")]
  pub close_time: u64,
  #[serde(rename = "F")]
  pub first_trade_id: i64,
  #[serde(rename = "L")]
  pub last_trade_id: i64,
  #[serde(rename = "n")]
  pub num_trades: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WindowTickerEvent {
  #[serde(rename = "e")]
  pub event_type: String,
  #[serde(rename = "E")]
  pub event_time: u64,
  #[serde(rename = "s")]
  pub symbol: String,
  #[serde(rename = "p")]
  pub price_change: String,
  #[serde(rename = "P")]
  pub price_change_percent: String,
  #[serde(rename = "o")]
  pub open: String,
  #[serde(rename = "h")]
  pub high: String,
  #[serde(rename = "l")]
  pub low: String,
  #[serde(rename = "c")]
  pub current_close: String,
  #[serde(rename = "w")]
  pub average_price: String,
  #[serde(rename = "v")]
  pub volume: String,
  #[serde(rename = "q")]
  pub quote_volume: String,
  #[serde(rename = "O")]
  pub open_time: u64,
  #[serde(rename = "C")]
  pub close_time: u64,
  #[serde(rename = "F")]
  pub first_trade_id: i64,
  #[serde(rename = "L")]
  pub last_trade_id: i64,
  #[serde(rename = "n")]
  pub num_trades: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MiniTickerEvent {
  #[serde(rename = "e")]
  pub event_type: String,
  #[serde(rename = "E")]
  pub event_time: u64,
  #[serde(rename = "s")]
  pub symbol: String,
  #[serde(rename = "c")]
  pub close: String,
  #[serde(rename = "o")]
  pub open: String,
  #[serde(rename = "h")]
  pub high: String,
  #[serde(rename = "l")]
  pub low: String,
  #[serde(rename = "v")]
  pub volume: String,
  #[serde(rename = "q")]
  pub quote_volume: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KlineEvent {
  #[serde(rename = "e")]
  pub event_type: String,
  #[serde(rename = "E")]
  pub event_time: u64,
  #[serde(rename = "s")]
  pub symbol: String,
  #[serde(rename = "k")]
  pub kline: Kline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
  #[serde(rename = "t")]
  pub open_time: i64,
  #[serde(rename = "T")]
  pub close_time: i64,
  #[serde(rename = "s")]
  pub symbol: String,
  #[serde(rename = "i")]
  pub interval: String,
  #[serde(rename = "f")]
  pub first_trade_id: i64,
  #[serde(rename = "L")]
  pub last_trade_id: i64,
  #[serde(rename = "o")]
  pub open: String,
  #[serde(rename = "c")]
  pub close: String,
  #[serde(rename = "h")]
  pub high: String,
  #[serde(rename = "l")]
  pub low: String,
  #[serde(rename = "v")]
  pub volume: String,
  #[serde(rename = "n")]
  pub number_of_trades: i64,
  #[serde(rename = "x")]
  pub is_final_bar: bool,
  #[serde(rename = "q")]
  pub quote_asset_volume: String,
  #[serde(rename = "V")]
  pub taker_buy_base_asset_volume: String,
  #[serde(rename = "Q")]
  pub taker_buy_quote_asset_volume: String,
  #[serde(skip, rename = "B")]
  pub ignore_me: String,
}

// https://binance-docs.github.io/apidocs/futures/en/#continuous-contract-kline-candlestick-streams

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousKlineEvent {
  #[serde(rename = "e")]
  pub event_type: String,
  #[serde(rename = "E")]
  pub event_time: u64,
  #[serde(rename = "ps")]
  pub pair: String,
  #[serde(rename = "ct")]
  pub contract_type: String,
  #[serde(rename = "k")]
  pub kline: ContinuousKline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousKline {
  #[serde(rename = "t")]
  pub start_time: i64,
  #[serde(rename = "T")]
  pub end_time: i64,
  #[serde(rename = "i")]
  pub interval: String,
  #[serde(rename = "f")]
  pub first_trade_id: i64,
  #[serde(rename = "L")]
  pub last_trade_id: i64,
  #[serde(rename = "o")]
  pub open: String,
  #[serde(rename = "c")]
  pub close: String,
  #[serde(rename = "h")]
  pub high: String,
  #[serde(rename = "l")]
  pub low: String,
  #[serde(rename = "v")]
  pub volume: String,
  #[serde(rename = "n")]
  pub number_of_trades: i64,
  #[serde(rename = "x")]
  pub is_final_bar: bool,
  #[serde(rename = "q")]
  pub quote_volume: String,
  #[serde(rename = "V")]
  pub active_buy_volume: String,
  #[serde(rename = "Q")]
  pub active_volume_buy_quote: String,
  #[serde(skip, rename = "B")]
  pub ignore_me: String,
}

// https://binance-docs.github.io/apidocs/delivery/en/#index-kline-candlestick-streams

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexKlineEvent {
  #[serde(rename = "e")]
  pub event_type: String,
  #[serde(rename = "E")]
  pub event_time: u64,
  #[serde(rename = "ps")]
  pub pair: String,
  #[serde(rename = "k")]
  pub kline: IndexKline,
}

// https://binance-docs.github.io/apidocs/delivery/en/#index-kline-candlestick-streams
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IndexKline {
  #[serde(rename = "t")]
  pub start_time: i64,
  #[serde(rename = "T")]
  pub end_time: i64,
  #[serde(skip, rename = "s")]
  pub ignore_me: String,
  #[serde(rename = "i")]
  pub interval: String,
  #[serde(rename = "f")]
  pub first_trade_id: i64,
  #[serde(rename = "L")]
  pub last_trade_id: i64,
  #[serde(rename = "o")]
  pub open: String,
  #[serde(rename = "c")]
  pub close: String,
  #[serde(rename = "h")]
  pub high: String,
  #[serde(rename = "l")]
  pub low: String,
  #[serde(rename = "v")]
  pub volume: String,
  #[serde(rename = "n")]
  pub number_of_trades: i64,
  #[serde(rename = "x")]
  pub is_final_bar: bool,
  #[serde(skip, rename = "q")]
  pub ignore_me2: String,
  #[serde(skip, rename = "V")]
  pub ignore_me3: String,
  #[serde(skip, rename = "Q")]
  pub ignore_me4: String,
  #[serde(skip, rename = "B")]
  pub ignore_me5: String,
}
