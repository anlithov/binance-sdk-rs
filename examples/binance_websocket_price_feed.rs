use anyhow::Result;
use binance::websocket_stream::spot::events::WebsocketSpotEvent;
use binance::websocket_stream::spot::WebSocketSpotStream;
use dotenvy::dotenv;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
  dotenv().expect("Failed to read .env file");

  last_price_for_one_symbol().await?;
  market_websocket().await?;

  sleep(Duration::from_secs(30)).await;

  Ok(())
}

async fn last_price_for_one_symbol() -> Result<()> {
  let mut btcusdt: f64 = 0.0;

  let web_socket = WebSocketSpotStream::new(move |event: WebsocketSpotEvent| {
    if let WebsocketSpotEvent::DayTicker(ticker_event) = event {
      btcusdt = ticker_event.average_price;
      let btcusdt_close = ticker_event.current_close;
      println!("{} - {} - {}", ticker_event.symbol, btcusdt, btcusdt_close);
    }

    Ok(())
  });

  web_socket
    .subscribe(vec![
      "btcusdt@ticker".to_string(),
      "solusdt@ticker".to_string(),
    ])
    .await?; // check error
  web_socket
    .subscribe(vec!["ethusdt@ticker".to_string()])
    .await?;
  web_socket
    .subscribe(vec!["btcusdt@ticker".to_string()])
    .await?;

  tokio::time::sleep(Duration::from_secs(10)).await;

  web_socket
    .unsubscribe(vec![
      "btcusdt@ticker".to_string(),
      "solusdt@ticker".to_string(),
    ])
    .await?;

  web_socket
    .unsubscribe(vec![
      "btcusdt@ticker".to_string(),
      "solusdt@ticker".to_string(),
    ])
    .await?;

  tokio::time::sleep(Duration::from_secs(10)).await;

  println!("disconnected");

  Ok(())
}

async fn market_websocket() -> Result<()> {
  let btc_trade = vec!["btcusdt@trade".to_string()];
  let web_socket = WebSocketSpotStream::new(move |event: WebsocketSpotEvent| {
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

  sleep(Duration::from_secs(30)).await;

  println!("disconnected");

  Ok(())
}
