use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModerationRequest {
    pub model: String,
    pub input: Value,
}
impl ModerationRequest {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            model: "omni-moderation-latest".into(),
            input: Value::String(input.into()),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Moderation {
    pub id: String,
    pub model: String,
    pub results: Vec<ModerationResult>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModerationResult {
    pub flagged: bool,
    pub categories: Map<String, Value>,
    pub category_scores: Map<String, Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
