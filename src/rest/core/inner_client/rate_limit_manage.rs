use crate::rest::core::inner_client::InnerClient;
use crate::rest::core::rate_limiter::RateLimitManager;
use crate::rest::core::rate_limiter::ip_rate_limit_manager::IpIntervalAndNum;
use crate::rest::endpoints::{SpotV3, API};
use crate::result::AnyhowResult;
use reqwest::header::HeaderMap;
use std::sync::Arc;

impl InnerClient {
  /// Get rate limit manager
  pub fn rate_limit_manager(&self) -> Option<Arc<RateLimitManager>> {
    self.rate_limit_manager.clone()
  }

  pub(crate) async fn acquire_ip_limit_permit(
    &self,
    endpoint: &API,
    query: Option<String>,
  ) -> AnyhowResult<()> {
    if let Some(rate_limiter) = &self.rate_limit_manager {
      rate_limiter
        .ip_rate_limiter()
        .acquire(&endpoint, query)
        .await?;
    }

    Ok(())
  }

  pub(crate) async fn acquire_ip_and_order_limits_permit(
    &self,
    endpoint: &API,
    query: Option<String>,
  ) -> AnyhowResult<()> {
    self.acquire_ip_limit_permit(endpoint, query).await?;

    if let Some(rate_limiter) = &self.rate_limit_manager {
      if self.is_order_endpoint(&endpoint) {
        rate_limiter.order_rate_limiter().acquire().await?;
      }
    }

    Ok(())
  }

  /// Check if the endpoint is order-related (subject to order rate limiting)
  fn is_order_endpoint(&self, endpoint: &API) -> bool {
    match endpoint {
      API::SpotV3(spot_v3) => matches!(
        spot_v3,
        SpotV3::Order
          | SpotV3::OrderTest
          | SpotV3::Oco
          | SpotV3::OrderList
          | SpotV3::OpenOrderList
          | SpotV3::AllOrderList
      ),
      _ => false,
    }
  }

  pub(crate) async fn extract_and_update_ip_used_weight(
    &self,
    headers: &HeaderMap,
  ) -> AnyhowResult<()> {
    if let Some(rate_limiter) = &self.rate_limit_manager {
      // Look for all headers starting with X-MBX-USED-WEIGHT
      for (header_name, header_value) in headers.iter() {
        let header_name_str = header_name.as_str().to_lowercase();
        // Check for X-MBX-USED-WEIGHT-(intervalNum)(intervalLetter) pattern
        if header_name_str.starts_with("x-mbx-used-weight-") {
          // Extract the weight value from the header
          if let Ok(weight_str) = header_value.to_str() {
            if let Ok(weight) = weight_str.parse::<u64>() {
              // Parse the interval from the header name
              if let Some(interval) = IpIntervalAndNum::from_header_name(&header_name_str) {
                // Update the used weight in the rate limiter with the specific interval
                if let Err(e) = rate_limiter
                  .ip_rate_limiter()
                  .update_used_weight(&interval, weight)
                  .await
                {
                  eprintln!(
                    "Failed to update used weight from header {}: {}",
                    header_name_str, e
                  );
                }
              } else {
                eprintln!(
                  "Failed to parse interval from header name: {}",
                  header_name_str
                );
              }
            } else {
              eprintln!(
                "Failed to parse weight value from header {}: {}",
                header_name_str,
                header_value.to_str().unwrap_or_default()
              );
            }
          }
        }
      }
    }

    Ok(())
  }
}
