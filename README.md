# auth0-mgmt-api

Rust client library for the [Auth0 Management API v2](https://auth0.com/docs/api/management/v2).

## Features

- Async/await support via Tokio
- Automatic token management (fetches and refreshes M2M tokens)
- TLS via rustls (cross-platform, no OpenSSL dependency)
- Type-safe request/response models
- Feature flags for optional API resources

## Installation

```toml
[dependencies]
auth0-mgmt-api = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Usage

```rust
use auth0_mgmt_api::{ManagementClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ManagementClient::builder()
        .domain("your-tenant.auth0.com")
        .client_id("your-client-id")
        .client_secret("your-client-secret")
        .build()?;

    // List users
    let users = client.users().list(None).await?;
    for user in users {
        println!("User: {} - {:?}", user.user_id, user.email);
    }

    // Get a specific user
    let user = client.users().get("auth0|123456").await?;

    // Create a user
    use auth0_mgmt_api::CreateUserRequest;
    let new_user = client.users().create(CreateUserRequest {
        connection: "Username-Password-Authentication".into(),
        email: Some("user@example.com".into()),
        password: Some("SecurePassword123!".into()),
        ..Default::default()
    }).await?;

    Ok(())
}
```

## Feature Flags

By default, all API resources are enabled. You can disable them to reduce compile time:

```toml
[dependencies]
auth0-mgmt-api = { version = "0.1", default-features = false, features = ["users"] }
```

Available features:
- `users` - Users API
- `clients` - Applications/Clients API  
- `connections` - Connections API

## Auth0 Setup

1. Create a Machine-to-Machine application in your Auth0 dashboard
2. Authorize it for the Auth0 Management API
3. Grant the required scopes (e.g., `read:users`, `create:users`, etc.)
4. Use the client ID and secret in your application

## License

MIT OR Apache-2.0
