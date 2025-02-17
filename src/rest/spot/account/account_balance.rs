use super::model::{AccountInformationResponse, AssetBalanceResponse};
use super::Account;
use crate::rest::endpoints::{Spot, API};
use crate::util::build_signed_request;
use anyhow::bail;
use anyhow::Result;
use std::collections::BTreeMap;

// Balances
impl Account {
  /// Get current account information.
  pub async fn fetch_info_summary(&self) -> Result<AccountInformationResponse> {
    let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
    self
      .client
      .get_signed(API::Spot(Spot::Account), Some(request))
      .await
  }

  /// Get current ALL non-zero account balances.
  /// *Only free or locked > 0
  pub async fn fetch_balances(&self) -> Result<Vec<AssetBalanceResponse>> {
    self.fetch_info_summary().await.map(|r| {
      r.balances
        .into_iter()
        .filter(|asset| asset.free > 0.0 || asset.locked > 0.0)
        .collect()
    })
  }

  /// Get current FREE account balances.
  /// *Only free > 0
  pub async fn fetch_balances_free(&self) -> Result<Vec<AssetBalanceResponse>> {
    self.fetch_info_summary().await.map(|r| {
      r.balances
        .into_iter()
        .filter(|asset| asset.free > 0.0)
        .collect()
    })
  }

  /// Get current LOCKED account balances.
  /// *Only locked > 0
  pub async fn fetch_balances_locked(&self) -> Result<Vec<AssetBalanceResponse>> {
    self.fetch_info_summary().await.map(|r| {
      r.balances
        .into_iter()
        .filter(|asset| asset.locked > 0.0)
        .collect()
    })
  }

  /// Get Balance for a single Asset
  pub async fn fetch_balance_single<S>(&self, asset: S) -> Result<AssetBalanceResponse>
  where
    S: Into<String>,
  {
    match self.fetch_info_summary().await {
      Ok(account) => {
        let cmp_asset = asset.into();
        for balance in account.balances {
          if balance.asset == cmp_asset {
            return Ok(balance);
          }
        }
        bail!("Asset not found");
      }
      Err(e) => Err(e),
    }
  }
}
