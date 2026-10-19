#![doc = include_str!("../README.md")]

mod client;
pub mod endpoints;
mod error;
mod pagination;
mod request;
pub mod resources;
mod response;
pub mod stream;
pub mod types;
mod upload;
#[cfg(feature = "webhooks")]
pub mod webhooks;
#[cfg(feature = "websocket")]
pub mod websocket;

pub use client::{Client, ClientBuilder};
pub use error::{ApiError, Error, Result};
pub use pagination::Page;
pub use request::Request;
pub use reqwest::{Method, StatusCode, Url, header};
pub use response::ApiResponse;
pub use stream::{Event, EventStream};
pub use upload::{Multipart, Upload};
