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
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;
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

/// Command enum that the internal actor will handle
enum Command {
  Subscribe(Vec<String>),
  Unsubscribe(Vec<String>),
  Shutdown,
}

struct WebSocketActor {
  /// Active subscriptions
  subscriptions: HashSet<String>,
  /// Event handler (can modify Arcs inside)
  handler: Box<dyn FnMut(WebsocketSpotEvent) -> Result<()> + Send + Sync + 'static>,
  /// WebSocket connection
  socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl WebSocketActor {
  /// Construct a new WebSockets struct with callback
  pub fn new<Callback>(handler: Callback) -> Self
  where
    Callback: FnMut(WebsocketSpotEvent) -> Result<()> + Send + Sync + 'static,
  {
    Self {
      subscriptions: HashSet::new(),
      handler: Box::new(handler),
      socket: None,
    }
  }

  /// Subscribe to a stream (adds it dynamically)
  pub async fn subscribe(&mut self, streams: Vec<String>) -> Result<()> {
    let mut new_streams: Vec<String> = vec![];

    streams.iter().for_each(|x| {
      let inserted = self.subscriptions.insert(x.clone().to_string());
      if inserted {
        return new_streams.push(x.to_string());
      }
    });

    if new_streams.is_empty() {
      return Ok(());
    }

    if let Some(socket) = &mut self.socket {
      let msg = json!({
          "method": "SUBSCRIBE",
          "params": new_streams,
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
  pub async fn unsubscribe(&mut self, streams: Vec<String>) -> Result<()> {
    let mut remove_streams: Vec<String> = vec![];

    streams.iter().for_each(|x| {
      let removed = self.subscriptions.remove(&x.clone());
      if removed {
        return remove_streams.push(x.to_string());
      }
    });

    if remove_streams.is_empty() {
      return Ok(());
    }

    if let Some(socket) = &mut self.socket {
      let msg = json!({
          "method": "UNSUBSCRIBE",
          "params": remove_streams,
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

  async fn run(mut self, mut cmd_rx: Receiver<Command>) -> Result<()> {
    let mut keep_running = true;

    while keep_running {
      tokio::select! {
        command = cmd_rx.recv() => {
          match command {
            Some(Command::Subscribe(stream)) => {
              self.subscribe(stream).await?;
            }
            Some(Command::Unsubscribe(streams)) => {
              self.unsubscribe(streams).await?;
            }
            Some(Command::Shutdown) => {
              // Exiting this loop will close the websocket and end the task
              keep_running = false;
            }
            None => {
              // The sender side was dropped, so just shutdown
              keep_running = false;
            }
              }
          },


          // 2) Or read next WebSocket message (if connected)
          next_message = async {
          if let Some(socket) = &mut self.socket {
            socket.next().await
          } else {
            // If no socket, sleep briefly so we don't busy-loop
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            None // no message
          }
          }, if self.socket.is_some() => {
          match next_message {
            Some(Ok(Message::Text(msg))) => {
              self.handle_incoming_message(&msg)?;
            }
            Some(Ok(Message::Ping(payload))) => {
              if let Some(socket) = &mut self.socket {
                socket.send(Message::Pong(payload)).await?;
              }
            }
            Some(Ok(Message::Close(_))) => {
              eprintln!("WebSocket disconnected. Attempting to reconnect or break?");
              self.disconnect().await?;
              // optionally attempt reconnection here, or just keep running
            },
            Some(Err(e)) => {
                            eprintln!("WebSocket error: {:?}", e);
                            self.disconnect().await?;
                            // optionally attempt to reconnect
                        }
            _ => {}
          }
          }
      }
    }

    // Final cleanup
    self.disconnect().await?;
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

pub struct WebSocketSpotStream {
  command_tx: mpsc::Sender<Command>,
  join_handle: JoinHandle<Result<()>>,
}

impl WebSocketSpotStream {
  /// Construct and start background task immediately.
  ///
  /// - `handler` is the callback for incoming events.
  pub fn new<Callback>(handler: Callback) -> Self
  where
    Callback: FnMut(WebsocketSpotEvent) -> Result<()> + Send + Sync + 'static,
  {
    // Create the command channel
    let (tx, rx) = mpsc::channel(32);

    // Build the actor
    let actor = WebSocketActor::new(handler);

    // Spawn the actor in a background task
    let join_handle = tokio::spawn(async move {
      let result = actor.run(rx).await;
      if let Err(ref e) = result {
        eprintln!("WebSocket actor error: {:?}", e);
      }
      result
    });

    // Return a handle that can be used to send commands to that actor
    Self {
      command_tx: tx,
      join_handle,
    }
  }

  /// Subscribe to a stream
  pub async fn subscribe(&self, streams: Vec<String>) -> Result<()> {
    self
      .command_tx
      .send(Command::Subscribe(streams))
      .await
      .map_err(|_| anyhow::anyhow!("Actor task ended"))?;
    Ok(())
  }

  /// Unsubscribe from a stream
  pub async fn unsubscribe(&self, streams: Vec<String>) -> Result<()> {
    self
      .command_tx
      .send(Command::Unsubscribe(streams))
      .await
      .map_err(|_| anyhow::anyhow!("Actor task ended"))?;
    Ok(())
  }

  /// Ask the actor to shut down. Optionally you can `await` the join handle after this.
  pub async fn shutdown(&self) -> Result<()> {
    self
      .command_tx
      .send(Command::Shutdown)
      .await
      .map_err(|_| anyhow::anyhow!("Actor task ended"))?;
    Ok(())
  }

  /// If you want, you can provide a method to wait for the actor to finish.
  pub async fn wait_for_end(self) -> Result<()> {
    match self.join_handle.await {
      Ok(r) => r,
      Err(e) => Err(anyhow::anyhow!("Join error: {:?}", e)),
    }
  }
}
