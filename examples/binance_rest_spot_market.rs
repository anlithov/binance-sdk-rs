use binance::client::*;
use binance::rest::spot::v3::market::SpotMarketV3Manager;
use binance::rest::spot::v3::market::enums::KlineInterval;
use binance::rest::spot::v3::market::requests::KlinesRequest;
use dotenvy::dotenv;
use std::env;

macro_rules! handle_result {
  ($expression:expr) => {
    match $expression {
      Ok(answer) => println!("{:#?}", answer),
      Err(e) => println!("Error: {:?}", e),
    }
  };
}

#[tokio::main]
async fn main() {
  dotenv().expect("Failed to read .env file");

  market_example().await;
}

async fn market_example() {
  let api_key = Some(env::var("API_KEY").unwrap_or("YOUR_API_KEY".into()));
  let secret_key = Some(env::var("API_SECRET").unwrap_or("YOUR_API_KEY".into()));

  let trade: SpotMarketV3Manager = Binance::new(api_key, secret_key);

  handle_result!(
    //
    trade
      .list_klines_custom(KlinesRequest {
        symbol: "BTCUSDT".into(),
        interval: KlineInterval::Day1,
        end_time: None,
        start_time: None,
        limit: Some(5)
      })
      .await
  );
}
