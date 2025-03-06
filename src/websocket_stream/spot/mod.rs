use anyhow::Result;
use events::{
  AccountUpdateEvent, AggTradesEvent, BalanceUpdateEvent, BookTickerEvent, DayTickerEvent,
  DepthOrderBookEvent, KlineEvent, OrderBook, OrderTradeEvent, TradeEvent, WebsocketSpotEvent,
  WindowTickerEvent,
};
use futures_util::SinkExt;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub mod events;

enum WebsocketUrl {
  Default,
  MultiStream,
  Custom(String),
}

impl WebsocketUrl {
  fn params(self, subscription: &str) -> String {
    match self {
      WebsocketUrl::Default => format!("wss://stream.binance.com:443/ws/{}", subscription),
      WebsocketUrl::MultiStream => {
        format!(
          "wss://stream.binance.com:443/stream?streams={}",
          subscription
        )
      }
      WebsocketUrl::Custom(url) => format!("{}/{}", url, subscription),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum InternalEvents {
  DayTickerEventAll(Vec<DayTickerEvent>),
  WindowTickerEventAll(Vec<WindowTickerEvent>),
  BalanceUpdateEvent(BalanceUpdateEvent),
  DayTickerEvent(DayTickerEvent),
  WindowTickerEvent(WindowTickerEvent),
  BookTickerEvent(BookTickerEvent),
  AccountUpdateEvent(AccountUpdateEvent),
  OrderTradeEvent(OrderTradeEvent),
  AggTradesEvent(AggTradesEvent),
  TradeEvent(TradeEvent),
  KlineEvent(KlineEvent),
  OrderBook(OrderBook),
  DepthOrderBookEvent(DepthOrderBookEvent),
}

pub struct WebSocketSpotStream<'a> {
  /// Active subscriptions
  subscriptions: HashSet<String>,
  /// Event handler (can modify Arcs inside)
  handler: Box<dyn FnMut(WebsocketSpotEvent) -> Result<()> + 'a + Send + Sync>,
  /// WebSocket connection
  socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl<'a> WebSocketSpotStream<'a> {
  /// Construct a new WebSockets struct with callback
  pub fn new<Callback>(handler: Callback) -> Self
  where
    Callback: FnMut(WebsocketSpotEvent) -> Result<()> + 'a + Send + Sync,
  {
    Self {
      subscriptions: HashSet::new(),
      handler: Box::new(handler),
      socket: None,
    }
  }

  /// Subscribe to a stream (adds it dynamically)
  pub async fn subscribe(&mut self, stream: &str) -> Result<()> {
    let inserted = self.subscriptions.insert(stream.to_string());
    if !inserted {
      return Ok(());
    }

    if let Some(socket) = &mut self.socket {
      let msg = json!({
          "method": "SUBSCRIBE",
          "params": [stream],
          "id": 1
      })
      .to_string();
      socket.send(Message::Text(Utf8Bytes::from(msg))).await?;
    } else {
      self.connect().await?;
    }
    Ok(())
  }

  /// Unsubscribe from a stream (removes dynamically)
  pub async fn unsubscribe(&mut self, stream: &str) -> Result<()> {
    let removed = self.subscriptions.remove(stream);
    if !removed {
      return Ok(());
    }
    if let Some(socket) = &mut self.socket {
      let msg = json!({
          "method": "UNSUBSCRIBE",
          "params": [stream],
          "id": 1
      })
      .to_string();
      socket.send(Message::Text(Utf8Bytes::from(msg))).await?;
    }

    if self.subscriptions.is_empty() {
      self.disconnect().await?;
    }
    Ok(())
  }

  /// Connects to Binance WebSocket API with current subscriptions
  async fn connect(&mut self) -> Result<()> {
    if self.subscriptions.is_empty() {
      return Ok(());
    }

    let res: Vec<String> = self.subscriptions.iter().cloned().collect();
    let url = WebsocketUrl::MultiStream.params(&res.join("/"));
    let (socket, _) = connect_async(url).await?;
    self.socket = Some(socket);
    Ok(())
  }

  /// Disconnect WebSocket
  async fn disconnect(&mut self) -> Result<()> {
    if let Some(mut socket) = self.socket.take() {
      socket.close(None).await?;
    }
    Ok(())
  }

  /// Main event loop (runs in a separate async task)
  pub async fn event_loop(&mut self, running: Arc<AtomicBool>) -> Result<()> {
    while running.load(Ordering::Relaxed) {
      if let Some(socket) = &mut self.socket {
        match socket.next().await {
          Some(Ok(Message::Text(msg))) => {
            self.handle_incoming_message(&msg)?;
          }
          Some(Ok(Message::Ping(payload))) => {
            socket.send(Message::Pong(payload)).await?;
          }
          Some(Ok(Message::Close(_))) | None => {
            eprintln!("WebSocket disconnected, exiting event loop.");
            break;
          }
          Some(Err(e)) => {
            eprintln!("WebSocket error: {:?}", e);
            break;
          }
          _ => {}
        }
      } else {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
      }
    }
    Ok(())
  }

  /// Processes incoming messages
  fn handle_incoming_message(&mut self, msg: &str) -> Result<()> {
    let json: serde_json::Value = serde_json::from_str(msg)?;

    if let Some(data) = json.get("data") {
      self.handle_incoming_message(&data.to_string())?;
      return Ok(());
    }

    let res_json = serde_json::from_value::<InternalEvents>(json);
    if let Ok(events) = res_json {
      let action = match events {
        InternalEvents::DayTickerEventAll(v) => WebsocketSpotEvent::DayTickerAll(v),
        InternalEvents::WindowTickerEventAll(v) => WebsocketSpotEvent::WindowTickerAll(v),
        InternalEvents::BookTickerEvent(v) => WebsocketSpotEvent::BookTicker(v),
        InternalEvents::BalanceUpdateEvent(v) => WebsocketSpotEvent::BalanceUpdate(v),
        InternalEvents::AccountUpdateEvent(v) => WebsocketSpotEvent::AccountUpdate(v),
        InternalEvents::OrderTradeEvent(v) => WebsocketSpotEvent::OrderTrade(v),
        InternalEvents::AggTradesEvent(v) => WebsocketSpotEvent::AggTrades(v),
        InternalEvents::TradeEvent(v) => WebsocketSpotEvent::Trade(v),
        InternalEvents::DayTickerEvent(v) => WebsocketSpotEvent::DayTicker(v),
        InternalEvents::WindowTickerEvent(v) => WebsocketSpotEvent::WindowTicker(v),
        InternalEvents::KlineEvent(v) => WebsocketSpotEvent::Kline(v),
        InternalEvents::OrderBook(v) => WebsocketSpotEvent::OrderBook(v),
        InternalEvents::DepthOrderBookEvent(v) => WebsocketSpotEvent::DepthOrderBook(v),
      };
      (self.handler)(action)?;
    }
    Ok(())
  }
}
