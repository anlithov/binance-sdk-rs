use crate::rest::core::rate_limiter::ip_rate_limit_manager::{
	IpIntervalAndNum, IpRateLimitManager,
};
use crate::rest::endpoints::API;
use crate::result::AnyhowResult;
use anyhow::anyhow;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct IpRateIntervalSetup {
  pub weight_count: u64,
  pub weight_limit: u64,
}

impl IpRateLimitManager {
  /// Get the weight count for a specific interval in this period
  pub async fn weight_count_this_period(&self, interval: &IpIntervalAndNum) -> AnyhowResult<u64> {
    let count_rates = self.intervals.get(interval);

    if let Some(count_rates) = count_rates {
      Ok(*count_rates.weight_count.lock().await)
    } else {
      Err(anyhow!("Interval not found"))
    }
  }

  /// Get the weight count for a specific interval in this period
  pub async fn weight_limit_this_period(&self, interval: &IpIntervalAndNum) -> AnyhowResult<u64> {
    let count_rates = self.intervals.get(interval);

    if let Some(count_rates) = count_rates {
      Ok(*count_rates.weight_count.lock().await)
    } else {
      Err(anyhow!("Interval not found"))
    }
  }

  /// Get existing periods
  pub async fn intervals(&self) -> AnyhowResult<Vec<IpIntervalAndNum>> {
    Ok(self.intervals.keys().cloned().collect())
  }

  /// Get the weight count for a specific interval in this period
  pub async fn weight_all_periods(
    &self,
  ) -> AnyhowResult<HashMap<IpIntervalAndNum, IpRateIntervalSetup>> {
    let mut map = HashMap::new();

    for (key, value) in self.intervals.iter() {
      map.insert(
        key.clone(),
        IpRateIntervalSetup {
          weight_count: *value.weight_count.lock().await,
          weight_limit: *value.weight_limit.lock().await,
        },
      );
    }

    Ok(map)
  }

  /// Get the weight for a specific API endpoint and query
  pub fn calc_endpoint_weight(&self, api: &API, query: Option<String>) -> u64 {
    // First check if we have a specific calculator for this endpoint
    if let Some(calculator) = self.endpoint_weight_calculators.get(api) {
      return calculator(api, query);
    }

    // If no specific calculator, use the default calculator for the API type
    match api {
      API::SpotV3(_) => {
        if let Some(calculator) = self.default_calculators.get("SpotV3") {
          return calculator(api, query);
        }
      }
      // Handle other API types
      _ => {}
    }

    // Fallback to default weight of 1
    1
  }
}
