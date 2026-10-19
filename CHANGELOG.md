# Changelog

## 0.1.0

- Shared async HTTP client with typed Responses, Chat Completions, embeddings,
  images, audio, files, models, batches, and moderation workflows.
- 268 named REST routes covering the supported reference API catalog, including
  organization administration, vector stores, conversations, evals, and skills.
- Custom request and response types, explicit null fields, HTTP metadata,
  incremental downloads, replayable multipart uploads, and cursor pagination.
- Bounded retries with exponential jitter, server-directed delays, and stable
  idempotency keys.
- Incremental SSE with fragmented UTF-8, multiline data, and stream errors.
- Optional JSON WebSockets and Standard Webhooks signature verification.
- Azure v1 and compatible base URLs, examples, and CI.

