use auth0_mgmt_api::{ManagementClient, PasswordChangeTicketRequest};
use wiremock::matchers::{bearer_token, body_json, method, path};
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
async fn creates_a_password_change_ticket_by_email_and_connection() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/api/v2/tickets/password-change"))
        .and(bearer_token("test_token"))
        .and(body_json(serde_json::json!({
            "email": "user@example.com",
            "connection_id": "con_123"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "ticket": "https://tenant.auth0.com/lo/reset?ticket=example"
        })))
        .mount(&server)
        .await;

    let response = client
        .tickets()
        .create_password_change(PasswordChangeTicketRequest {
            email: "user@example.com".to_owned(),
            connection_id: "con_123".to_owned(),
        })
        .await
        .expect("Failed to create password-change ticket");

    assert_eq!(
        response.ticket,
        "https://tenant.auth0.com/lo/reset?ticket=example"
    );
}
