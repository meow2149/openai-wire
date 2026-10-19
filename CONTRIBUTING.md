# Contributing

Keep transport policy in `request.rs`, parsing in `stream.rs`, and resource-specific
conveniences in `resources.rs`. Prefer ordinary structs and direct constructors.
Add types when they reduce application boilerplate; retain custom JSON and serde
escape hatches. Public source, comments, documentation, and examples are English.

Run the formatting and Clippy checks in README before submitting changes.
Examples read credentials from the environment. Never commit credentials or `.env` files.

## Update REST routes

Update `src/endpoints.rs` and the matching entries in `docs/endpoints.md` together.
Keep the route functions and `CATALOG` consistent. Preserve unknown response fields
and use extensible strings for API values that may change.

