use reqwest::{StatusCode, header::HeaderMap};
use std::ops::Deref;

/// Decoded data together with HTTP status, headers, and server request ID.
#[derive(Clone)]
pub struct ApiResponse<T> {
    pub data: T,
    pub status: StatusCode,
    pub headers: HeaderMap,
}

impl<T> ApiResponse<T> {
    pub fn request_id(&self) -> Option<&str> {
        self.headers.get("x-request-id")?.to_str().ok()
    }
    pub fn into_inner(self) -> T {
        self.data
    }
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> ApiResponse<U> {
        ApiResponse {
            data: f(self.data),
            status: self.status,
            headers: self.headers,
        }
    }
}

impl<T> Deref for ApiResponse<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> std::fmt::Debug for ApiResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiResponse")
            .field("status", &self.status)
            .field("request_id", &self.request_id())
            .finish_non_exhaustive()
    }
}
