use crate::rest::core::inner_client::ip_rate_limit_manage::extract_and_update_ip_weight_count;
use crate::rest::core::inner_client::unfilled_order_rate_limit_manage::extract_and_update_order_count;
use crate::rest::core::rate_limiter::ip_rate_limit_manager::IpRateLimitManager;
use crate::rest::core::rate_limiter::unfilled_order_rate_limit_manager::UnfilledOrderRateLimitManager;
use crate::rest::endpoints::{SpotV3, API};
use crate::result::AnyhowResult;
use reqwest::header::HeaderMap;
use std::sync::Arc;

/// Check if the endpoint is order-related (subject to order rate limiting)
pub(crate) fn is_order_endpoint(endpoint: &API) -> bool {
  match endpoint {
    API::SpotV3(spot_v3) => matches!(spot_v3, SpotV3::Order | SpotV3::OrderTest | SpotV3::Oco),
    _ => false,
  }
}

pub(crate) async fn extract_and_update_rate_limiter_counts(
  ip_rate_limit_manager: &Option<Arc<IpRateLimitManager>>,
  unfilled_order_rate_limit_manager: &Option<Arc<UnfilledOrderRateLimitManager>>,
  headers: &HeaderMap,
  endpoint: &API,
) -> AnyhowResult<()> {
  extract_and_update_ip_weight_count(ip_rate_limit_manager, headers).await?;
  if is_order_endpoint(&endpoint) {
    extract_and_update_order_count(unfilled_order_rate_limit_manager, headers).await?;
  }

  Ok(())
}
