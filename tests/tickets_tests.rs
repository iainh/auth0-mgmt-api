use auth0_mgmt_api::{
    ClientId, ManagementClient, PasswordChangeTicketIdentity, PasswordChangeTicketRequest,
    PasswordChangeTicketTarget, UserId,
};
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
        .create_password_change(PasswordChangeTicketRequest::for_email(
            "user@example.com",
            "con_123",
        ))
        .await
        .expect("Failed to create password-change ticket");

    assert_eq!(
        response.ticket,
        "https://tenant.auth0.com/lo/reset?ticket=example"
    );
}

#[tokio::test]
async fn creates_a_password_change_ticket_by_user_with_options() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/api/v2/tickets/password-change"))
        .and(bearer_token("test_token"))
        .and(body_json(serde_json::json!({
            "user_id": "auth0|123",
            "identity": { "user_id": "123", "provider": "auth0" },
            "client_id": "client_123",
            "ttl_sec": 3600,
            "mark_email_as_verified": true,
            "includeEmailInRedirect": false
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "ticket": "https://tenant.auth0.com/lo/reset?ticket=user"
        })))
        .mount(&server)
        .await;

    let response = client
        .tickets()
        .create_password_change(PasswordChangeTicketRequest {
            target: PasswordChangeTicketTarget::User {
                user_id: UserId::new("auth0|123"),
                identity: Some(PasswordChangeTicketIdentity::auth0("123")),
            },
            client_id: Some(ClientId::new("client_123")),
            ttl_sec: Some(3600),
            mark_email_as_verified: Some(true),
            include_email_in_redirect: Some(false),
            ..PasswordChangeTicketRequest::for_user("unused")
        })
        .await
        .expect("Failed to create password-change ticket");

    assert_eq!(
        response.ticket,
        "https://tenant.auth0.com/lo/reset?ticket=user"
    );
}

#[test]
fn user_ticket_request_serializes_only_the_user_id() {
    let body = serde_json::to_value(PasswordChangeTicketRequest::for_user("auth0|123"))
        .expect("Failed to serialize request");

    assert_eq!(body, serde_json::json!({ "user_id": "auth0|123" }));
}
