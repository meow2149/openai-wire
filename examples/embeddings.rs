use openai_wire::{
    Client,
    types::embeddings::{EmbeddingRequest, EmbeddingVector},
};

#[tokio::main]
async fn main() -> openai_wire::Result<()> {
    let result = Client::from_env()?
        .embeddings()
        .create(EmbeddingRequest::new(
            "text-embedding-3-small",
            "A library should be easy to understand.",
        ))
        .send()
        .await?;
    for embedding in result.data {
        if let EmbeddingVector::Float(vector) = embedding.embedding {
            println!(
                "Embedding {} has {} dimensions",
                embedding.index,
                vector.len()
            );
        }
    }
    Ok(())
}
