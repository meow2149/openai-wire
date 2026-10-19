use openai_wire::{Client, types::responses::ResponseRequest};

#[tokio::main]
async fn main() -> openai_wire::Result<()> {
    let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4.1-mini".into());
    let client = Client::from_env()?;
    let response = client
        .responses()
        .create(ResponseRequest::new(
            model,
            "Explain Rust ownership in three sentences.",
        ))
        .response()
        .await?;
    println!("Request ID: {:?}", response.request_id());
    println!("{}", response.output_text());
    Ok(())
}
