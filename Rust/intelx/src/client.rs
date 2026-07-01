//! The base HTTP client shared by every endpoint module.

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::{IntelXError, Result, api_error_from_status};

const DEFAULT_BASE_URL: &str = "https://2.intelx.io";
const DEFAULT_USER_AGENT: &str = concat!("IX-Rust/", env!("CARGO_PKG_VERSION"));
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_RATE_LIMIT: Duration = Duration::from_secs(1);

/// An async client for the general Intelligence X API (default base URL `https://2.intelx.io`).
///
/// Construct one with [`IntelXClient::new`] for the common case, or [`IntelXClient::builder`]
/// to configure a custom base URL, proxy, timeout, or rate limit.
#[derive(Clone, Debug)]
pub struct IntelXClient {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: url::Url,
    pub(crate) api_key: String,
    pub(crate) rate_limit: Duration,
}

/// Builder for [`IntelXClient`].
pub struct IntelXClientBuilder {
    api_key: Option<String>,
    base_url: String,
    user_agent: String,
    proxy: Option<String>,
    danger_accept_invalid_certs: bool,
    timeout: Duration,
    rate_limit: Duration,
}

impl Default for IntelXClientBuilder {
    fn default() -> Self {
        Self {
            api_key: None,
            base_url: DEFAULT_BASE_URL.to_string(),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            proxy: None,
            danger_accept_invalid_certs: false,
            timeout: DEFAULT_TIMEOUT,
            rate_limit: DEFAULT_RATE_LIMIT,
        }
    }
}

impl IntelXClientBuilder {
    /// Sets the API key sent as the `X-Key` header on every request. Required.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Overrides the base URL. Defaults to `https://2.intelx.io`.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Overrides the `User-Agent` header. Defaults to `IX-Rust/<crate version>`.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Routes all requests through the given proxy URL.
    pub fn proxy(mut self, proxy_url: impl Into<String>) -> Self {
        self.proxy = Some(proxy_url.into());
        self
    }

    /// Disables TLS certificate verification. Mirrors the Python SDK's `verify=False`.
    ///
    /// This is dangerous: only use it against a known/trusted endpoint (e.g. for local
    /// testing through an intercepting proxy).
    pub fn danger_accept_invalid_certs(mut self, accept: bool) -> Self {
        self.danger_accept_invalid_certs = accept;
        self
    }

    /// Sets the per-request HTTP timeout. Defaults to 30 seconds.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the delay applied before each request to stay within the API's rate limit.
    /// Defaults to 1 second, matching the Python SDK's `API_RATE_LIMIT`.
    pub fn rate_limit(mut self, rate_limit: Duration) -> Self {
        self.rate_limit = rate_limit;
        self
    }

    /// Builds the [`IntelXClient`].
    ///
    /// Returns [`IntelXError::MissingApiKey`] if no API key was set, or
    /// [`IntelXError::InvalidUrl`] if the configured base URL doesn't parse.
    pub fn build(self) -> Result<IntelXClient> {
        let api_key = self.api_key.ok_or(IntelXError::MissingApiKey)?;
        let base_url = url::Url::parse(&self.base_url)?;

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Key",
            HeaderValue::from_str(&api_key).map_err(|_| IntelXError::Api {
                status: 0,
                message: "invalid API key header value".into(),
            })?,
        );

        let mut builder = reqwest::Client::builder()
            .user_agent(self.user_agent)
            .default_headers(headers)
            .timeout(self.timeout)
            .danger_accept_invalid_certs(self.danger_accept_invalid_certs);

        if let Some(proxy) = self.proxy {
            builder = builder.proxy(reqwest::Proxy::all(proxy)?);
        }

        let http = builder.build()?;

        Ok(IntelXClient {
            http,
            base_url,
            api_key,
            rate_limit: self.rate_limit,
        })
    }
}

impl IntelXClient {
    /// Creates a builder for configuring a client.
    pub fn builder() -> IntelXClientBuilder {
        IntelXClientBuilder::default()
    }

    /// Creates a client for `api_key` using all other defaults, equivalent to Python's
    /// `intelx(api_key)`.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }

    /// The configured base URL.
    pub fn base_url(&self) -> &url::Url {
        &self.base_url
    }

    /// The configured API key.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Returns the current user's API capabilities (allowed buckets, per-endpoint credits,
    /// concurrent search limits). Mirrors the Python SDK's `GET_CAPABILITIES()`.
    pub async fn get_capabilities(&self) -> Result<crate::models::Capabilities> {
        self.rate_limit_sleep().await;
        let no_params: [(&str, &str); 0] = [];
        self.get("/authenticate/info", &no_params).await
    }

    fn resolve_url(&self, path: &str) -> Result<url::Url> {
        if path.starts_with("http://") || path.starts_with("https://") {
            Ok(url::Url::parse(path)?)
        } else {
            Ok(self.base_url.join(path.trim_start_matches('/'))?)
        }
    }

    /// Sleeps for the configured rate-limit duration before issuing a request, mirroring the
    /// Python SDK's `time.sleep(self.API_RATE_LIMIT)`.
    pub(crate) async fn rate_limit_sleep(&self) {
        tokio::time::sleep(self.rate_limit).await;
    }

    pub(crate) async fn get_response(
        &self,
        path: &str,
        query: &(impl Serialize + ?Sized),
    ) -> Result<reqwest::Response> {
        let url = self.resolve_url(path)?;
        let response = self.http.get(url).query(query).send().await?;
        Ok(response)
    }

    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &(impl Serialize + ?Sized),
    ) -> Result<T> {
        let response = self.get_response(path, query).await?;
        Self::deserialize_or_error(response).await
    }

    pub(crate) async fn post_json<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.resolve_url(path)?;
        let response = self.http.post(url).json(body).send().await?;
        Self::deserialize_or_error(response).await
    }

    async fn deserialize_or_error<T: DeserializeOwned>(response: reqwest::Response) -> Result<T> {
        let status = response.status();
        if !status.is_success() {
            return Err(api_error_from_status(status));
        }
        let bytes = response.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_fails_without_api_key() {
        let err = IntelXClient::builder().build().unwrap_err();
        assert!(matches!(err, IntelXError::MissingApiKey));
    }

    #[test]
    fn builder_applies_python_compatible_defaults() {
        let client = IntelXClient::new("test-key").unwrap();
        assert_eq!(client.base_url().as_str(), "https://2.intelx.io/");
        assert_eq!(client.api_key(), "test-key");
        assert_eq!(client.rate_limit, DEFAULT_RATE_LIMIT);
    }

    #[test]
    fn builder_overrides_are_applied() {
        let client = IntelXClient::builder()
            .api_key("k")
            .base_url("https://example.test")
            .rate_limit(Duration::from_millis(0))
            .build()
            .unwrap();
        assert_eq!(client.base_url().as_str(), "https://example.test/");
        assert_eq!(client.rate_limit, Duration::from_millis(0));
    }

    #[test]
    fn resolve_url_joins_relative_paths_against_base() {
        let client = IntelXClient::new("k").unwrap();
        let url = client.resolve_url("/intelligent/search").unwrap();
        assert_eq!(url.as_str(), "https://2.intelx.io/intelligent/search");
    }

    #[test]
    fn resolve_url_leaves_absolute_urls_untouched() {
        let client = IntelXClient::new("k").unwrap();
        let url = client.resolve_url("https://other.example/x").unwrap();
        assert_eq!(url.as_str(), "https://other.example/x");
    }
}
