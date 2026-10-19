//! Incremental server-sent events without a background task or fixed payload limits.
use crate::{ApiError, Error, Result};
use bytes::Bytes;
use futures_core::Stream;
use futures_util::StreamExt;
use reqwest::{StatusCode, header::HeaderMap};
use serde::de::DeserializeOwned;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes>> + Send>>;

/// An SSE record. `data` preserves multiline payloads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    pub event: String,
    pub data: String,
    pub id: Option<String>,
    pub retry: Option<u64>,
}
impl Event {
    pub fn json<T: DeserializeOwned>(&self) -> Result<T> {
        Ok(serde_json::from_str(&self.data)?)
    }

    fn is_error(&self) -> bool {
        self.event == "error"
            || self.json::<serde_json::Value>().ok().is_some_and(|value| {
                value.get("type").and_then(|v| v.as_str()) == Some("error")
                    || value.get("error").is_some_and(|v| !v.is_null())
            })
    }
}

/// A stream with HTTP metadata. Drop it to cancel; events are never reconnected
/// or replayed automatically once the HTTP response has started.
pub struct EventStream<T> {
    pub status: StatusCode,
    pub headers: HeaderMap,
    inner: Pin<Box<dyn Stream<Item = Result<T>> + Send>>,
}

impl<T> EventStream<T> {
    pub fn request_id(&self) -> Option<&str> {
        self.headers.get("x-request-id")?.to_str().ok()
    }
}
impl<T> Stream for EventStream<T> {
    type Item = Result<T>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
    }
}
impl<T> std::fmt::Debug for EventStream<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventStream")
            .field("status", &self.status)
            .field("request_id", &self.request_id())
            .finish_non_exhaustive()
    }
}

pub(crate) fn parse_events(response: reqwest::Response) -> EventStream<Event> {
    let status = response.status();
    let headers = response.headers().clone();
    let error_headers = headers.clone();
    let inner = async_stream::try_stream! {
        let mut bytes = response.bytes_stream();
        let mut decoder = Decoder::default();
        while let Some(chunk) = bytes.next().await {
            for byte in chunk? {
                if let Some(event) = decoder.push(byte)? {
                    if event.data.trim() == "[DONE]" {
                        return;
                    }
                    if event.is_error() {
                        Err(Error::Api(Box::new(ApiError::from_body(
                            None, error_headers.clone(), event.data.as_bytes(),
                        ))))?;
                    }
                    yield event;
                }
            }
        }
        if decoder.has_pending_data() {
            Err(Error::Stream("connection ended inside an SSE event".into()))?;
        }
    };
    EventStream {
        status,
        headers,
        inner: Box::pin(inner),
    }
}

pub(crate) fn decode_events<T: DeserializeOwned + Send + 'static>(
    events: EventStream<Event>,
) -> EventStream<T> {
    let EventStream {
        status,
        headers,
        inner: mut events,
    } = events;
    let inner = async_stream::try_stream! {
        while let Some(event) = events.next().await {
            yield event?.json::<T>()?;
        }
    };
    EventStream {
        status,
        headers,
        inner: Box::pin(inner),
    }
}

#[derive(Default)]
struct Decoder {
    line: Vec<u8>,
    data: String,
    event: String,
    id: Option<String>,
    retry: Option<u64>,
    skip_lf: bool,
    started: bool,
}

impl Decoder {
    fn push(&mut self, byte: u8) -> Result<Option<Event>> {
        if self.skip_lf {
            self.skip_lf = false;
            if byte == b'\n' {
                return Ok(None);
            }
        }
        if byte == b'\r' || byte == b'\n' {
            self.skip_lf = byte == b'\r';
            return self.finish_line();
        }
        self.line.push(byte);
        Ok(None)
    }
    fn finish_line(&mut self) -> Result<Option<Event>> {
        let bytes = std::mem::take(&mut self.line);
        let line = std::str::from_utf8(&bytes)
            .map_err(|_| Error::Stream("SSE contains invalid UTF-8".into()))?;
        let line = if !self.started {
            self.started = true;
            line.trim_start_matches('\u{feff}')
        } else {
            line
        };
        if line.is_empty() {
            let event = if !self.data.is_empty() {
                self.data.pop();
                Some(Event {
                    event: if self.event.is_empty() {
                        "message".into()
                    } else {
                        std::mem::take(&mut self.event)
                    },
                    data: std::mem::take(&mut self.data),
                    id: self.id.clone(),
                    retry: self.retry,
                })
            } else {
                None
            };
            self.event.clear();
            self.data.clear();
            return Ok(event);
        }
        if line.starts_with(':') {
            return Ok(None);
        }
        let (name, value) = line.split_once(':').unwrap_or((line, ""));
        let value = value.strip_prefix(' ').unwrap_or(value);
        match name {
            "data" => {
                self.data.push_str(value);
                self.data.push('\n');
            }
            "event" => {
                self.event = value.into();
            }
            "id" if !value.contains('\0') => {
                self.id = Some(value.into());
            }
            "retry" if !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) => {
                self.retry = value.parse().ok();
            }
            _ => {}
        }
        Ok(None)
    }
    fn has_pending_data(&self) -> bool {
        !self.data.is_empty() || !self.line.is_empty()
    }
}
