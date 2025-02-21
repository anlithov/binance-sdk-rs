use super::responses::{AccountInformationResponse, AssetBalanceResponse};
use super::SpotAccountManagerV3;
use crate::rest::endpoints::{SpotV3, API};
use crate::util::build_signed_query;
use anyhow::bail;
use anyhow::Result;
use std::collections::BTreeMap;

// Balances
impl SpotAccountManagerV3 {
  /// Get current account_v3 information.
  pub async fn fetch_info_summary(&self) -> Result<AccountInformationResponse> {
    let request = build_signed_query(BTreeMap::new(), self.recv_window)?;
    self
      .client
      .get_signed(API::SpotV3(SpotV3::Account), Some(request))
      .await
  }

  /// Get current ALL non-zero account_v3 balances.
  /// *Only free or locked > 0
  pub async fn list_balances(&self) -> Result<Vec<AssetBalanceResponse>> {
    self.fetch_info_summary().await.map(|r| {
      r.balances
        .into_iter()
        .filter(|asset| asset.free > 0.0 || asset.locked > 0.0)
        .collect()
    })
  }

  /// Get current FREE account_v3 balances.
  /// *Only free > 0
  pub async fn list_balances_free(&self) -> Result<Vec<AssetBalanceResponse>> {
    self.fetch_info_summary().await.map(|r| {
      r.balances
        .into_iter()
        .filter(|asset| asset.free > 0.0)
        .collect()
    })
  }

  /// Get current LOCKED account_v3 balances.
  /// *Only locked > 0
  pub async fn list_balances_locked(&self) -> Result<Vec<AssetBalanceResponse>> {
    self.fetch_info_summary().await.map(|r| {
      r.balances
        .into_iter()
        .filter(|asset| asset.locked > 0.0)
        .collect()
    })
  }

  /// Get Balance for a single coin
  /// e.g. BTC/ETH/USDT
  pub async fn fetch_balance_by_coin<S>(&self, coin: S) -> Result<AssetBalanceResponse>
  where
    S: Into<String>,
  {
    match self.fetch_info_summary().await {
      Ok(account) => {
        let cmp_coin = coin.into();
        for balance in account.balances {
          if balance.asset == cmp_coin {
            return Ok(balance);
          }
        }
        bail!("Asset not found");
      }
      Err(e) => Err(e),
    }
  }
}
