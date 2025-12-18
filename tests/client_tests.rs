use auth0_mgmt_api::{ManagementClient, RetryConfig};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn setup_mock_server() -> (MockServer, ManagementClient) {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "test_token",
            "expires_in": 86400,
            "token_type": "Bearer"
        })))
        .mount(&server)
        .await;

    let client = ManagementClient::builder()
        .domain(server.uri())
        .client_id("test_client_id")
        .client_secret("test_client_secret")
        .build()
        .expect("Failed to build client");

    (server, client)
}

#[tokio::test]
async fn test_concurrent_token_refresh_single_request() {
    let server = MockServer::start().await;
    let token_request_count = Arc::new(AtomicUsize::new(0));
    let count_clone = token_request_count.clone();

    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(move |_: &wiremock::Request| {
            count_clone.fetch_add(1, Ordering::SeqCst);
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "access_token": "test_token",
                    "expires_in": 86400,
                    "token_type": "Bearer"
                }))
        })
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .mount(&server)
        .await;

    let client = ManagementClient::builder()
        .domain(server.uri())
        .client_id("test_client_id")
        .client_secret("test_client_secret")
        .build()
        .expect("Failed to build client");

    let mut handles = vec![];
    for _ in 0..10 {
        let client_clone = client.clone();
        handles.push(tokio::spawn(async move {
            client_clone.users().list(None).await
        }));
    }

    for handle in handles {
        handle.await.expect("Task panicked").expect("Request failed");
    }

    assert_eq!(
        token_request_count.load(Ordering::SeqCst),
        1,
        "Token should only be requested once despite concurrent requests"
    );
}

#[tokio::test]
async fn test_token_reuse_across_requests() {
    let server = MockServer::start().await;
    let token_request_count = Arc::new(AtomicUsize::new(0));
    let count_clone = token_request_count.clone();

    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(move |_: &wiremock::Request| {
            count_clone.fetch_add(1, Ordering::SeqCst);
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "access_token": "test_token",
                    "expires_in": 86400,
                    "token_type": "Bearer"
                }))
        })
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .mount(&server)
        .await;

    let client = ManagementClient::builder()
        .domain(server.uri())
        .client_id("test_client_id")
        .client_secret("test_client_secret")
        .build()
        .expect("Failed to build client");

    for _ in 0..5 {
        client.users().list(None).await.expect("Request failed");
    }

    assert_eq!(
        token_request_count.load(Ordering::SeqCst),
        1,
        "Token should only be requested once and reused"
    );
}

#[tokio::test]
async fn test_client_clone_shares_token() {
    let server = MockServer::start().await;
    let token_request_count = Arc::new(AtomicUsize::new(0));
    let count_clone = token_request_count.clone();

    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(move |_: &wiremock::Request| {
            count_clone.fetch_add(1, Ordering::SeqCst);
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "access_token": "test_token",
                    "expires_in": 86400,
                    "token_type": "Bearer"
                }))
        })
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .mount(&server)
        .await;

    let client = ManagementClient::builder()
        .domain(server.uri())
        .client_id("test_client_id")
        .client_secret("test_client_secret")
        .build()
        .expect("Failed to build client");

    let client2 = client.clone();
    let client3 = client.clone();

    client.users().list(None).await.expect("Request failed");
    client2.users().list(None).await.expect("Request failed");
    client3.users().list(None).await.expect("Request failed");

    assert_eq!(
        token_request_count.load(Ordering::SeqCst),
        1,
        "Cloned clients should share the same token"
    );
}

#[tokio::test]
async fn test_builder_missing_domain() {
    let result = ManagementClient::builder()
        .client_id("test_client_id")
        .client_secret("test_client_secret")
        .build();

    assert!(result.is_err());
}

#[tokio::test]
async fn test_builder_missing_client_id() {
    let result = ManagementClient::builder()
        .domain("test.auth0.com")
        .client_secret("test_client_secret")
        .build();

    assert!(result.is_err());
}

#[tokio::test]
async fn test_builder_missing_client_secret() {
    let result = ManagementClient::builder()
        .domain("test.auth0.com")
        .client_id("test_client_id")
        .build();

    assert!(result.is_err());
}

#[tokio::test]
async fn test_builder_with_full_url() {
    let (server, _client) = setup_mock_server().await;

    let client = ManagementClient::builder()
        .domain(server.uri())
        .client_id("test_client_id")
        .client_secret("test_client_secret")
        .build();

    assert!(client.is_ok());
}

#[tokio::test]
async fn test_token_auth_failure() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "error": "access_denied",
            "error_description": "Invalid credentials"
        })))
        .mount(&server)
        .await;

    let client = ManagementClient::builder()
        .domain(server.uri())
        .client_id("invalid_client_id")
        .client_secret("invalid_client_secret")
        .build()
        .expect("Failed to build client");

    let result = client.users().list(None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_token_refresh_retry_on_503() {
    let server = MockServer::start().await;
    let attempt_count = Arc::new(AtomicUsize::new(0));
    let count_clone = attempt_count.clone();

    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(move |_: &wiremock::Request| {
            let attempt = count_clone.fetch_add(1, Ordering::SeqCst);
            if attempt < 2 {
                ResponseTemplate::new(503)
            } else {
                ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "access_token": "test_token",
                    "expires_in": 86400,
                    "token_type": "Bearer"
                }))
            }
        })
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .mount(&server)
        .await;

    let client = ManagementClient::builder()
        .domain(server.uri())
        .client_id("test_client_id")
        .client_secret("test_client_secret")
        .retry_config(RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(100),
            multiplier: 2.0,
        })
        .build()
        .expect("Failed to build client");

    let result = client.users().list(None).await;
    assert!(result.is_ok(), "Should succeed after retries");
    assert_eq!(attempt_count.load(Ordering::SeqCst), 3, "Should have made 3 attempts");
}

#[tokio::test]
async fn test_token_refresh_retry_exhaustion() {
    let server = MockServer::start().await;
    let attempt_count = Arc::new(AtomicUsize::new(0));
    let count_clone = attempt_count.clone();

    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(move |_: &wiremock::Request| {
            count_clone.fetch_add(1, Ordering::SeqCst);
            ResponseTemplate::new(503)
        })
        .mount(&server)
        .await;

    let client = ManagementClient::builder()
        .domain(server.uri())
        .client_id("test_client_id")
        .client_secret("test_client_secret")
        .retry_config(RetryConfig {
            max_retries: 2,
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(100),
            multiplier: 2.0,
        })
        .build()
        .expect("Failed to build client");

    let result = client.users().list(None).await;
    assert!(result.is_err(), "Should fail after exhausting retries");
    assert_eq!(attempt_count.load(Ordering::SeqCst), 3, "Should have made initial + 2 retry attempts");
}

#[tokio::test]
async fn test_token_refresh_retry_with_rate_limit() {
    let server = MockServer::start().await;
    let attempt_count = Arc::new(AtomicUsize::new(0));
    let count_clone = attempt_count.clone();

    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(move |_: &wiremock::Request| {
            let attempt = count_clone.fetch_add(1, Ordering::SeqCst);
            if attempt == 0 {
                ResponseTemplate::new(429).insert_header("retry-after", "1")
            } else {
                ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "access_token": "test_token",
                    "expires_in": 86400,
                    "token_type": "Bearer"
                }))
            }
        })
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .mount(&server)
        .await;

    let client = ManagementClient::builder()
        .domain(server.uri())
        .client_id("test_client_id")
        .client_secret("test_client_secret")
        .retry_config(RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_secs(5),
            multiplier: 2.0,
        })
        .build()
        .expect("Failed to build client");

    let result = client.users().list(None).await;
    assert!(result.is_ok(), "Should succeed after rate limit retry");
    assert_eq!(attempt_count.load(Ordering::SeqCst), 2, "Should have made 2 attempts");
}

#[tokio::test]
async fn test_no_retry_on_auth_failure() {
    let server = MockServer::start().await;
    let attempt_count = Arc::new(AtomicUsize::new(0));
    let count_clone = attempt_count.clone();

    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(move |_: &wiremock::Request| {
            count_clone.fetch_add(1, Ordering::SeqCst);
            ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "access_denied",
                "error_description": "Invalid credentials"
            }))
        })
        .mount(&server)
        .await;

    let client = ManagementClient::builder()
        .domain(server.uri())
        .client_id("invalid_client_id")
        .client_secret("invalid_client_secret")
        .retry_config(RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(100),
            multiplier: 2.0,
        })
        .build()
        .expect("Failed to build client");

    let result = client.users().list(None).await;
    assert!(result.is_err(), "Should fail on auth error");
    assert_eq!(attempt_count.load(Ordering::SeqCst), 1, "Should not retry on auth failure");
}
