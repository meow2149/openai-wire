# Architecture

`Client` shares immutable settings and a reqwest connection pool through `Arc`.
Resource methods return owned, lazy `Request<T>` values. The request type keeps
resource-specific code out of transport policy and avoids generic configuration
types or a procedural macro dependency.

`Request<T>` owns its relative path, query, headers, replayable body, and overrides.
Serialization and header failures are retained until execution. Changing the output
type with `cast<U>` preserves the request. The execution loop rebuilds each attempt
from the same body, uses one idempotency key, and respects server retry advice.
Successful HTTP bodies are read only by the selected response adapter.

Typed request structs use constructors for required fields and ordinary optional
fields. New parameters can be supplied with `field`. Models, roles, statuses, and
event discriminators remain strings because the server can add values independently
of a crate release. Complex modality/tool payloads use JSON values where complete
generated types would inflate the public surface. Flattened maps retain extra data.

`ApiResponse<T>` and `EventStream<T>` expose status and response headers alongside
the data. Debug implementations for transport wrappers omit bodies and header values.
API errors preserve raw diagnostics for callers without printing them implicitly.

SSE decoding consumes bytes as they arrive. Lines are collected once, decoded as
UTF-8 at delimiters, and joined into records. It handles split CRLF and codepoints,
multiline data, BOM, comments, IDs, and terminators. No background task, reconnect,
or arbitrary payload limit is added. Truncated records report a protocol error.
The caller owns generation lifecycle validation and cancellation.

Disk uploads use Tokio's file reader and reqwest streaming multipart parts. Files
are reopened on retry. The caller must keep their source stable during execution.
Pagination copies the request settings for each page, replaces cursor parameters,
and tracks previously used cursors to reject loops.

Optional WebSockets use tokio-tungstenite directly with the client's destination,
authentication, and default query. A connection can be split for concurrent work.
There is no automatic session recovery. HTTP pool/proxy/TLS settings do not apply
to this separate transport. Webhooks authenticate original bytes with HMAC-SHA256
before JSON decoding and enforce a configurable replay window.

REST route functions and the `CATALOG` list live in `endpoints.rs`.
Maintain the matching endpoint table in `docs/endpoints.md` when changing routes.

