use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchRequest {
    pub input_file_id: String,
    pub endpoint: String,
    pub completion_window: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Map<String, Value>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
impl BatchRequest {
    pub fn new(file_id: impl Into<String>, endpoint: impl Into<String>) -> Self {
        Self {
            input_file_id: file_id.into(),
            endpoint: endpoint.into(),
            completion_window: "24h".into(),
            metadata: None,
            extra: Map::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Batch {
    pub id: String,
    pub object: String,
    pub endpoint: String,
    pub input_file_id: String,
    pub completion_window: String,
    pub status: String,
    pub created_at: u64,
    pub output_file_id: Option<String>,
    pub error_file_id: Option<String>,
    pub request_counts: Option<RequestCounts>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestCounts {
    pub total: u64,
    pub completed: u64,
    pub failed: u64,
}
