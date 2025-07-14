use binance::client::*;
use binance::config::Config;
use binance::rest::core::rate_limiter::RateLimitManager;
use binance::rest::spot::v3::account::SpotAccountManagerV3;
use binance::rest::spot::v3::general::GeneralManagerV3;
use binance::rest::spot::v3::market::SpotMarketV3Manager;
use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;

pub type AnyhowResult<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> AnyhowResult<()> {
  dotenv().expect("Failed to read .env file");

  println!("=========");
  println!("Initing rate limiter");
  let rate_limiter = create_and_log_rate_limiter().await?;
  print_rate_limits(rate_limiter.clone()).await?;
  println!("=========");
  // Set up custom rate limits for demonstration
  let config = Config::default().set_rate_limit_manager(rate_limiter.clone());

  let api_key = Some(env::var("API_KEY").unwrap_or("YOUR_API_KEY".into()));
  let secret_key = Some(env::var("API_SECRET").unwrap_or("YOUR_API_KEY".into()));

  let general_client =
    GeneralManagerV3::new_with_config(api_key.clone(), secret_key.clone(), &config);
  // Make a request to ping the server (lightweight request)
  println!("Making a ping request");
  general_client.try_ping().await?;
  print_rate_limits(rate_limiter.clone()).await?;
  println!("=========");

  let account_client =
    SpotAccountManagerV3::new_with_config(api_key.clone(), secret_key.clone(), &config);
  println!("\nMaking exchange info request (weight: 40)...");
  account_client.fetch_rate_limits_for_orders().await?;

  print_rate_limits(rate_limiter.clone()).await?;
  println!("=========");

  let client = SpotMarketV3Manager::new_with_config(api_key, secret_key, &config);
  println!("\nMaking exchange info request (weight: 20)...");
  client.fetch_general_exchange_info().await?;

  print_rate_limits(rate_limiter.clone()).await?;
  println!("=========");

  Ok(())
}

async fn create_and_log_rate_limiter() -> AnyhowResult<Arc<RateLimitManager>> {
  let api_key = dotenvy::var("API_KEY").unwrap_or("YOUR_API_KEY".into());
  let secret_key = dotenvy::var("API_SECRET").unwrap_or("YOUR_API_KEY".into());

  let rate_limiter = RateLimitManager::new()
    .with_ip_rate_limiter()
    .await?
    .with_order_rate_limiter(api_key, secret_key)
    .await?;
  println!("Rate limiter initialized");

  let arcc = Arc::new(rate_limiter);
  print_rate_limits(arcc.clone()).await?;

  Ok(arcc)
}

async fn print_rate_limits(rate_limiter: Arc<RateLimitManager>) -> AnyhowResult<()> {
  let ip_rate_limits = if let Some(ip_rate) = rate_limiter.ip_rate_limiter() {
    ip_rate.weight_all_periods().await?
  } else {
    HashMap::new()
  };
  let order_rate_limits = if let Some(order_rate) = rate_limiter.order_rate_limiter() {
    order_rate.orders_all_periods().await?
  } else {
    HashMap::new()
  };

  println!("Order Rate limits: {:#?}", order_rate_limits);
  println!("Ip Rate limits: {:#?}", ip_rate_limits);

  Ok(())
}
