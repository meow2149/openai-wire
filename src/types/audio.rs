use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpeechRequest {
    pub model: String,
    pub input: String,
    pub voice: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
impl SpeechRequest {
    pub fn new(
        model: impl Into<String>,
        voice: impl Into<String>,
        input: impl Into<String>,
    ) -> Self {
        Self {
            model: model.into(),
            voice: Value::String(voice.into()),
            input: input.into(),
            response_format: None,
            speed: None,
            instructions: None,
            extra: Map::new(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transcription {
    pub text: String,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
