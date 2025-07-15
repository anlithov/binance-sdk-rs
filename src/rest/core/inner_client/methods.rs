use crate::errors::BinanceContentError;
use crate::rest::core::inner_client::rate_limit_manage::extract_and_update_rate_limiter_counts;
use crate::rest::core::inner_client::InnerClient;
use crate::rest::endpoints::API;
use crate::result::AnyhowResult;
use anyhow::bail;
use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

impl InnerClient {
  pub async fn get_signed<T: DeserializeOwned>(
    &self,
    endpoint: API,
    query: Option<String>,
  ) -> AnyhowResult<T> {
    self
      .acquire_ip_limit_permit(&endpoint, query.clone())
      .await?;

    let url = self.build_signed_url(&endpoint, query);
    let http_client = &self.http_client;

    let response = http_client
      .get(url.as_str())
      .headers(self.build_headers(true)?)
      .send()
      .await?;

    self.handler(response, &endpoint).await
  }

  pub async fn post_signed<T: DeserializeOwned>(
    &self,
    endpoint: API,
    query: String,
  ) -> AnyhowResult<T> {
    self
      .acquire_ip_and_order_limits_permit(&endpoint, Some(query.clone()))
      .await?;

    let url = self.build_signed_url(&endpoint, Some(query));

    let http_client = &self.http_client;
    let response = http_client
      .post(url.as_str())
      .headers(self.build_headers(true)?)
      .body("".to_string())
      .send()
      .await?;

    self.handler(response, &endpoint).await
  }

  pub async fn delete_signed<T: DeserializeOwned>(
    &self,
    endpoint: API,
    query: Option<String>,
  ) -> AnyhowResult<T> {
    self
      .acquire_ip_limit_permit(&endpoint, query.clone())
      .await?;

    let url = self.build_signed_url(&endpoint, query);
    let http_client = &self.http_client;
    let response = http_client
      .delete(url.as_str())
      .headers(self.build_headers(true)?)
      .send()
      .await?;

    self.handler(response, &endpoint).await
  }

  pub async fn get<T: DeserializeOwned>(
    &self,
    endpoint: API,
    query: Option<String>,
  ) -> AnyhowResult<T> {
    self
      .acquire_ip_limit_permit(&endpoint, query.clone())
      .await?;

    let mut url = self.build_url(&endpoint);
    if let Some(request) = query {
      if !request.is_empty() {
        url.push_str(format!("?{}", request).as_str());
      }
    }

    let http_client = &self.http_client;

    let response = http_client.get(url.as_str()).send().await?;

    self.handler(response, &endpoint).await
  }

  pub async fn post<T: DeserializeOwned>(&self, endpoint: API) -> AnyhowResult<T> {
    let url = self.build_url(&endpoint);

    let http_client = &self.http_client;
    let response = http_client
      .post(url.as_str())
      .headers(self.build_headers(false)?)
      .send()
      .await?;

    self.handler(response, &endpoint).await
  }

  pub async fn put<T: DeserializeOwned>(&self, endpoint: API, listen_key: &str) -> AnyhowResult<T> {
    let url = self.build_url(&endpoint);
    let data: String = format!("listenKey={}", listen_key);

    let http_client = &self.http_client;
    let response = http_client
      .put(url.as_str())
      .headers(self.build_headers(false)?)
      .body(data)
      .send()
      .await?;

    self.handler(response, &endpoint).await
  }

  pub async fn delete<T: DeserializeOwned>(
    &self,
    endpoint: API,
    listen_key: &str,
  ) -> AnyhowResult<T> {
    let url = self.build_url(&endpoint);
    let data: String = format!("listenKey={}", listen_key);

    let http_client = &self.http_client;
    let response = http_client
      .delete(url.as_str())
      .headers(self.build_headers(false)?)
      .body(data)
      .send()
      .await?;

    self.handler(response, &endpoint).await
  }

  fn build_signed_url(&self, endpoint: &API, query: Option<String>) -> String {
    let mut signed_key =
      HmacSha256::new_from_slice(self.secret_key.as_ref().unwrap().as_bytes()).unwrap();
    let signed_query;
    if let Some(params) = query {
      signed_key.update(params.as_bytes());
      let signature = hex_encode(signed_key.finalize().into_bytes());
      signed_query = format!("{}&signature={}", params, signature);
    } else {
      let signature = hex_encode(signed_key.finalize().into_bytes());
      signed_query = format!("&signature={}", signature);
    }

    format!("{}{}?{}", self.server_host, endpoint.as_ref(), signed_query)
  }

  fn build_url(&self, endpoint: &API) -> String {
    format!("{}{}", self.server_host, endpoint.as_ref())
  }

  fn build_headers(&self, content_type: bool) -> anyhow::Result<HeaderMap> {
    let mut custom_headers = HeaderMap::new();

    custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-sdk-rs"));
    if content_type {
      custom_headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
      );
    }

    if let Some(api_key) = &self.api_key {
      custom_headers.insert(
        HeaderName::from_static("x-mbx-apikey"),
        HeaderValue::from_str(api_key.as_str())?,
      );
    }

    Ok(custom_headers)
  }

  async fn handler<T: DeserializeOwned>(
    &self,
    response: Response,
    endpoint: &API,
  ) -> AnyhowResult<T> {
    // Extract headers first to update rate limit usage
    let headers = response.headers().clone();

    // Process headers to update rate limit usage
    if let Err(e) = extract_and_update_rate_limiter_counts(
      &self.ip_rate_limit_manager,
      &self.unfilled_order_rate_limit_manager,
      &headers,
      endpoint,
    )
    .await
    {
      eprintln!("Error updating used weights from headers: {}", e);
    }

    match response.status() {
      StatusCode::OK => {
        let response = response.json::<T>().await?;
        Ok(response)
      }
      StatusCode::INTERNAL_SERVER_ERROR => {
        bail!("Internal Server Error");
      }
      StatusCode::SERVICE_UNAVAILABLE => {
        bail!("Service Unavailable");
      }
      StatusCode::UNAUTHORIZED => {
        bail!("Unauthorized");
      }
      StatusCode::BAD_REQUEST => {
        let error: BinanceContentError = response.json().await?;

        bail!("Binance error: code={}, msg={}", error.code, error.msg)
      }
      StatusCode::TOO_MANY_REQUESTS => {
        bail!("Rate limit exceeded (429). Please try again later.")
      }
      s => {
        let text = response.text().await?;
        bail!("Received response. Status:{:?}. Text: {:?}", s, text);
      }
    }
  }
}
