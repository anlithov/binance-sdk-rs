use binance::client::*;
use binance::config::Config;
use binance::rest::core::rate_limiter::ip_rate_limit_manager::IpRateLimitManager;
use binance::rest::core::rate_limiter::unfilled_order_rate_limit_manager::UnfilledOrderRateLimitManager;
use binance::rest::spot::v3::account::SpotAccountManagerV3;
use binance::rest::spot::v3::general::GeneralManagerV3;
use binance::rest::spot::v3::market::SpotMarketV3Manager;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

pub type AnyhowResult<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> AnyhowResult<()> {
  dotenv().expect("Failed to read .env file");

  println!("=========");
  println!("Initing rate limiter");
  let (ip_rate_limiter, unfilled_order_rate_limiter) = create_and_log_rate_limiter().await?;
  print_rate_limits(ip_rate_limiter.clone(), unfilled_order_rate_limiter.clone()).await?;
  println!("=========");
  // Set up custom rate limits for demonstration
  let config = Config::default()
    .set_ip_rate_limit_manager(ip_rate_limiter.clone())
    .set_unfilled_order_rate_limit_manager(unfilled_order_rate_limiter.clone());

  let api_key = Some(env::var("API_KEY").unwrap_or("YOUR_API_KEY".into()));
  let secret_key = Some(env::var("API_SECRET").unwrap_or("YOUR_API_KEY".into()));

  let general_client =
    GeneralManagerV3::new_with_config(api_key.clone(), secret_key.clone(), &config);
  // Make a request to ping the server (lightweight request)
  println!("Making a ping request");
  general_client.try_ping().await?;
  print_rate_limits(ip_rate_limiter.clone(), unfilled_order_rate_limiter.clone()).await?;
  println!("=========");

  let account_client =
    SpotAccountManagerV3::new_with_config(api_key.clone(), secret_key.clone(), &config);
  println!("\nMaking exchange info request (weight: 40)...");
  account_client.fetch_rate_limits_for_orders().await?;

  print_rate_limits(ip_rate_limiter.clone(), unfilled_order_rate_limiter.clone()).await?;
  println!("=========");

  let client = SpotMarketV3Manager::new_with_config(api_key, secret_key, &config);
  println!("\nMaking exchange info request (weight: 20)...");
  client.fetch_general_exchange_info().await?;

  print_rate_limits(ip_rate_limiter.clone(), unfilled_order_rate_limiter.clone()).await?;
  println!("=========");

  Ok(())
}

async fn create_and_log_rate_limiter()
-> AnyhowResult<(Arc<IpRateLimitManager>, Arc<UnfilledOrderRateLimitManager>)> {
  let api_key = dotenvy::var("API_KEY").unwrap_or("YOUR_API_KEY".into());
  let secret_key = dotenvy::var("API_SECRET").unwrap_or("YOUR_API_KEY".into());

  let ip_rate_limiter = IpRateLimitManager::new().await?;
  println!("Ip rate limiter initialized");
  let unfilled_order_rate_limiter =
    UnfilledOrderRateLimitManager::new(api_key, secret_key, Some(ip_rate_limiter.clone())).await?;
  println!("Order rate limiter initialized");

  print_rate_limits(ip_rate_limiter.clone(), unfilled_order_rate_limiter.clone()).await?;

  Ok((ip_rate_limiter, unfilled_order_rate_limiter))
}

async fn print_rate_limits(
  ip_rate_limiter: Arc<IpRateLimitManager>,
  order_limiter: Arc<UnfilledOrderRateLimitManager>,
) -> AnyhowResult<()> {
  let ip_rate_limits = ip_rate_limiter.weight_all_periods().await?;
  let order_rate_limits = order_limiter.orders_all_periods().await?;

  println!("Ip Rate limits: {:#?}", ip_rate_limits);

  Ok(())
}
