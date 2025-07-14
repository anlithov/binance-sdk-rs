use crate::rest::core::rate_limiter::ip_rate_limit_manager::{
  IpRateLimitManager, WeightCalculator,
};
use crate::rest::endpoints::{AccountGeneral, Futures, Savings, SpotV3, API};

impl IpRateLimitManager {
  /// Initialize weight calculator functions for different API endpoints
  /// Weights are based on Binance API documentation
  /// https://binance.github.io/binance-api-swagger/spot_api.yaml
  pub(crate) fn init_weight_calculators(&mut self) {
    // First set default calculators
    self.set_default_calculators();

    // === SPECIFIC ENDPOINT CALCULATORS ===

    // Depth API - weight depends on limit parameter
    self
      .endpoint_weight_calculators
      .insert(API::SpotV3(SpotV3::Depth), |_, query| {
        // Default to higher weight if we can't parse
        if query.is_none() {
          return 5;
        }

        // Parse limit from query
        let query = query.unwrap();
        if let Some(limit_str) = Self::extract_param(query, "limit") {
          if let Ok(limit) = limit_str.parse::<u64>() {
            // Apply weights based on limit ranges
            match limit {
              0..=100 => 1,
              101..=500 => 5,
              501..=1000 => 10,
              1001..=5000 => 50,
              _ => 100, // Very high weight for anything larger
            }
          } else {
            5 // Default weight if we can't parse the limit
          }
        } else {
          1 // Default limit is usually 100, which is weight 1
        }
      });

    // Ticker24hr - weight depends on if symbol is specified
    self
      .endpoint_weight_calculators
      .insert(API::SpotV3(SpotV3::Ticker24hr), |_, query| {
        if let Some(query_str) = query {
          // If symbol is specified, weight is 1
          if query_str.contains("symbol=") {
            1
          } else {
            // Without symbol, or with symbols parameter, weight is 40
            40
          }
        } else {
          40 // Default to higher weight if no query
        }
      });

    // Price - weight depends on if symbol is specified
    self
      .endpoint_weight_calculators
      .insert(API::SpotV3(SpotV3::Price), |_, query| {
        if let Some(query_str) = query {
          // If symbol is specified, weight is 1
          if query_str.contains("symbol=") {
            1
          } else {
            // Without symbol, or with symbols parameter, weight is 2
            2
          }
        } else {
          2 // Default to higher weight if no query
        }
      });

    // BookTicker - weight depends on if symbol is specified
    self
      .endpoint_weight_calculators
      .insert(API::SpotV3(SpotV3::BookTicker), |_, query| {
        if let Some(query_str) = query {
          // If symbol is specified, weight is 1
          if query_str.contains("symbol=") {
            1
          } else {
            // Without symbol, or with symbols parameter, weight is 2
            2
          }
        } else {
          2 // Default to higher weight if no query
        }
      });

    // OpenOrders - weight depends on if symbol is specified
    self
      .endpoint_weight_calculators
      .insert(API::SpotV3(SpotV3::OpenOrders), |_, query| {
        if let Some(query_str) = query {
          // If symbol is specified, weight is 1
          if query_str.contains("symbol=") {
            1
          } else {
            // Without symbol weight is 3
            3
          }
        } else {
          3 // Default to higher weight if no query
        }
      });

    // Futures Depth API - weight depends on limit parameter (similar to Spot)
    self
      .endpoint_weight_calculators
      .insert(API::Futures(Futures::Depth), |_, query| {
        // Default to higher weight if we can't parse
        if query.is_none() {
          return 5;
        }

        // Parse limit from query
        let query = query.unwrap();
        if let Some(limit_str) = Self::extract_param(query, "limit") {
          if let Ok(limit) = limit_str.parse::<u32>() {
            // Apply weights based on limit ranges
            match limit {
              0..=100 => 1,
              101..=500 => 5,
              501..=1000 => 10,
              1001..=5000 => 50,
              _ => 100, // Very high weight for anything larger
            }
          } else {
            5 // Default weight if we can't parse the limit
          }
        } else {
          1 // Default limit is usually 100, which is weight 1
        }
      });

    // Futures Ticker24hr - weight depends on if symbol is specified
    self
      .endpoint_weight_calculators
      .insert(API::Futures(Futures::Ticker24hr), |_, query| {
        if let Some(query_str) = query {
          // If symbol is specified, weight is 1
          if query_str.contains("symbol=") {
            1
          } else {
            // Without symbol, or with symbols parameter, weight is 40
            40
          }
        } else {
          40 // Default to higher weight if no query
        }
      });

    // Futures TickerPrice - weight depends on if symbol is specified
    self
      .endpoint_weight_calculators
      .insert(API::Futures(Futures::TickerPrice), |_, query| {
        if let Some(query_str) = query {
          // If symbol is specified, weight is 1
          if query_str.contains("symbol=") {
            1
          } else {
            // Without symbol, or with symbols parameter, weight is 2
            2
          }
        } else {
          2 // Default to higher weight if no query
        }
      });

    // Futures BookTicker - weight depends on if symbol is specified
    self
      .endpoint_weight_calculators
      .insert(API::Futures(Futures::BookTicker), |_, query| {
        if let Some(query_str) = query {
          // If symbol is specified, weight is 1
          if query_str.contains("symbol=") {
            1
          } else {
            // Without symbol, or with symbols parameter, weight is 2
            2
          }
        } else {
          2 // Default to higher weight if no query
        }
      });
  }

  /// Set default weight calculators by API type
  fn set_default_calculators(&mut self) {
    // Default calculator for Spot V3 endpoints
    let spot_v3_calculator: WeightCalculator = |endpoint, _| {
      match endpoint {
        API::SpotV3(variant) => {
          match variant {
            // System endpoints
            SpotV3::Ping => 1,
            SpotV3::Time => 1,
            SpotV3::ExchangeInfo => 10,

            // Market Data endpoints - weights vary based on parameters
            // Note: Some endpoints have specific calculators that override these defaults
            SpotV3::Trades => 5,
            SpotV3::HistoricalTrades => 5,
            SpotV3::AggTrades => 1,
            SpotV3::Klines => 1,
            SpotV3::AvgPrice => 1,

            // These endpoints have dedicated calculators for accurate weight calculation
            // but we provide defaults here as fallback
            SpotV3::Depth => 5,
            SpotV3::Ticker24hr => 40,
            SpotV3::Price => 2,
            SpotV3::BookTicker => 2,

            // Account and Order endpoints
            SpotV3::Order => 1,
            SpotV3::OrderTest => 1,
            SpotV3::OpenOrders => 3, // 3 with no symbol, 1 with symbol
            SpotV3::AllOrders => 10,
            SpotV3::Oco => 1,
            SpotV3::OrderList => 2,
            SpotV3::AllOrderList => 10,
            SpotV3::OpenOrderList => 3,
            SpotV3::Account => 10,
            SpotV3::MyTrades => 10,
            SpotV3::RateLimitOrder => 20,
            SpotV3::MyPreventedMatches => 10,
            SpotV3::MyAllocations => 10,
            SpotV3::AccountCommissions => 20,

            // User Stream endpoints
            SpotV3::UserDataStream => 1,
          }
        }
        _ => 1, // Should never happen
      }
    };

    // Default calculator for Savings endpoints
    let savings_calculator: WeightCalculator = |endpoint, _| {
      match endpoint {
        API::Savings(variant) => match variant {
          Savings::AllCoins => 10,
          Savings::AssetDetail => 1,
          Savings::DepositAddress => 10,
          Savings::SpotFuturesTransfer => 1,
        },
        _ => 1, // Should never happen
      }
    };

    // Default calculator for Futures endpoints
    let futures_calculator: WeightCalculator = |endpoint, _| {
      match endpoint {
        API::Futures(variant) => {
          match variant {
            // System endpoints
            Futures::Ping => 1,
            Futures::Time => 1,
            Futures::ExchangeInfo => 1,

            // Market data endpoints
            Futures::Depth => 5, // Variable weight like spot
            Futures::Trades => 5,
            Futures::HistoricalTrades => 20,
            Futures::AggTrades => 20,
            Futures::Klines => 1,
            Futures::ContinuousKlines => 1,
            Futures::IndexPriceKlines => 1,
            Futures::MarkPriceKlines => 1,
            Futures::PremiumIndex => 1,
            Futures::FundingRate => 1,

            // Ticker endpoints
            Futures::Ticker24hr => 40, // 40 when no symbol
            Futures::TickerPrice => 2, // 2 when no symbol
            Futures::BookTicker => 2,  // 2 when no symbol

            // Account/Trading endpoints
            Futures::Order => 1,
            Futures::AllForceOrders => 20,
            Futures::AllOpenOrders => 40,
            Futures::AllOrders => 5,
            Futures::OpenOrders => 1,
            Futures::UserTrades => 5,
            Futures::PositionSide => 1,
            Futures::PositionRisk => 5,
            Futures::Balance => 5,
            Futures::Account => 5,
            Futures::ChangeInitialLeverage => 1,
            Futures::MarginType => 1,
            Futures::PositionMargin => 1,
            Futures::Income => 30,

            // Data endpoints
            Futures::OpenInterest => 1,
            Futures::OpenInterestHist => 1,
            Futures::TopLongShortAccountRatio => 1,
            Futures::TopLongShortPositionRatio => 1,
            Futures::GlobalLongShortAccountRatio => 1,
            Futures::TakerlongshortRatio => 1,
            Futures::LvtKlines => 1,
            Futures::IndexInfo => 1,

            // User Stream
            Futures::UserDataStream => 1,
          }
        }
        _ => 1, // Should never happen
      }
    };

    // Default calculator for Account General endpoints
    let account_general_calculator: WeightCalculator = |endpoint, _| {
      match endpoint {
        API::AccountGeneral(variant) => match variant {
          AccountGeneral::ApiRestrictions => 1,
        },
        _ => 1, // Should never happen
      }
    };

    // Store default calculators by type
    self
      .default_calculators
      .insert("SpotV3".to_string(), spot_v3_calculator);
    self
      .default_calculators
      .insert("Savings".to_string(), savings_calculator);
    self
      .default_calculators
      .insert("Futures".to_string(), futures_calculator);
    self
      .default_calculators
      .insert("AccountGeneral".to_string(), account_general_calculator);
  }

  /// Helper function to extract a parameter value from a query string
  fn extract_param(query: String, param_name: &str) -> Option<String> {
    let param_prefix = format!("{}=", param_name);
    let parts: Vec<&str> = query.split('&').collect();

    for part in parts {
      if part.starts_with(&param_prefix) {
        return Some(part[param_prefix.len()..].to_string());
      }
    }

    None
  }
}
