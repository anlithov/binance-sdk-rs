use crate::errors::BinanceContentError;
use crate::rest::endpoints::API;
use anyhow::bail;
use anyhow::Result;
use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct InnerClient {
  api_key: String,
  secret_key: String,
  server_host: String,
  http_client: Client,
}

impl InnerClient {
  pub fn new(api_key: Option<String>, secret_key: Option<String>, server_host: String) -> Self {
    InnerClient {
      api_key: api_key.unwrap_or_default(),
      secret_key: secret_key.unwrap_or_default(),
      server_host,
      http_client: Client::builder().pool_idle_timeout(None).build().unwrap(),
    }
  }
}

impl InnerClient {
  pub async fn get_signed<T: DeserializeOwned>(
    &self,
    endpoint: API,
    query: Option<String>,
  ) -> Result<T> {
    let url = self.build_signed_url(endpoint, query);
    let http_client = &self.http_client;

    let response = http_client
      .get(url.as_str())
      .headers(self.build_headers(true)?)
      .send()
      .await?;

    self.handler(response).await
  }

  pub async fn post_signed<T: DeserializeOwned>(&self, endpoint: API, query: String) -> Result<T> {
    let url = self.build_signed_url(endpoint, Some(query));

    let http_client = &self.http_client;
    let response = http_client
      .post(url.as_str())
      .headers(self.build_headers(true)?)
      .body("".to_string())
      .send()
      .await?;

    self.handler(response).await
  }

  pub async fn delete_signed<T: DeserializeOwned>(
    &self,
    endpoint: API,
    query: Option<String>,
  ) -> Result<T> {
    let url = self.build_signed_url(endpoint, query);
    let http_client = &self.http_client;
    let response = http_client
      .delete(url.as_str())
      .headers(self.build_headers(true)?)
      .send()
      .await?;

    self.handler(response).await
  }

  pub async fn get<T: DeserializeOwned>(&self, endpoint: API, query: Option<String>) -> Result<T> {
    let mut url = self.build_url(endpoint);
    if let Some(request) = query {
      if !request.is_empty() {
        url.push_str(format!("?{}", request).as_str());
      }
    }

    let http_client = &self.http_client;

    let response = http_client.get(url.as_str()).send().await?;

    self.handler(response).await
  }

  pub async fn post<T: DeserializeOwned>(&self, endpoint: API) -> Result<T> {
    let url = self.build_url(endpoint);

    let http_client = &self.http_client;
    let response = http_client
      .post(url.as_str())
      .headers(self.build_headers(false)?)
      .send()
      .await?;

    self.handler(response).await
  }

  pub async fn put<T: DeserializeOwned>(&self, endpoint: API, listen_key: &str) -> Result<T> {
    let url = self.build_url(endpoint);
    let data: String = format!("listenKey={}", listen_key);

    let http_client = &self.http_client;
    let response = http_client
      .put(url.as_str())
      .headers(self.build_headers(false)?)
      .body(data)
      .send()
      .await?;

    self.handler(response).await
  }

  pub async fn delete<T: DeserializeOwned>(&self, endpoint: API, listen_key: &str) -> Result<T> {
    let url = self.build_url(endpoint);
    let data: String = format!("listenKey={}", listen_key);

    let http_client = &self.http_client;
    let response = http_client
      .delete(url.as_str())
      .headers(self.build_headers(false)?)
      .body(data)
      .send()
      .await?;

    self.handler(response).await
  }

  fn build_signed_url(&self, endpoint: API, query: Option<String>) -> String {
    let mut signed_key = HmacSha256::new_from_slice(self.secret_key.as_bytes()).unwrap();
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

  fn build_url(&self, endpoint: API) -> String {
    format!("{}{}", self.server_host, endpoint.as_ref())
  }

  fn build_headers(&self, content_type: bool) -> Result<HeaderMap> {
    let mut custom_headers = HeaderMap::new();

    custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-sdk-rs"));
    if content_type {
      custom_headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
      );
    }
    custom_headers.insert(
      HeaderName::from_static("x-mbx-apikey"),
      HeaderValue::from_str(self.api_key.as_str())?,
    );

    Ok(custom_headers)
  }

  async fn handler<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
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
      s => {
        bail!("Received response: {:?}", s);
      }
    }
  }
}
