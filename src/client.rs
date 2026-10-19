use crate::{Error, Request, Result, endpoints::Endpoint};
use reqwest::{
    Method, Url,
    header::{AUTHORIZATION, HeaderMap, HeaderName, HeaderValue, USER_AGENT},
};
use serde_json::Value;
use std::{sync::Arc, time::Duration};

/// A cheap-to-clone client sharing a connection pool and immutable configuration.
#[derive(Clone)]
pub struct Client {
    pub(crate) inner: Arc<Inner>,
}

pub(crate) struct Inner {
    pub http: reqwest::Client,
    pub base_url: Url,
    pub headers: HeaderMap,
    pub timeout: Duration,
    pub max_retries: u32,
    pub max_retry_delay: Duration,
    pub query: Vec<(String, String)>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field(
                "origin",
                &self.inner.base_url.origin().ascii_serialization(),
            )
            .field("timeout", &self.inner.timeout)
            .field("max_retries", &self.inner.max_retries)
            .finish()
    }
}

impl Client {
    /// Configure a client explicitly. Environment variables are read only by `from_env`.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Create a client using `OPENAI_API_KEY`, `OPENAI_BASE_URL`, `OPENAI_ORG_ID`,
    /// and `OPENAI_PROJECT_ID`. Missing credentials are reported immediately.
    pub fn from_env() -> Result<Self> {
        let key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| Error::Config("OPENAI_API_KEY is not set".into()))?;
        let mut builder = Self::builder().api_key(key);
        if let Ok(v) = std::env::var("OPENAI_BASE_URL") {
            builder = builder.base_url(v);
        }
        if let Ok(v) = std::env::var("OPENAI_ORG_ID") {
            builder = builder.organization(v);
        }
        if let Ok(v) = std::env::var("OPENAI_PROJECT_ID") {
            builder = builder.project(v);
        }
        builder.build()
    }

    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }

    /// Start a request to a relative API path. Absolute URLs and traversal are rejected.
    pub fn request<T>(&self, method: Method, path: impl Into<String>) -> Request<T> {
        Request::new(self.clone(), method, path.into())
    }

    pub fn endpoint(&self, endpoint: Endpoint) -> Request<Value> {
        self.request(endpoint.method, endpoint.path)
    }

    pub(crate) fn url(&self, path: &str) -> Result<Url> {
        let relative = path.strip_prefix('/').unwrap_or(path);
        if relative.starts_with('/')
            || relative.contains(['?', '#', '\\'])
            || relative.contains("://")
        {
            return Err(Error::InvalidRequest(
                "use a relative API path and query() for parameters".into(),
            ));
        }
        for segment in relative.split('/') {
            let decoded = decode_segment(segment)?;
            if decoded == "." || decoded == ".." {
                return Err(Error::InvalidRequest(
                    "path traversal is not allowed".into(),
                ));
            }
        }
        let mut url = self.inner.base_url.clone();
        url.set_path(&format!("{}{}", url.path(), relative));
        Ok(url)
    }
}

/// Client settings. Credentials and header values are excluded from debugging.
pub struct ClientBuilder {
    key: Option<String>,
    base_url: String,
    headers: Vec<(String, String)>,
    timeout: Duration,
    max_retries: u32,
    max_retry_delay: Duration,
    http: Option<reqwest::Client>,
    query: Vec<(String, String)>,
    anonymous: bool,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            key: None,
            base_url: "https://api.openai.com/v1/".into(),
            headers: Vec::new(),
            timeout: Duration::from_secs(600),
            max_retries: 2,
            max_retry_delay: Duration::from_secs(120),
            http: None,
            query: Vec::new(),
            anonymous: false,
        }
    }
}

impl ClientBuilder {
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }
    pub fn organization(self, id: impl Into<String>) -> Self {
        self.header("OpenAI-Organization", id)
    }
    pub fn project(self, id: impl Into<String>) -> Self {
        self.header("OpenAI-Project", id)
    }
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    /// Number of additional attempts. Use zero to disable automatic retries.
    pub fn max_retries(mut self, count: u32) -> Self {
        self.max_retries = count;
        self
    }
    /// Server-directed delays above this threshold stop retrying instead of retrying early.
    pub fn max_retry_delay(mut self, delay: Duration) -> Self {
        self.max_retry_delay = delay;
        self
    }
    /// Use a custom connection pool. Configure its redirect policy appropriately.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http = Some(client);
        self
    }
    /// Omit bearer authentication for a local or otherwise anonymous compatible API.
    pub fn anonymous(mut self) -> Self {
        self.anonymous = true;
        self
    }
    /// Configure Azure's current `/openai/v1/` API with API-key authentication.
    pub fn azure(mut self, endpoint: impl Into<String>, key: impl Into<String>) -> Self {
        self.base_url = format!("{}/openai/v1/", endpoint.into().trim_end_matches('/'));
        self.anonymous = true;
        self.header("api-key", key)
    }
    /// Add a default query parameter (for example an API version for a compatible service).
    pub fn query(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.push((name.into(), value.into()));
        self
    }

    pub fn build(self) -> Result<Client> {
        let mut base_url =
            Url::parse(&self.base_url).map_err(|_| Error::Config("invalid base URL".into()))?;
        if !matches!(base_url.scheme(), "http" | "https")
            || base_url.host_str().is_none()
            || !base_url.username().is_empty()
            || base_url.password().is_some()
            || base_url.query().is_some()
            || base_url.fragment().is_some()
        {
            return Err(Error::Config(
                "base URL must be HTTP(S), without credentials, query, or fragment".into(),
            ));
        }
        if !base_url.path().ends_with('/') {
            base_url.set_path(&format!("{}/", base_url.path()));
        }
        if self.timeout.is_zero() {
            return Err(Error::Config("timeout must be positive".into()));
        }
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(concat!("openai-wire/", env!("CARGO_PKG_VERSION"))),
        );
        if let Some(key) = self.key {
            if key.trim().is_empty() {
                return Err(Error::Config("API key is empty".into()));
            }
            headers.insert(AUTHORIZATION, sensitive_value(&format!("Bearer {key}"))?);
        } else if !self.anonymous {
            return Err(Error::Config(
                "provide an API key or opt into anonymous()".into(),
            ));
        }
        for (name, value) in self.headers {
            insert_header(&mut headers, &name, &value)?;
        }
        let http = match self.http {
            Some(http) => http,
            None => reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .connect_timeout(Duration::from_secs(10))
                .build()?,
        };
        Ok(Client {
            inner: Arc::new(Inner {
                http,
                base_url,
                headers,
                timeout: self.timeout,
                max_retries: self.max_retries,
                max_retry_delay: self.max_retry_delay,
                query: self.query,
            }),
        })
    }
}

pub(crate) fn insert_header(headers: &mut HeaderMap, name: &str, value: &str) -> Result<()> {
    let name = HeaderName::from_bytes(name.as_bytes())
        .map_err(|_| Error::InvalidRequest("invalid header name".into()))?;
    let mut value = HeaderValue::from_str(value)
        .map_err(|_| Error::InvalidRequest("invalid header value".into()))?;
    // All custom values can carry application secrets; never expose them through Debug.
    value.set_sensitive(true);
    headers.insert(name, value);
    Ok(())
}

fn sensitive_value(value: &str) -> Result<HeaderValue> {
    let mut value =
        HeaderValue::from_str(value).map_err(|_| Error::Config("invalid API key".into()))?;
    value.set_sensitive(true);
    Ok(value)
}

/// Encode an opaque identifier as one URL path segment.
pub fn encode_segment(value: &str) -> Result<String> {
    if value.is_empty() || matches!(value, "." | "..") {
        return Err(Error::InvalidRequest(
            "identifier must be nonempty and cannot be a dot segment".into(),
        ));
    }
    let mut encoded = String::new();
    use std::fmt::Write;
    for b in value.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            encoded.push(b as char);
        } else {
            write!(encoded, "%{b:02X}").expect("writing to String cannot fail");
        }
    }
    Ok(encoded)
}

fn decode_segment(value: &str) -> Result<String> {
    let mut bytes = Vec::new();
    let mut iter = value.bytes();
    while let Some(b) = iter.next() {
        if b == b'%' {
            let a = iter.next().and_then(|v| (v as char).to_digit(16));
            let b = iter.next().and_then(|v| (v as char).to_digit(16));
            let (Some(a), Some(b)) = (a, b) else {
                return Err(Error::InvalidRequest("invalid path escape".into()));
            };
            bytes.push((a * 16 + b) as u8);
        } else {
            bytes.push(b);
        }
    }
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}
