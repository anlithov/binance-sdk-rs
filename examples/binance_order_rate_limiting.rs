use anyhow::Result;
use binance::client::*;
use binance::config::Config;
use binance::rest::spot::v3::trade::SpotTradeV3Manager;
use dotenvy::dotenv;
use std::env;
use tokio::time::{sleep, Duration};

pub type AnyhowResult<T> = Result<T>;

#[tokio::main]
async fn main() -> AnyhowResult<()> {
  dotenv().expect("Failed to read .env file");

  // Set up custom rate limits for demonstration
  let config = Config::default().with_order_rate_limiting(); // 3/sec, 10/min, 50/day
  println!("{:?}", config);

  let api_key = Some(env::var("API_KEY").unwrap_or("YOUR_API_KEY".into()));
  let secret_key = Some(env::var("API_SECRET").unwrap_or("YOUR_API_KEY".into()));

  let trade_client = SpotTradeV3Manager::new_with_config(api_key, secret_key, &config).await?;

  // First, test the per-second limit
  println!("Testing orders per second limit (3 orders/sec)...");
  test_order_rate_second_limit(&trade_client).await?;

  // Wait a bit to reset counters
  sleep(Duration::from_secs(5)).await;

  // Then test the per-minute limit
  println!("\nTesting orders per minute limit (10 orders/min)...");
  test_order_rate_minute_limit(&trade_client).await?;

  Ok(())
}

async fn test_order_rate_second_limit(trade_client: &SpotTradeV3Manager) -> AnyhowResult<()> {
  // Try to place 5 orders in quick succession to trigger the per-second rate limit
  for i in 1..=5 {
    println!("Attempting to place order #{}", i);

    let result = trade_client
      .test_place_limit_buy_order("BTCUSDT".to_string(), 0.001, 110000)
      .await;

    match result {
      Ok(_) => println!("Order #{} placed successfully", i),
      Err(e) => println!("Order #{} failed: {}", i, e),
    }

    // No artificial delay for the first 3 orders to demonstrate rate limiting
    if i >= 3 {
      // Add a small delay after hitting the rate limit
      sleep(Duration::from_millis(500)).await;
    }
  }

  Ok(())
}

async fn test_order_rate_minute_limit(trade_client: &SpotTradeV3Manager) -> AnyhowResult<()> {
  // Try to place 12 orders with a small delay to trigger the per-minute rate limit
  for i in 1..=12 {
    println!("Attempting to place order #{}", i);

    let result = trade_client
      .test_place_limit_buy_order("BTCUSDT".to_string(), 0.001, 110000)
      .await;

    match result {
      Ok(_) => println!("Order #{} placed successfully", i),
      Err(e) => println!("Order #{} failed: {}", i, e),
    }

    // Add a delay between orders to avoid per-second limit but still hit per-minute limit
    sleep(Duration::from_millis(1500)).await;
  }

  Ok(())
}
