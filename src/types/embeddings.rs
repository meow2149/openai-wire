use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    Text(String),
    Texts(Vec<String>),
    Tokens(Vec<u32>),
    TokenBatches(Vec<Vec<u32>>),
}
impl From<String> for EmbeddingInput {
    fn from(v: String) -> Self {
        Self::Text(v)
    }
}
impl From<&str> for EmbeddingInput {
    fn from(v: &str) -> Self {
        Self::Text(v.into())
    }
}
impl From<Vec<String>> for EmbeddingInput {
    fn from(v: Vec<String>) -> Self {
        Self::Texts(v)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    pub model: String,
    pub input: EmbeddingInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
impl EmbeddingRequest {
    pub fn new(model: impl Into<String>, input: impl Into<EmbeddingInput>) -> Self {
        Self {
            model: model.into(),
            input: input.into(),
            dimensions: None,
            encoding_format: Some("float".into()),
            extra: Map::new(),
        }
    }
    pub fn dimensions(mut self, dimensions: u32) -> Self {
        self.dimensions = Some(dimensions);
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingVector {
    Float(Vec<f32>),
    Base64(String),
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Embedding {
    pub object: String,
    pub index: u64,
    pub embedding: EmbeddingVector,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Embeddings {
    pub object: String,
    pub data: Vec<Embedding>,
    pub model: String,
    pub usage: super::Usage,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
