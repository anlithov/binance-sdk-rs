use anyhow::Result;
use binance::websocket_stream::spot::events::WebsocketSpotEvent;
use binance::websocket_stream::spot::WebSocketSpotStream;
use dotenvy::dotenv;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv().expect("Failed to read .env file");

  market_websocket().await?;

  Ok(())
}

async fn market_websocket() -> Result<()> {
  let keep_running = Arc::new(AtomicBool::new(true)); // Used to control the event loop
  let btc_trade = "btcusdt@trade";
  let mut web_socket: WebSocketSpotStream<'_> =
    WebSocketSpotStream::new(|event: WebsocketSpotEvent| {
      match event {
        WebsocketSpotEvent::Trade(trade) => {
          println!(
            "Symbol: {}, price: {}, qty: {}",
            trade.symbol, trade.price, trade.qty
          );
        }
        WebsocketSpotEvent::DepthOrderBook(depth_order_book) => {
          println!(
            "Symbol: {}, Bids: {:?}, Ask: {:?}",
            depth_order_book.symbol, depth_order_book.bids, depth_order_book.asks
          );
        }
        WebsocketSpotEvent::OrderBook(order_book) => {
          println!(
            "last_update_id: {}, Bids: {:?}, Ask: {:?}",
            order_book.last_update_id, order_book.bids, order_book.asks
          );
        }
        _ => (),
      };

      Ok(())
    });

  web_socket.subscribe(btc_trade).await?; // check error
  let keep_running_clone = keep_running.clone();
  tokio::spawn(async move {
    if let Err(e) = web_socket.event_loop(keep_running_clone).await {
      eprintln!("WebSocket error: {}", e);
    }
  });
  sleep(Duration::from_secs(30)).await;

  println!("disconnected");

  Ok(())
}
