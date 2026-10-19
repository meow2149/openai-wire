# Releasing

1. Update the version in `Cargo.toml` and add a changelog entry.
2. Run the formatting and Clippy checks in README, then check the package:

   ```sh
   cargo package --locked
   ```

3. Commit the changes and tag the version, for example `v0.1.0`.
4. Push the commit and tag. Run `cargo publish --locked` to publish to crates.io.

CI checks formatting and Clippy on stable Rust. Releases are published manually.

