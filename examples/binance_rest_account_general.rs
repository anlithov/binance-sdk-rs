use binance::client::Binance;
use binance::rest::account_general::v1::AccountGeneralManagerV1;
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

  account_api_restrictions_example().await;
  /* trade_order_action_example().await;*/
}

#[allow(dead_code)]
async fn account_api_restrictions_example() {
  let api_key = Some(env::var("API_KEY").unwrap_or("YOUR_API_KEY".into()));
  let secret_key = Some(env::var("API_SECRET").unwrap_or("YOUR_API_KEY".into()));

  let account_v1: AccountGeneralManagerV1 = Binance::new(api_key, secret_key);

  handle_result!(
    // Get current account_general information.
    account_v1.fetch_api_restrictions().await
  );
}
