# openai-wire

An async Rust client for OpenAI with typed requests, streaming, and realtime support.

## Install

```toml
[dependencies]
openai-wire = "0.1"
tokio = { version = "1.53", features = ["macros", "rt-multi-thread"] }
```

Before publication, use `openai-wire = { path = "../openai-wire" }` or the
[Git repository](https://github.com/meow2149/openai-wire).

Set `OPENAI_API_KEY` in your environment. Use stable Rust and a Tokio runtime.

## Quick start

```rust
use openai_wire::{Client, types::responses::ResponseRequest};

#[tokio::main]
async fn main() -> openai_wire::Result<()> {
    let client = Client::from_env()?;
    let response = client.responses()
        .create(ResponseRequest::new("gpt-4.1-mini", "Explain Rust ownership briefly."))
        .send().await?;
    println!("{}", response.output_text());
    Ok(())
}
```

Set optional fields on request structs or use `.field("max_output_tokens", 128)`.
Use `.timeout(...)` and `.max_retries(...)` to override request settings.
`.response().await?` includes response headers and the request ID.

## Streaming

Add `futures-util = "0.3"` and use the following inside an async function:

```rust
use futures_util::StreamExt;
use openai_wire::{Client, types::responses::{ResponseEvent, ResponseRequest}};

let mut events = Client::from_env()?.responses()
    .create(ResponseRequest::new("gpt-4.1-mini", "Tell me a short story."))
    .stream::<ResponseEvent>().await?;
while let Some(event) = events.next().await {
    let event = event?;
    if let Some(text) = event.text_delta() { print!("{text}"); }
    if event.is_terminal() { println!("\n{}", event.kind); }
}
```

Handle failed or incomplete terminal events. A closed stream alone does not mean
success. Dropping the stream cancels it; started streams are not retried.

## APIs

Typed helpers cover Responses, Chat Completions, embeddings, images, audio,
files, models, batches, and moderation. The [endpoint catalog](docs/endpoints.md)
lists 268 REST routes, including administration, vector stores, and fine-tuning.

Use `client.endpoint(...)` for named routes or
`client.request::<T>(Method::POST, "/path")` for custom endpoints.
`.json(...)` accepts serializable data; `.cast::<T>()` selects your response type.
`.items()` paginates lists, `Upload` handles files, and `.bytes()` or
`.byte_stream()` downloads binary responses.

## Configuration

`Client::from_env()` reads `OPENAI_API_KEY`, `OPENAI_BASE_URL`, `OPENAI_ORG_ID`,
and `OPENAI_PROJECT_ID`. `Client::builder()` configures these explicitly and
supports custom HTTP clients and Azure v1. For administrative routes,
`Client::admin_from_env()` reads `OPENAI_ADMIN_KEY`.

Requests retry connection failures, timeouts, and HTTP 408/409/429/5xx up to twice
by default, respecting server retry headers. Use `.max_retries(0)` to disable
retries. `Error::api_error()` exposes API details, headers, and the raw error body.

| Feature | Default | Purpose |
| --- | --- | --- |
| `rustls` | Yes | HTTPS and WSS |
| `webhooks` | Yes | Webhook signature verification |
| `websocket` | No | Realtime and Responses WebSockets |

Verify webhook body bytes with `WebhookVerifier` before parsing.
Enable `features = ["websocket"]` for `client.realtime(...)` and
`client.responses_websocket()`. Applications handle reconnection.
With default features disabled, HTTPS needs a TLS-enabled custom client.

## Examples and development

```sh
cargo run --example responses
cargo run --example stream
cargo run --example realtime --features websocket
```

Examples read `OPENAI_API_KEY` and accept `OPENAI_MODEL` where applicable.
See [examples](https://github.com/meow2149/openai-wire/tree/main/examples)
for chat, tool calls, structured output, embeddings, and files.

```sh
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
```

CI uses stable Rust. See [architecture](docs/architecture.md),
[contributing](CONTRIBUTING.md), and [releasing](docs/releasing.md).
Licensed under [MIT](./LICENSE).
