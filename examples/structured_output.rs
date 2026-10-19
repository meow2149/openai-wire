use openai_wire::{Client, types::responses::ResponseRequest};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct Answer {
    explanation: String,
    languages: Vec<String>,
}

#[tokio::main]
async fn main() -> openai_wire::Result<()> {
    let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4.1-mini".into());
    let request = ResponseRequest::new(model, "Explain which languages can compile to WebAssembly.")
        .json_schema("answer", json!({
            "type": "object", "properties": {
                "explanation": {"type":"string"}, "languages": {"type":"array", "items":{"type":"string"}}
            }, "required": ["explanation", "languages"], "additionalProperties": false
        }));
    let response = Client::from_env()?
        .responses()
        .create(request)
        .send()
        .await?;
    let answer: Answer = response.parse()?;
    println!(
        "{}\nLanguages: {}",
        answer.explanation,
        answer.languages.join(", ")
    );
    Ok(())
}
