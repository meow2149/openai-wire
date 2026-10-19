use reqwest::{StatusCode, header::HeaderMap};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The result type used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// A failed HTTP request, including structured API details and response headers.
///
/// Bodies are available explicitly and are excluded from `Debug` and `Display`.
#[derive(Clone, Deserialize, Serialize)]
pub struct ApiError {
    #[serde(skip)]
    pub status: Option<StatusCode>,
    #[serde(default)]
    pub message: String,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub code: Option<Value>,
    pub param: Option<Value>,
    #[serde(skip)]
    pub request_id: Option<String>,
    #[serde(skip)]
    pub headers: HeaderMap,
    #[serde(skip)]
    pub body: Vec<u8>,
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiError")
            .field("status", &self.status)
            .field("request_id", &self.request_id)
            .finish_non_exhaustive()
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API request failed")?;
        if let Some(status) = self.status {
            write!(f, " ({status})")?;
        }
        if let Some(id) = &self.request_id {
            write!(f, ", request ID: {id}")?;
        }
        Ok(())
    }
}

impl std::error::Error for ApiError {}

impl ApiError {
    pub(crate) fn from_body(status: Option<StatusCode>, headers: HeaderMap, body: &[u8]) -> Self {
        let value: Value = serde_json::from_slice(body).unwrap_or(Value::Null);
        let details = value
            .get("error")
            .filter(|v| v.is_object())
            .unwrap_or(&value);
        Self {
            status,
            message: details["message"]
                .as_str()
                .unwrap_or("Unstructured API error")
                .to_owned(),
            kind: details["type"].as_str().map(str::to_owned),
            code: details.get("code").filter(|v| !v.is_null()).cloned(),
            param: details.get("param").filter(|v| !v.is_null()).cloned(),
            request_id: headers
                .get("x-request-id")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned),
            headers,
            body: body.to_vec(),
        }
    }
}

/// Transport, decoding, configuration, and API failures.
#[derive(thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Api(Box<ApiError>),
    #[error("HTTP transport failed")]
    Transport(#[source] reqwest::Error),
    #[error("JSON encoding or decoding failed")]
    Json(#[from] serde_json::Error),
    #[error("Invalid configuration: {0}")]
    Config(String),
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error("File I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("Streaming protocol error: {0}")]
    Stream(String),
    #[cfg(feature = "webhooks")]
    #[error("Webhook verification failed: {0}")]
    Webhook(String),
    #[cfg(feature = "websocket")]
    #[error("WebSocket transport failed")]
    WebSocket(#[source] Box<tokio_tungstenite::tungstenite::Error>),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // serde and transport sources can contain rejected values or response bodies.
        std::fmt::Display::fmt(self, f)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Transport(error.without_url())
    }
}

impl Error {
    /// Access status, request ID, API message, and raw body for an API failure.
    pub fn api_error(&self) -> Option<&ApiError> {
        match self {
            Self::Api(error) => Some(error),
            _ => None,
        }
    }
    pub fn status(&self) -> Option<StatusCode> {
        self.api_error().and_then(|e| e.status)
    }
    pub fn request_id(&self) -> Option<&str> {
        self.api_error().and_then(|e| e.request_id.as_deref())
    }
}
