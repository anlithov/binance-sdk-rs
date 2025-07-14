use crate::rest::core::rate_limiter::order_rate_limit_manager::{
  OrderIntervalAndNum, OrderRateLimitInterval, OrderRateLimitManager,
};
use crate::result::AnyhowResult;
use std::sync::Arc;
use tokio::sync::Mutex;

impl OrderRateLimitManager {
  pub fn add_interval_and_limit(&mut self, interval: OrderIntervalAndNum, limit: u64) {
    self.intervals.insert(
      interval,
      OrderRateLimitInterval {
        order_limit: Arc::new(Mutex::new(limit)),
        order_count: Arc::new(Mutex::new(0)),
      },
    );
  }

  pub async fn set_order_limit(
    &self,
    interval: &OrderIntervalAndNum,
    weight_limit: u64,
  ) -> AnyhowResult<()> {
    if let Some(set) = self.intervals.get(interval) {
      let mut ff = set.order_limit.lock().await;
      *ff = weight_limit
    }

    Ok(())
  }

  /// Set the current available weights based on actual usage from headers
  pub async fn set_order_count(
    &self,
    interval: &OrderIntervalAndNum,
    used_weight: u64,
  ) -> AnyhowResult<()> {
    if let Some(set) = self.intervals.get(interval) {
      let mut ff = set.order_count.lock().await;
      *ff = used_weight
    }

    Ok(())
  }
}
