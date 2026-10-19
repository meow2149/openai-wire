use super::{Function, Usage};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCall>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
impl Message {
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: Some(Value::String(content.into())),
            name: None,
            tool_call_id: None,
            tool_calls: Vec::new(),
            refusal: None,
            extra: Map::new(),
        }
    }
    pub fn user(text: impl Into<String>) -> Self {
        Self::new("user", text)
    }
    pub fn system(text: impl Into<String>) -> Self {
        Self::new("system", text)
    }
    pub fn developer(text: impl Into<String>) -> Self {
        Self::new("developer", text)
    }
    pub fn assistant(text: impl Into<String>) -> Self {
        Self::new("assistant", text)
    }
    pub fn tool(call_id: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            tool_call_id: Some(call_id.into()),
            ..Self::new("tool", output)
        }
    }
    pub fn text(&self) -> Option<&str> {
        self.content.as_ref()?.as_str()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub function: FunctionCall,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}
impl FunctionCall {
    pub fn parse<T: serde::de::DeserializeOwned>(&self) -> crate::Result<T> {
        Ok(serde_json::from_str(&self.arguments)?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
impl ChatRequest {
    pub fn new(model: impl Into<String>, messages: impl Into<Vec<Message>>) -> Self {
        Self {
            model: model.into(),
            messages: messages.into(),
            temperature: None,
            max_completion_tokens: None,
            top_p: None,
            store: None,
            tools: Vec::new(),
            tool_choice: None,
            response_format: None,
            extra: Map::new(),
        }
    }
    pub fn function(mut self, function: Function) -> Self {
        self.tools
            .push(json!({"type": "function", "function": function}));
        self
    }
    pub fn json_schema(mut self, name: &str, schema: Value) -> Self {
        self.response_format = Some(
            json!({"type": "json_schema", "json_schema": {"name": name, "strict": true, "schema": schema}}),
        );
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatCompletion {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub usage: Option<Usage>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Choice {
    pub index: u64,
    pub message: Message,
    pub finish_reason: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
impl ChatCompletion {
    pub fn text(&self) -> Option<&str> {
        self.choices.first()?.message.text()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChunkChoice>,
    #[serde(default)]
    pub usage: Option<Usage>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChunkChoice {
    pub index: u64,
    pub delta: Delta,
    pub finish_reason: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Delta {
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<Value>>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
impl ChatChunk {
    pub fn text_delta(&self) -> Option<&str> {
        self.choices.first()?.delta.content.as_deref()
    }
}
