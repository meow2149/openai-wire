use openai_wire::{
    Client,
    types::chat::{ChatRequest, Message},
};

#[tokio::main]
async fn main() -> openai_wire::Result<()> {
    let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4.1-mini".into());
    let result = Client::from_env()?
        .chat()
        .create(ChatRequest::new(
            model,
            vec![
                Message::developer("You explain concepts concisely."),
                Message::user("What is an async Rust future?"),
            ],
        ))
        .send()
        .await?;
    if let Some(text) = result.text() {
        println!("{text}");
    }
    Ok(())
}
