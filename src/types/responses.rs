use super::{Function, Usage};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

/// Text or a sequence of API input items. JSON items support every input modality
/// and can carry previous output items unchanged.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input {
    Text(String),
    Items(Vec<Value>),
}
impl From<String> for Input {
    fn from(v: String) -> Self {
        Self::Text(v)
    }
}
impl From<&str> for Input {
    fn from(v: &str) -> Self {
        Self::Text(v.into())
    }
}
impl From<Vec<Value>> for Input {
    fn from(v: Vec<Value>) -> Self {
        Self::Items(v)
    }
}

impl Input {
    pub fn message(role: &str, text: impl Into<String>) -> Value {
        json!({"role": role, "content": [{"type": "input_text", "text": text.into()}]})
    }
    pub fn image(url: impl Into<String>) -> Value {
        json!({"type": "input_image", "image_url": url.into()})
    }
    pub fn function_output(call_id: &str, output: impl Into<String>) -> Value {
        json!({"type": "function_call_output", "call_id": call_id, "output": output.into()})
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseRequest {
    pub model: String,
    pub input: Input,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Map<String, Value>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl ResponseRequest {
    pub fn new(model: impl Into<String>, input: impl Into<Input>) -> Self {
        Self {
            model: model.into(),
            input: input.into(),
            instructions: None,
            max_output_tokens: None,
            previous_response_id: None,
            conversation: None,
            temperature: None,
            top_p: None,
            store: None,
            background: None,
            tools: Vec::new(),
            tool_choice: None,
            parallel_tool_calls: None,
            reasoning: None,
            text: None,
            metadata: None,
            include: Vec::new(),
            extra: Map::new(),
        }
    }
    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }
    pub fn max_output_tokens(mut self, value: u64) -> Self {
        self.max_output_tokens = Some(value);
        self
    }
    pub fn previous_response(mut self, id: impl Into<String>) -> Self {
        self.previous_response_id = Some(id.into());
        self
    }
    pub fn tool(mut self, tool: Value) -> Self {
        self.tools.push(tool);
        self
    }
    pub fn function(self, function: Function) -> Self {
        let mut tool = json!(function);
        tool["type"] = json!("function");
        self.tool(tool)
    }
    pub fn json_schema(mut self, name: &str, schema: Value) -> Self {
        self.text = Some(
            json!({"format": {"type": "json_schema", "name": name, "strict": true, "schema": schema}}),
        );
        self
    }
}

/// A response output item. Open string discriminators and flattened fields retain
/// newly added tool outputs without a library update.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputItem {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<Content>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
impl OutputItem {
    pub fn arguments<T: serde::de::DeserializeOwned>(&self) -> crate::Result<T> {
        Ok(serde_json::from_str(
            self.arguments.as_deref().ok_or_else(|| {
                crate::Error::InvalidRequest("item has no function arguments".into())
            })?,
        )?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub model: String,
    pub status: String,
    #[serde(default)]
    pub output: Vec<OutputItem>,
    pub usage: Option<Usage>,
    pub error: Option<Value>,
    pub incomplete_details: Option<Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
impl Response {
    pub fn output_text(&self) -> String {
        self.output
            .iter()
            .filter(|item| item.kind == "message")
            .flat_map(|item| item.content.iter().flatten())
            .filter(|part| part.kind == "output_text")
            .filter_map(|part| part.text.as_deref())
            .collect()
    }
    pub fn function_calls(&self) -> impl Iterator<Item = &OutputItem> {
        self.output
            .iter()
            .filter(|item| item.kind == "function_call")
    }
    /// Parse structured output. Failure or incomplete generation must be handled first.
    pub fn parse<T: serde::de::DeserializeOwned>(&self) -> crate::Result<T> {
        if self.status != "completed" {
            return Err(crate::Error::InvalidRequest(
                "response is not completed".into(),
            ));
        }
        Ok(serde_json::from_str(&self.output_text())?)
    }
}

/// Semantic streaming events, retaining unknown event names and payload fields.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseEvent {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Response>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_index: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_index: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<OutputItem>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
impl ResponseEvent {
    pub fn text_delta(&self) -> Option<&str> {
        if self.kind == "response.output_text.delta" {
            self.delta.as_deref()
        } else {
            None
        }
    }
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.kind.as_str(),
            "response.completed" | "response.failed" | "response.incomplete" | "response.cancelled"
        )
    }
}
