use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
impl ImageRequest {
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: prompt.into(),
            size: None,
            quality: None,
            n: None,
            output_format: None,
            background: None,
            extra: Map::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Images {
    pub created: u64,
    pub data: Vec<Image>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    pub b64_json: Option<String>,
    pub url: Option<String>,
    pub revised_prompt: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
