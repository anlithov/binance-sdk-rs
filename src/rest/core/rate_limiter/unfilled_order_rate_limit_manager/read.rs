use crate::rest::core::rate_limiter::unfilled_order_rate_limit_manager::{
  OrderIntervalAndNum, UnfilledOrderRateLimitManager,
};
use crate::result::AnyhowResult;
use anyhow::anyhow;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OrderRateIntervalSetup {
  pub order_count: u64,
  pub order_limit: u64,
}

impl UnfilledOrderRateLimitManager {
  /// Get the weight count for a specific interval in this period
  pub async fn order_count_this_period(&self, interval: &OrderIntervalAndNum) -> AnyhowResult<u64> {
    let count_rates = self.intervals.get(interval);

    if let Some(count_rates) = count_rates {
      Ok(*count_rates.order_count.lock().await)
    } else {
      Err(anyhow!("Interval not found"))
    }
  }

  /// Get the weight count for a specific interval in this period
  pub async fn order_limit_this_period(&self, interval: &OrderIntervalAndNum) -> AnyhowResult<u64> {
    let count_rates = self.intervals.get(interval);

    if let Some(count_rates) = count_rates {
      Ok(*count_rates.order_count.lock().await)
    } else {
      Err(anyhow!("Interval not found"))
    }
  }

  /// Get existing periods
  pub async fn intervals(&self) -> AnyhowResult<Vec<OrderIntervalAndNum>> {
    Ok(self.intervals.keys().cloned().collect())
  }

  /// Get the weight count for a specific interval in this period
  pub async fn orders_all_periods(
    &self,
  ) -> AnyhowResult<HashMap<OrderIntervalAndNum, OrderRateIntervalSetup>> {
    let mut map = HashMap::new();

    for (key, value) in self.intervals.iter() {
      map.insert(
        key.clone(),
        OrderRateIntervalSetup {
          order_count: *value.order_count.lock().await,
          order_limit: *value.order_limit.lock().await,
        },
      );
    }

    Ok(map)
  }
}
