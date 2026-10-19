//! Standard Webhooks verification over the original request bytes.
use crate::{Error, Result};
use base64::{Engine, engine::general_purpose::STANDARD};
use hmac::{Hmac, KeyInit, Mac};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::Sha256;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// An authenticated webhook payload with forward-compatible event types.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    pub object: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub created_at: u64,
    pub data: Value,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A reusable verifier with a five-minute replay window by default.
pub struct WebhookVerifier {
    key: Vec<u8>,
    tolerance: Duration,
}
impl std::fmt::Debug for WebhookVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebhookVerifier")
            .field("tolerance", &self.tolerance)
            .finish_non_exhaustive()
    }
}
impl Drop for WebhookVerifier {
    fn drop(&mut self) {
        self.key.fill(0);
    }
}

impl WebhookVerifier {
    /// `whsec_` secrets are base64 decoded; unprefixed secrets are used as raw UTF-8.
    pub fn new(secret: &str) -> Result<Self> {
        let key = if let Some(encoded) = secret.strip_prefix("whsec_") {
            STANDARD
                .decode(encoded)
                .map_err(|_| Error::Webhook("invalid secret encoding".into()))?
        } else {
            secret.as_bytes().to_vec()
        };
        if key.is_empty() {
            return Err(Error::Webhook("secret is empty".into()));
        }
        Ok(Self {
            key,
            tolerance: Duration::from_secs(300),
        })
    }
    pub fn from_env() -> Result<Self> {
        let secret = std::env::var("OPENAI_WEBHOOK_SECRET")
            .map_err(|_| Error::Config("OPENAI_WEBHOOK_SECRET is not set".into()))?;
        Self::new(&secret)
    }
    pub fn tolerance(mut self, tolerance: Duration) -> Self {
        self.tolerance = tolerance;
        self
    }
    pub fn verify(&self, payload: &[u8], headers: &HeaderMap) -> Result<()> {
        self.verify_at(payload, headers, SystemTime::now())
    }
    /// Verify at a caller-supplied time, useful with an application clock.
    pub fn verify_at(&self, payload: &[u8], headers: &HeaderMap, now: SystemTime) -> Result<()> {
        let get = |name| -> Result<&str> {
            headers
                .get(name)
                .and_then(|v| v.to_str().ok())
                .filter(|v| !v.is_empty())
                .ok_or_else(|| Error::Webhook(format!("missing or invalid {name} header")))
        };
        let id = get("webhook-id")?;
        let timestamp = get("webhook-timestamp")?;
        let signature = get("webhook-signature")?;
        let time: u64 = timestamp
            .parse()
            .map_err(|_| Error::Webhook("invalid timestamp".into()))?;
        let now = now
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::Webhook("clock precedes Unix epoch".into()))?
            .as_secs();
        if now.abs_diff(time) > self.tolerance.as_secs() {
            return Err(Error::Webhook(
                "timestamp is outside the replay window".into(),
            ));
        }
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key)
            .map_err(|_| Error::Webhook("invalid signing key".into()))?;
        mac.update(id.as_bytes());
        mac.update(b".");
        mac.update(timestamp.as_bytes());
        mac.update(b".");
        mac.update(payload);
        for signatures in headers.get_all("webhook-signature") {
            let signatures = signatures.to_str().unwrap_or(signature);
            for part in signatures.split_whitespace() {
                let encoded = match part.split_once(',') {
                    Some(("v1", sig)) => sig,
                    Some(_) => continue,
                    None => part,
                };
                if let Ok(bytes) = STANDARD.decode(encoded)
                    && mac.clone().verify_slice(&bytes).is_ok()
                {
                    return Ok(());
                }
            }
        }
        Err(Error::Webhook("signature does not match".into()))
    }
    /// Authenticate before parsing JSON. Supply the untouched HTTP request body.
    pub fn unwrap(&self, payload: &[u8], headers: &HeaderMap) -> Result<WebhookEvent> {
        self.verify(payload, headers)?;
        Ok(serde_json::from_slice(payload)?)
    }
}
