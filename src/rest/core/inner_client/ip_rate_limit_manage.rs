use crate::rest::core::inner_client::InnerClient;
use crate::rest::core::rate_limiter::ip_rate_limit_manager::{
  IpIntervalAndNum, IpRateLimitManager,
};
use crate::rest::endpoints::API;
use crate::result::AnyhowResult;
use reqwest::header::HeaderMap;
use std::sync::Arc;

impl InnerClient {
  /// Get rate limit manager
  pub fn ip_rate_limit_manager(&self) -> Option<Arc<IpRateLimitManager>> {
    self.ip_rate_limit_manager.clone()
  }

  pub(crate) async fn acquire_ip_limit_permit(
    &self,
    endpoint: &API,
    query: Option<String>,
  ) -> AnyhowResult<()> {
    if let Some(ip_rate_limiter) = &self.ip_rate_limit_manager {
      ip_rate_limiter.acquire(&endpoint, query).await?;
    }

    Ok(())
  }
}

pub(crate) async fn extract_and_update_ip_weight_count(
  ip_rate_limit_manager: &Option<Arc<IpRateLimitManager>>,
  headers: &HeaderMap,
) -> AnyhowResult<()> {
  if let Some(ip_rate_limiter) = ip_rate_limit_manager {
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
              if let Err(e) = ip_rate_limiter.set_weight_count(&interval, weight).await {
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
