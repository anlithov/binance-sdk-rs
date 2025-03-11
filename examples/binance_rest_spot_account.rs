use binance::client::*;
use binance::rest::spot::v3::account::*;
use binance::rest::spot::v3::market::SpotMarketV3Manager;
use binance::rest::spot::v3::trade::SpotTradeV3Manager;
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

  /*  account_balances_and_info_example().await;*/
  /* trade_order_action_example().await;*/
  account_order_action_example().await;
}

#[allow(dead_code)]
async fn account_balances_and_info_example() {
  let api_key = Some(env::var("API_KEY").unwrap_or("YOUR_API_KEY".into()));
  let secret_key = Some(env::var("API_SECRET").unwrap_or("YOUR_API_KEY".into()));

  let account_v3: SpotAccountManagerV3 = Binance::new(api_key, secret_key);

  handle_result!(
    // Get current account_general information.
    account_v3.fetch_info_summary().await
  );
  handle_result!(
    // Get current ALL non-zero account_general balances.
    // *Only free or locked > 0
    account_v3.list_balances().await
  );
  handle_result!(
    // Get current FREE account_general balances.
    // *Only free > 0
    account_v3.list_balances_free().await
  );
  handle_result!(
    // Get current LOCKED account_general balances.
    // *Only locked > 0
    account_v3.list_balances_locked().await
  );
  handle_result!(
    // Get balance for a single Asset
    account_v3.fetch_balance_by_coin("USDT").await
  );
}

async fn account_order_info_example() {
  let api_key = Some(env::var("API_KEY").unwrap_or("YOUR_API_KEY".into()));
  let secret_key = Some(env::var("API_SECRET").unwrap_or("YOUR_API_KEY".into()));

  let trade: SpotTradeV3Manager = Binance::new(api_key, secret_key);

  handle_result!(
    //
    trade.list_all_open_orders().await
  );
  handle_result!(
    //
    trade.list_open_orders_by_symbol("HBARUSDT").await
  );
}

async fn account_order_action_example() {
  let api_key = Some(env::var("API_KEY").unwrap_or("YOUR_API_KEY".into()));
  let secret_key = Some(env::var("API_SECRET").unwrap_or("YOUR_API_KEY".into()));

  let market: SpotMarketV3Manager = Binance::new(api_key.clone(), secret_key.clone());

  handle_result!(
    //
    market.fetch_instrument_info("HBARUSDT").await
  );

  let trade: SpotTradeV3Manager = Binance::new(api_key, secret_key);

  handle_result!(
    //
    trade
      .place_market_buy_order_with_quote_quantity("HBARUSDT", 0.3)
      .await
  );
  handle_result!(
    //
    trade.place_market_sell_order("HBARUSDT", 30).await
  );
}
