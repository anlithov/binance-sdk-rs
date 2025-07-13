use crate::rest::core::rate_limiter::ip_rate_limit_manager::{
  IpIntervalAndNum, IpRateLimitManager,
};
use crate::result::AnyhowResult;

impl IpRateLimitManager {
  pub async fn update_weight_limit(
    &self,
    interval: &IpIntervalAndNum,
    weight_limit: u64,
  ) -> AnyhowResult<()> {
    if let Some(set) = self.intervals.get(interval) {
      let mut ff = set.weight_limit.lock().await;
      *ff = weight_limit
    }

    Ok(())
  }

  /// Set the current available weights based on actual usage from headers
  pub async fn update_used_weight(
    &self,
    interval: &IpIntervalAndNum,
    used_weight: u64,
  ) -> AnyhowResult<()> {
    if let Some(set) = self.intervals.get(interval) {
      let mut ff = set.weight_used.lock().await;
      *ff = used_weight
    }

    Ok(())
  }
}
