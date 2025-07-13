use crate::rest::account_general::v1::responses::AccountApiRestrictions;
use crate::rest::core::inner_client::InnerClient;
use crate::rest::endpoints::{AccountGeneral, API};
use crate::util::build_signed_query;
use anyhow::Result;
use std::collections::BTreeMap;

pub mod responses;

#[derive(Clone)]
pub struct AccountGeneralManagerV1 {
  pub client: InnerClient,
  pub recv_window: u64,
}

impl AccountGeneralManagerV1 {
  pub async fn fetch_api_restrictions(&self) -> Result<AccountApiRestrictions> {
    let request = build_signed_query(BTreeMap::new(), self.recv_window)?;

    self
      .client
      .get_signed(
        API::AccountGeneral(AccountGeneral::ApiRestrictions),
        Some(request),
      )
      .await
  }
}
