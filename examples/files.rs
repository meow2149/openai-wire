use openai_wire::{Client, Error, Upload};

#[tokio::main]
async fn main() -> openai_wire::Result<()> {
    let path = std::env::args().nth(1).ok_or_else(|| {
        Error::InvalidRequest("usage: cargo run --example files -- <path>".into())
    })?;
    let client = Client::from_env()?;
    let file = client
        .files()
        .upload("user_data", Upload::path(path)?)
        .send()
        .await?;
    println!(
        "Uploaded {} as {} ({} bytes)",
        file.filename, file.id, file.bytes
    );
    Ok(())
}
