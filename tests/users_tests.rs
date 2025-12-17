use auth0_mgmt_api::{
    CreateUserRequest, GetUserLogsParams, ListUsersParams, ManagementClient, UpdateUserRequest,
    UserId,
};
use wiremock::matchers::{bearer_token, body_json, method, path, query_param};
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
async fn test_list_users() {
    let (server, client) = setup_mock_server().await;

    let users_response = serde_json::json!([
        {
            "user_id": "auth0|123456789",
            "email": "test@example.com",
            "email_verified": true,
            "name": "Test User",
            "nickname": "test",
            "created_at": "2023-01-01T00:00:00.000Z",
            "updated_at": "2023-11-15T10:00:00.000Z",
            "logins_count": 5
        },
        {
            "user_id": "auth0|987654321",
            "email": "another@example.com",
            "email_verified": false,
            "name": "Another User"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&users_response))
        .mount(&server)
        .await;

    let users = client
        .users()
        .list(None)
        .await
        .expect("Failed to list users");

    assert_eq!(users.len(), 2);
    assert_eq!(users[0].user_id, "auth0|123456789");
    assert_eq!(users[0].email, Some("test@example.com".to_string()));
    assert_eq!(users[0].email_verified, Some(true));
    assert_eq!(users[1].user_id, "auth0|987654321");
    assert_eq!(users[1].email_verified, Some(false));
}

#[tokio::test]
async fn test_list_users_with_params() {
    let (server, client) = setup_mock_server().await;

    let users_response = serde_json::json!([
        {
            "user_id": "auth0|123456789",
            "email": "test@example.com"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users"))
        .and(query_param("page", "0"))
        .and(query_param("per_page", "10"))
        .and(query_param("q", "email:test@example.com"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&users_response))
        .mount(&server)
        .await;

    let params = ListUsersParams {
        page: Some(0),
        per_page: Some(10),
        q: Some("email:test@example.com".to_string()),
        ..Default::default()
    };

    let users = client
        .users()
        .list(Some(params))
        .await
        .expect("Failed to list users with params");

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].user_id, "auth0|123456789");
}

#[tokio::test]
async fn test_get_user_by_id() {
    let (server, client) = setup_mock_server().await;

    let user_response = serde_json::json!({
        "user_id": "auth0|123456789",
        "email": "test@example.com",
        "email_verified": true,
        "name": "Test User",
        "nickname": "test",
        "picture": "https://example.com/avatar.png",
        "given_name": "Test",
        "family_name": "User",
        "identities": [
            {
                "connection": "Username-Password-Authentication",
                "user_id": "123456789",
                "provider": "auth0",
                "isSocial": false
            }
        ],
        "last_ip": "192.168.1.1",
        "last_login": "2023-11-15T10:00:00.000Z",
        "logins_count": 10
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/users/auth0%7C123456789"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&user_response))
        .mount(&server)
        .await;

    let user = client
        .users()
        .get(UserId::new("auth0|123456789"))
        .await
        .expect("Failed to get user");

    assert_eq!(user.user_id, "auth0|123456789");
    assert_eq!(user.email, Some("test@example.com".to_string()));
    assert_eq!(user.given_name, Some("Test".to_string()));
    assert_eq!(user.family_name, Some("User".to_string()));
    assert_eq!(user.logins_count, Some(10));

    let identities = user.identities.expect("Expected identities");
    assert_eq!(identities.len(), 1);
    assert_eq!(identities[0].provider, "auth0");
    assert!(!identities[0].is_social);
}

#[tokio::test]
async fn test_get_user_not_found() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users/auth0%7Cnonexistent"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "statusCode": 404,
            "error": "Not Found",
            "message": "The user does not exist."
        })))
        .mount(&server)
        .await;

    let result = client.users().get(UserId::new("auth0|nonexistent")).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_user() {
    let (server, client) = setup_mock_server().await;

    let user_response = serde_json::json!({
        "user_id": "auth0|new_user_id",
        "email": "newuser@example.com",
        "email_verified": false,
        "name": "New User",
        "connection": "Username-Password-Authentication"
    });

    let request = CreateUserRequest {
        connection: "Username-Password-Authentication".to_string(),
        email: Some("newuser@example.com".to_string()),
        password: Some("SecurePassword123!".to_string()),
        name: Some("New User".to_string()),
        ..Default::default()
    };

    Mock::given(method("POST"))
        .and(path("/api/v2/users"))
        .and(bearer_token("test_token"))
        .and(body_json(serde_json::json!({
            "connection": "Username-Password-Authentication",
            "email": "newuser@example.com",
            "password": "SecurePassword123!",
            "name": "New User"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(&user_response))
        .mount(&server)
        .await;

    let user = client
        .users()
        .create(request)
        .await
        .expect("Failed to create user");

    assert_eq!(user.user_id, "auth0|new_user_id");
    assert_eq!(user.email, Some("newuser@example.com".to_string()));
}

#[tokio::test]
async fn test_create_user_conflict() {
    let (server, client) = setup_mock_server().await;

    let request = CreateUserRequest {
        connection: "Username-Password-Authentication".to_string(),
        email: Some("existing@example.com".to_string()),
        password: Some("SecurePassword123!".to_string()),
        ..Default::default()
    };

    Mock::given(method("POST"))
        .and(path("/api/v2/users"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
            "statusCode": 409,
            "error": "Conflict",
            "message": "The user already exists."
        })))
        .mount(&server)
        .await;

    let result = client.users().create(request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_user() {
    let (server, client) = setup_mock_server().await;

    let user_response = serde_json::json!({
        "user_id": "auth0|123456789",
        "email": "updated@example.com",
        "name": "Updated User",
        "blocked": false
    });

    let request = UpdateUserRequest {
        email: Some("updated@example.com".to_string()),
        name: Some("Updated User".to_string()),
        blocked: Some(false),
        ..Default::default()
    };

    Mock::given(method("PATCH"))
        .and(path("/api/v2/users/auth0%7C123456789"))
        .and(bearer_token("test_token"))
        .and(body_json(serde_json::json!({
            "email": "updated@example.com",
            "name": "Updated User",
            "blocked": false
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(&user_response))
        .mount(&server)
        .await;

    let user = client
        .users()
        .update(UserId::new("auth0|123456789"), request)
        .await
        .expect("Failed to update user");

    assert_eq!(user.email, Some("updated@example.com".to_string()));
    assert_eq!(user.name, Some("Updated User".to_string()));
}

#[tokio::test]
async fn test_delete_user() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v2/users/auth0%7C123456789"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let result = client.users().delete(UserId::new("auth0|123456789")).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_user_not_found() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v2/users/auth0%7Cnonexistent"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "statusCode": 404,
            "error": "Not Found",
            "message": "The user does not exist."
        })))
        .mount(&server)
        .await;

    let result = client.users().delete(UserId::new("auth0|nonexistent")).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_users_by_email() {
    let (server, client) = setup_mock_server().await;

    let users_response = serde_json::json!([
        {
            "user_id": "auth0|123456789",
            "email": "test@example.com",
            "email_verified": true
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users-by-email"))
        .and(query_param("email", "test@example.com"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&users_response))
        .mount(&server)
        .await;

    let users = client
        .users()
        .get_by_email("test@example.com")
        .await
        .expect("Failed to get users by email");

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].user_id, "auth0|123456789");
    assert_eq!(users[0].email, Some("test@example.com".to_string()));
}

#[tokio::test]
async fn test_get_users_by_email_not_found() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users-by-email"))
        .and(query_param("email", "nonexistent@example.com"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .mount(&server)
        .await;

    let users = client
        .users()
        .get_by_email("nonexistent@example.com")
        .await
        .expect("Failed to get users by email");

    assert!(users.is_empty());
}

#[tokio::test]
async fn test_list_users_unauthorized() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "invalid_token",
            "expires_in": 86400,
            "token_type": "Bearer"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users"))
        .and(bearer_token("invalid_token"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "statusCode": 401,
            "error": "Unauthorized",
            "message": "Invalid token"
        })))
        .mount(&server)
        .await;

    let client = ManagementClient::builder()
        .domain(server.uri())
        .client_id("test_client_id")
        .client_secret("test_client_secret")
        .build()
        .expect("Failed to build client");

    let result = client.users().list(None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_users_rate_limited() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(429).set_body_json(serde_json::json!({
            "statusCode": 429,
            "error": "Too Many Requests",
            "message": "Rate limit exceeded"
        })))
        .mount(&server)
        .await;

    let result = client.users().list(None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_user_logs() {
    let (server, client) = setup_mock_server().await;

    let logs_response = serde_json::json!([
        {
            "log_id": "log_123",
            "type": "s",
            "date": "2023-11-15T10:00:00.000Z",
            "user_id": "auth0|123456789",
            "user_name": "test@example.com",
            "ip": "192.168.1.1",
            "client_id": "client_abc",
            "client_name": "My App",
            "description": "Successful login"
        },
        {
            "log_id": "log_124",
            "type": "f",
            "date": "2023-11-14T09:00:00.000Z",
            "user_id": "auth0|123456789",
            "description": "Failed login"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/auth0%7C123456789/logs"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&logs_response))
        .mount(&server)
        .await;

    let logs = client
        .users()
        .get_logs(UserId::new("auth0|123456789"), None)
        .await
        .expect("Failed to get user logs");

    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0].log_id, "log_123");
    assert_eq!(logs[0].event_type, "s");
    assert_eq!(logs[0].description, Some("Successful login".to_string()));
    assert_eq!(logs[1].log_id, "log_124");
    assert_eq!(logs[1].event_type, "f");
}

#[tokio::test]
async fn test_get_user_logs_with_params() {
    let (server, client) = setup_mock_server().await;

    let logs_response = serde_json::json!([
        {
            "log_id": "log_123",
            "type": "s",
            "date": "2023-11-15T10:00:00.000Z",
            "user_id": "auth0|123456789"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/users/auth0%7C123456789/logs"))
        .and(query_param("page", "0"))
        .and(query_param("per_page", "10"))
        .and(query_param("sort", "date:-1"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&logs_response))
        .mount(&server)
        .await;

    let params = GetUserLogsParams {
        page: Some(0),
        per_page: Some(10),
        sort: Some("date:-1".to_string()),
        ..Default::default()
    };

    let logs = client
        .users()
        .get_logs(UserId::new("auth0|123456789"), Some(params))
        .await
        .expect("Failed to get user logs with params");

    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].log_id, "log_123");
}

#[tokio::test]
async fn test_get_user_logs_empty() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users/auth0%7C123456789/logs"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .mount(&server)
        .await;

    let logs = client
        .users()
        .get_logs(UserId::new("auth0|123456789"), None)
        .await
        .expect("Failed to get user logs");

    assert!(logs.is_empty());
}

#[tokio::test]
async fn test_get_user_logs_not_found() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/users/auth0%7Cnonexistent/logs"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "statusCode": 404,
            "error": "Not Found",
            "message": "The user does not exist."
        })))
        .mount(&server)
        .await;

    let result = client.users().get_logs(UserId::new("auth0|nonexistent"), None).await;

    assert!(result.is_err());
}
