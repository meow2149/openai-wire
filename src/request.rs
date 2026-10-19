use crate::{
    ApiError, ApiResponse, Client, Error, Event, EventStream, Multipart, Result,
    client::insert_header,
    stream::{ByteStream, decode_events, parse_events},
};
use bytes::Bytes;
use futures_util::StreamExt;
use reqwest::{Method, StatusCode, header::HeaderMap};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{
    marker::PhantomData,
    time::{Duration, SystemTime},
};

#[derive(Clone)]
enum Body {
    Empty,
    Json(Value),
    Bytes(Bytes),
    Multipart(Multipart),
}

/// A lazy request. Customize headers, fields, types, timeout, or retries before sending.
///
/// Builder failures are retained and returned when the request is executed.
pub struct Request<T = Value> {
    pub(crate) client: Client,
    method: Method,
    pub(crate) path: String,
    body: Body,
    pub(crate) query: Vec<(String, String)>,
    headers: HeaderMap,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
    error: Option<Error>,
    marker: PhantomData<fn() -> T>,
}

impl<T> std::fmt::Debug for Request<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("method", &self.method)
            .finish_non_exhaustive()
    }
}

impl<T> Request<T> {
    pub(crate) fn new(client: Client, method: Method, path: String) -> Self {
        Self {
            client,
            method,
            path,
            body: Body::Empty,
            query: Vec::new(),
            headers: HeaderMap::new(),
            timeout: None,
            max_retries: None,
            error: None,
            marker: PhantomData,
        }
    }

    pub(crate) fn fail(mut self, error: Error) -> Self {
        self.error.get_or_insert(error);
        self
    }

    pub fn json(mut self, body: impl Serialize) -> Self {
        match serde_json::to_value(body) {
            Ok(value) => self.body = Body::Json(value),
            Err(e) => self = self.fail(e.into()),
        }
        self
    }

    /// Set a JSON field, including explicit null. Fields override earlier values.
    pub fn field(mut self, name: impl Into<String>, value: impl Serialize) -> Self {
        if matches!(self.body, Body::Empty) {
            self.body = Body::Json(json!({}));
        }
        match (&mut self.body, serde_json::to_value(value)) {
            (Body::Json(Value::Object(body)), Ok(value)) => {
                body.insert(name.into(), value);
            }
            (_, Err(e)) => return self.fail(e.into()),
            _ => {
                return self.fail(Error::InvalidRequest(
                    "field() requires a JSON object body".into(),
                ));
            }
        }
        self
    }

    pub fn body(mut self, bytes: impl Into<Bytes>) -> Self {
        self.body = Body::Bytes(bytes.into());
        self
    }
    pub fn multipart(mut self, form: Multipart) -> Self {
        self.body = Body::Multipart(form);
        self
    }

    /// Add or replace a text field in a multipart request.
    pub fn form_field(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        match &mut self.body {
            Body::Multipart(form) => form.set_text(name.into(), value.into()),
            Body::Empty => self.body = Body::Multipart(Multipart::new().text(name, value)),
            _ => {
                return self.fail(Error::InvalidRequest(
                    "form_field() requires multipart data".into(),
                ));
            }
        }
        self
    }

    /// Add URL-encoded parameters. Repeated names are preserved.
    pub fn query(mut self, query: impl Serialize) -> Self {
        match serde_urlencoded::to_string(query) {
            Ok(query) => self.query.extend(
                serde_urlencoded::from_str::<Vec<(String, String)>>(&query)
                    .expect("URL-encoded query is valid"),
            ),
            Err(_) => {
                self = self.fail(Error::InvalidRequest(
                    "query must serialize to key/value pairs".into(),
                ))
            }
        }
        self
    }

    pub fn header(mut self, name: &str, value: &str) -> Self {
        if let Err(e) = insert_header(&mut self.headers, name, value) {
            self = self.fail(e);
        }
        self
    }
    pub fn idempotency_key(self, key: &str) -> Self {
        self.header("Idempotency-Key", key)
    }
    pub fn timeout(mut self, timeout: Duration) -> Self {
        if timeout.is_zero() {
            return self.fail(Error::InvalidRequest("timeout must be positive".into()));
        }
        self.timeout = Some(timeout);
        self
    }
    pub fn max_retries(mut self, count: u32) -> Self {
        self.max_retries = Some(count);
        self
    }

    /// Decode into an application-defined response type.
    pub fn cast<U>(self) -> Request<U> {
        Request {
            client: self.client,
            method: self.method,
            path: self.path,
            body: self.body,
            query: self.query,
            headers: self.headers,
            timeout: self.timeout,
            max_retries: self.max_retries,
            error: self.error,
            marker: PhantomData,
        }
    }

    pub async fn send(self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(self.response().await?.data)
    }

    /// Decode JSON while retaining the response metadata.
    pub async fn response(self) -> Result<ApiResponse<T>>
    where
        T: DeserializeOwned,
    {
        let response = self.bytes().await?;
        let bytes = &response.data;
        // Empty successful responses are represented by JSON null (for example `()`).
        let data = serde_json::from_slice(if bytes.is_empty() { b"null" } else { bytes })?;
        Ok(response.map(|_| data))
    }

    /// Download the body as bytes, retaining the response metadata.
    pub async fn bytes(self) -> Result<ApiResponse<Bytes>> {
        let response = self.raw().await?;
        let status = response.status();
        let headers = response.headers().clone();
        Ok(ApiResponse {
            data: response.bytes().await?,
            status,
            headers,
        })
    }

    /// Download incrementally. Dropping the stream cancels the download.
    pub async fn byte_stream(self) -> Result<ApiResponse<ByteStream>> {
        let response = self.raw().await?;
        let status = response.status();
        let headers = response.headers().clone();
        Ok(ApiResponse {
            data: Box::pin(response.bytes_stream().map(|r| r.map_err(Error::from))),
            status,
            headers,
        })
    }

    /// Decode SSE JSON payloads into any event type. Streaming is enabled automatically
    /// for POST JSON and multipart requests. GET requests retain their query parameters.
    pub async fn stream<E: DeserializeOwned + Send + 'static>(self) -> Result<EventStream<E>> {
        let events = self.enable_stream().events().await?;
        Ok(decode_events(events))
    }

    fn enable_stream(mut self) -> Self {
        if self.method == Method::POST {
            match &mut self.body {
                Body::Multipart(form) => {
                    form.set_text("stream".into(), "true".into());
                }
                Body::Bytes(_) => {}
                _ => self = self.field("stream", true),
            }
        }
        self
    }

    /// Receive raw SSE events, including event names and IDs. This leaves the
    /// request body unchanged; set the endpoint's streaming fields explicitly.
    pub async fn events(self) -> Result<EventStream<Event>> {
        let response = self.header("Accept", "text/event-stream").raw().await?;
        if !response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| {
                v.split(';')
                    .next()
                    .is_some_and(|v| v.trim().eq_ignore_ascii_case("text/event-stream"))
            })
        {
            return Err(Error::Stream(
                "expected text/event-stream content type".into(),
            ));
        }
        Ok(parse_events(response))
    }

    /// Send and check the HTTP status without buffering successful response bodies.
    /// Redirects count as failures with the default client.
    pub async fn raw(mut self) -> Result<reqwest::Response> {
        if let Some(error) = self.error.take() {
            return Err(error);
        }
        let mut url = self.client.url(&self.path)?;
        url.query_pairs_mut()
            .extend_pairs(&self.client.inner.query)
            .extend_pairs(&self.query);
        let mut headers = self.client.inner.headers.clone();
        headers.extend(self.headers.clone());
        // A stable key makes replayed mutations recognizable by compatible services.
        if self.method != Method::GET
            && self.method != Method::HEAD
            && !headers.contains_key("idempotency-key")
        {
            insert_header(
                &mut headers,
                "idempotency-key",
                &format!("openai-wire-{:032x}", rand::random::<u128>()),
            )?;
        }
        let max_retries = self.max_retries.unwrap_or(self.client.inner.max_retries);
        let timeout = self.timeout.unwrap_or(self.client.inner.timeout);
        let mut attempt = 0;
        loop {
            let mut request = self
                .client
                .inner
                .http
                .request(self.method.clone(), url.clone())
                .headers(headers.clone())
                .timeout(timeout);
            request = match &self.body {
                Body::Empty => request,
                Body::Json(value) => request.json(value),
                Body::Bytes(bytes) => request.body(bytes.clone()),
                Body::Multipart(form) => request.multipart(form.form().await?),
            };
            let result = request.send().await;
            match result {
                Ok(response) if response.status().is_success() => return Ok(response),
                Ok(response) => {
                    let status = response.status();
                    let response_headers = response.headers().clone();
                    let delay = retry_after(&response_headers);
                    let within_budget =
                        delay.is_none_or(|d| d <= self.client.inner.max_retry_delay);
                    if attempt < max_retries
                        && within_budget
                        && should_retry(status, &response_headers)
                    {
                        drop(response);
                        tokio::time::sleep(delay.unwrap_or_else(|| backoff(attempt))).await;
                        attempt += 1;
                        continue;
                    }
                    let body = response.bytes().await?;
                    return Err(Error::Api(Box::new(ApiError::from_body(
                        Some(status),
                        response_headers,
                        &body,
                    ))));
                }
                Err(error)
                    if attempt < max_retries
                        && (error.is_timeout() || error.is_connect() || error.is_request()) =>
                {
                    tokio::time::sleep(backoff(attempt)).await;
                    attempt += 1;
                }
                Err(error) => return Err(error.into()),
            }
        }
    }

    pub(crate) fn duplicate(&self) -> Self {
        Self {
            client: self.client.clone(),
            method: self.method.clone(),
            path: self.path.clone(),
            body: self.body.clone(),
            query: self.query.clone(),
            headers: self.headers.clone(),
            timeout: self.timeout,
            max_retries: self.max_retries,
            error: None,
            marker: PhantomData,
        }
    }

    pub(crate) fn take_error(&mut self) -> Option<Error> {
        self.error.take()
    }
}

fn should_retry(status: StatusCode, headers: &HeaderMap) -> bool {
    match headers.get("x-should-retry").and_then(|v| v.to_str().ok()) {
        Some("true") => true,
        Some("false") => false,
        _ => matches!(status.as_u16(), 408 | 409 | 429) || status.is_server_error(),
    }
}

fn retry_after(headers: &HeaderMap) -> Option<Duration> {
    for (name, divisor) in [("retry-after-ms", 1000.0), ("retry-after", 1.0)] {
        if let Some(value) = headers.get(name).and_then(|v| v.to_str().ok()) {
            if let Ok(value) = value.parse::<f64>() {
                if !value.is_finite() {
                    return Some(Duration::MAX);
                }
                if value >= 0.0 {
                    return Some(
                        Duration::try_from_secs_f64(value / divisor).unwrap_or(Duration::MAX),
                    );
                }
            }
            if name == "retry-after"
                && let Ok(time) = httpdate::parse_http_date(value)
            {
                return Some(
                    time.duration_since(SystemTime::now())
                        .unwrap_or(Duration::ZERO),
                );
            }
        }
    }
    None
}

fn backoff(attempt: u32) -> Duration {
    let seconds = (0.5 * 2.0_f64.powi(attempt.min(5) as i32)).min(8.0);
    Duration::from_secs_f64(seconds * (0.75 + 0.25 * rand::random::<f64>()))
}
