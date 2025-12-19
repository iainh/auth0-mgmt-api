# AGENTS.md

## Commands
- Build: `cargo build`
- Test all: `cargo test`
- Test single: `cargo test test_name` or `cargo test module::test_name`
- Check: `cargo check`
- Lint: `cargo clippy`
- Format: `cargo fmt`
- Windows build/test: Use `--target x86_64-pc-windows-gnu` (e.g., `cargo build --target x86_64-pc-windows-gnu`)

## Architecture
Async Rust client library for Auth0 Management API v2 using reqwest + tokio.
- `src/client.rs` - ManagementClient with builder pattern, token management, HTTP methods
- `src/api/` - Resource APIs (users, clients, connections) with CRUD operations
- `src/types/` - Request/response structs with serde derive
- `src/error.rs` - Error types using thiserror

## Code Style
- Use `thiserror` for error types, `Result<T>` alias from error module
- Async functions return `Result<T>` with `Auth0Error`
- Feature flags gate API modules (users, clients, connections)
- Use `serde` derive for all request/response types with `#[serde(skip_serializing_if)]` for Option fields
- URL-encode path parameters with `urlencoding::encode`
- Use `secrecy` crate for sensitive data (client secrets)
- Prefer `rustls-tls` over native-tls for cross-platform builds
