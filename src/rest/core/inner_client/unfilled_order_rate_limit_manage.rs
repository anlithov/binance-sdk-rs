use crate::rest::core::inner_client::InnerClient;
use crate::rest::core::rate_limiter::unfilled_order_rate_limit_manager::{
  OrderIntervalAndNum, UnfilledOrderRateLimitManager,
};
use crate::rest::endpoints::API;
use crate::result::AnyhowResult;
use reqwest::header::HeaderMap;
use std::sync::Arc;

impl InnerClient {
  /// Get rate limit manager
  pub fn unfilled_order_rate_limiter_manager(&self) -> Option<Arc<UnfilledOrderRateLimitManager>> {
    self.unfilled_order_rate_limit_manager.clone()
  }

  pub(crate) async fn acquire_ip_and_order_limits_permit(
    &self,
    endpoint: &API,
    query: Option<String>,
  ) -> AnyhowResult<()> {
    self.acquire_ip_limit_permit(endpoint, query).await?;

    if let Some(order_rate_limiter) = &self.unfilled_order_rate_limit_manager {
      order_rate_limiter.acquire().await?;
    }

    Ok(())
  }
}

pub(crate) async fn extract_and_update_order_count(
  unfilled_order_rate_limit_manager: &Option<Arc<UnfilledOrderRateLimitManager>>,
  headers: &HeaderMap,
) -> AnyhowResult<()> {
  if let Some(unfilled_order_limit_manager) = unfilled_order_rate_limit_manager {
    // Look for all headers starting with X-MBX-USED-WEIGHT
    for (header_name, header_value) in headers.iter() {
      let header_name_str = header_name.as_str().to_lowercase();
      // Check for X-MBX-USED-WEIGHT-(intervalNum)(intervalLetter) pattern
      if header_name_str.starts_with("X-mbx-order-count-") {
        // Extract the weight value from the header
        if let Ok(weight_str) = header_value.to_str() {
          if let Ok(weight) = weight_str.parse::<u64>() {
            // Parse the interval from the header name
            if let Some(interval) = OrderIntervalAndNum::from_header_name(&header_name_str) {
              // Update the used weight in the rate limiter with the specific interval
              if let Err(e) = unfilled_order_limit_manager
                .set_order_count(&interval, weight)
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
