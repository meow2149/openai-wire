//! Types for common workflows. Optional request fields are omitted when unset;
//! use `Request::field` to send explicit null or newly introduced API parameters.
pub mod audio;
pub mod batches;
pub mod chat;
pub mod embeddings;
pub mod files;
pub mod images;
pub mod models;
pub mod moderations;
pub mod responses;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Confirmation returned by resource deletion endpoints.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deleted {
    pub id: String,
    pub deleted: bool,
    #[serde(default)]
    pub object: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Usage fields shared by Responses and Chat Completions. API-specific details
/// remain available in `extra`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Usage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<u64>,
    pub total_tokens: u64,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A function tool using JSON Schema arguments.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameters: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}
impl Function {
    pub fn new(name: impl Into<String>, parameters: Value) -> Self {
        Self {
            name: name.into(),
            parameters,
            description: None,
            strict: Some(true),
        }
    }
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }
}
