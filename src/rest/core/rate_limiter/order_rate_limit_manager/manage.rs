use crate::rest::core::rate_limiter::order_rate_limit_manager::{
	OrderIntervalAndNum, OrderRateLimitInterval, OrderRateLimitManager,
};
use std::sync::Arc;
use tokio::sync::Mutex;

impl OrderRateLimitManager {
  pub fn add_interval_and_limit(&mut self, interval: OrderIntervalAndNum, limit: u64) {
    self.intervals.insert(
      interval,
      OrderRateLimitInterval {
        limit: Arc::new(Mutex::new(limit)),
        count_used: Arc::new(Mutex::new(0)),
      },
    );
  }
}
