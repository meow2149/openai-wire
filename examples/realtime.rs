use openai_wire::Client;
use serde_json::{Value, json};

#[tokio::main]
async fn main() -> openai_wire::Result<()> {
    let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-realtime".into());
    let mut connection = Client::from_env()?.realtime(&model).await?;
    connection
        .update_session(json!({"type":"realtime", "output_modalities":["text"]}))
        .await?;
    connection
        .send(json!({"type":"conversation.item.create", "item":{
            "type":"message", "role":"user", "content":[{"type":"input_text", "text":"Say hello."}]
        }}))
        .await?;
    connection.send(json!({"type":"response.create"})).await?;
    let mut completed = false;
    while let Some(event) = connection.receive::<Value>().await? {
        match event["type"].as_str() {
            Some("response.output_text.delta") => {
                print!("{}", event["delta"].as_str().unwrap_or_default())
            }
            Some("response.done") => {
                completed = event["response"]["status"] == "completed";
                break;
            }
            Some("error") => {
                return Err(openai_wire::Error::Stream(
                    "Realtime API reported an error".into(),
                ));
            }
            _ => {}
        }
    }
    println!();
    connection.close().await?;
    if !completed {
        return Err(openai_wire::Error::Stream(
            "session ended without a completed response".into(),
        ));
    }
    Ok(())
}
