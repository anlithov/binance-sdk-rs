use crate::rest::core::rate_limiter::ip_rate_limit_manager::{
  IpIntervalAndNum, IpRateLimitManager,
};
use crate::rest::core::rate_limiter::order_rate_limit_manager::{
  OrderIntervalAndNum, OrderRateLimitManager,
};
use crate::rest::core::rate_limiter::RateLimitManager;
use crate::rest::spot::v3::market::responses::{RateLimitResponse, RateLimitTypeResponse};
use std::sync::Arc;

impl RateLimitManager {
  pub(crate) fn init_order_rate_limit_manager(
    exchange_info: &Vec<RateLimitResponse>,
  ) -> Arc<OrderRateLimitManager> {
    let mut order_rate_limit_manager = OrderRateLimitManager::new();

    for rate_limit in exchange_info {
      if RateLimitTypeResponse::Orders.eq(&rate_limit.rate_limit_type) {
        order_rate_limit_manager.add_interval_and_limit(
          OrderIntervalAndNum {
            interval: rate_limit.interval.clone(),
            interval_num: rate_limit.interval_num,
          },
          rate_limit.limit,
        );
      }
    }

    Arc::new(order_rate_limit_manager)
  }

  pub(crate) fn init_ip_rate_limit_manager(
    exchange_info: &Vec<RateLimitResponse>,
  ) -> Arc<IpRateLimitManager> {
    let mut ip_rate_limit_manager = IpRateLimitManager::new();

    for rate_limit in exchange_info {
      if RateLimitTypeResponse::RequestWeight.eq(&rate_limit.rate_limit_type) {
        ip_rate_limit_manager.add_interval_and_limit(
          IpIntervalAndNum {
            interval: rate_limit.interval.clone(),
            interval_num: rate_limit.interval_num,
          },
          rate_limit.limit,
        );
      }
    }

    Arc::new(ip_rate_limit_manager)
  }
}
