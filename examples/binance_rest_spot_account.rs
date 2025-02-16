use binance::client::*;
use binance::rest::spot::account::*;
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

  /* account_balances_and_info_example().await;*/
  /* account_order_action_example().await;*/
}

#[allow(dead_code)]
async fn account_balances_and_info_example() {
  let api_key = Some(env::var("API_KEY").unwrap_or("YOUR_API_KEY".into()));
  let secret_key = Some(env::var("API_SECRET").unwrap_or("YOUR_API_KEY".into()));

  let account: Account = Binance::new(api_key, secret_key);

  handle_result!(
    // Get current account information.
    account.info_summary().await
  );
  handle_result!(
    // Get current ALL non-zero account balances.
    // *Only free or locked > 0
    account.balances().await
  );
  handle_result!(
    // Get current FREE account balances.
    // *Only free > 0
    account.balances_free().await
  );
  handle_result!(
    // Get current LOCKED account balances.
    // *Only locked > 0
    account.balances_locked().await
  );
  handle_result!(
    // Get balance for a single Asset
    account.balance_single("USDT").await
  );
  handle_result!(
    //
    account.open_orders_all().await
  );
}

async fn account_order_action_example() {
  let api_key = Some(env::var("API_KEY").unwrap_or("YOUR_API_KEY".into()));
  let secret_key = Some(env::var("API_SECRET").unwrap_or("YOUR_API_KEY".into()));

  let account: Account = Binance::new(api_key, secret_key);

  handle_result!(
    //
    account.open_orders_all().await
  );
  handle_result!(
    //
    account.open_orders_by_symbol("HBARUSDT").await
  );
  handle_result!(
    //
    account.order_status("HBARUSDT", 2328238347_u64).await
  );
  handle_result!(
    //
    account.test_order_status("HBARUSDT", 2328238347_u64).await
  );
  handle_result!(
    //
    account.cancel_order_by_id("HBARUSDT", 2328238347_u64).await
  );
  handle_result!(
    //
    account.limit_sell("HBARUSDT", 20f64, 0.3).await
  );
  handle_result!(
    //
    account.test_limit_sell("HBARUSDT", 20f64, 0.3).await
  );
  handle_result!(
    //
    account.test_order_status("HBARUSDT", 2328238347_u64).await
  );
  handle_result!(
    //
    account.cancel_order_by_id("HBARUSDT", 2328238347_u64).await
  );
}
