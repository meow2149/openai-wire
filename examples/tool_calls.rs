use openai_wire::{
    Client, Error,
    types::{
        Function,
        responses::{Input, ResponseRequest},
    },
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Arguments {
    city: String,
}

#[tokio::main]
async fn main() -> openai_wire::Result<()> {
    let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4.1-mini".into());
    let client = Client::from_env()?;
    let function = Function::new(
        "weather",
        json!({
            "type":"object", "properties":{"city":{"type":"string"}},
            "required":["city"], "additionalProperties":false
        }),
    )
    .description("Get demonstration weather data for a city.");
    let mut request =
        ResponseRequest::new(&model, "What is the weather in Paris?").function(function.clone());
    for _ in 0..8 {
        let response = client.responses().create(request).send().await?;
        let mut outputs = Vec::new();
        for call in response.function_calls() {
            if call.name.as_deref() != Some("weather") {
                return Err(Error::InvalidRequest("unexpected tool name".into()));
            }
            let arguments: Arguments = call.arguments()?;
            let call_id = call
                .call_id
                .as_deref()
                .ok_or_else(|| Error::InvalidRequest("missing call ID".into()))?;
            outputs.push(Input::function_output(
                call_id,
                json!({"city": arguments.city, "temperature_celsius":20, "source":"demo data"})
                    .to_string(),
            ));
        }
        if outputs.is_empty() {
            println!("{}", response.output_text());
            return Ok(());
        }
        request = ResponseRequest::new(&model, outputs)
            .previous_response(response.id)
            .function(function.clone());
    }
    Err(Error::InvalidRequest(
        "tool call round-trip budget exhausted".into(),
    ))
}
