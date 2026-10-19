use futures_util::StreamExt;
use openai_wire::{
    Client, Error,
    types::responses::{ResponseEvent, ResponseRequest},
};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> openai_wire::Result<()> {
    let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4.1-mini".into());
    let mut events = Client::from_env()?
        .responses()
        .create(ResponseRequest::new(
            model,
            "Write a short story about a curious robot.",
        ))
        .stream::<ResponseEvent>()
        .await?;
    let mut completed = false;
    while let Some(event) = events.next().await {
        let event = event?;
        if let Some(text) = event.text_delta() {
            print!("{text}");
            io::stdout().flush()?;
        }
        if event.is_terminal() {
            completed = event.kind == "response.completed";
            if !completed {
                return Err(Error::Stream(format!(
                    "generation ended with {}",
                    event.kind
                )));
            }
        }
    }
    println!();
    if !completed {
        return Err(Error::Stream(
            "connection ended before response.completed".into(),
        ));
    }
    Ok(())
}
