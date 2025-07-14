use crate::rest::core::rate_limiter::ip_rate_limit_manager::{
  IpIntervalAndNum, IpIntervalSetup, IpRateLimitManager,
};
use crate::result::AnyhowResult;
use std::sync::Arc;
use tokio::sync::Mutex;

impl IpRateLimitManager {
  pub(crate) fn add_interval_and_limit(&mut self, interval: IpIntervalAndNum, weight_limit: u64) {
    self.intervals.insert(
      interval,
      IpIntervalSetup {
        weight_limit: Arc::new(Mutex::new(weight_limit)),
        weight_count: Arc::new(Mutex::new(0)),
      },
    );
  }

  pub(crate) async fn set_weight_limit(
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
  pub(crate) async fn set_weight_count(
    &self,
    interval: &IpIntervalAndNum,
    used_weight: u64,
  ) -> AnyhowResult<()> {
    if let Some(set) = self.intervals.get(interval) {
      let mut ff = set.weight_count.lock().await;
      *ff = used_weight
    }

    Ok(())
  }
}
