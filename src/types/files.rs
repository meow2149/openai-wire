use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub object: String,
    pub bytes: u64,
    pub created_at: u64,
    pub filename: String,
    pub purpose: String,
    pub expires_at: Option<u64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
