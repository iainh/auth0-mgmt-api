use auth0_mgmt_api::{
    CreateConnectionRequest, ListConnectionsParams, ManagementClient, UpdateConnectionRequest,
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
async fn test_list_connections() {
    let (server, client) = setup_mock_server().await;

    let connections_response = serde_json::json!([
        {
            "id": "con_123",
            "name": "Username-Password-Authentication",
            "display_name": "Database",
            "strategy": "auth0",
            "realms": ["Username-Password-Authentication"],
            "is_domain_connection": false,
            "enabled_clients": ["client_123", "client_456"]
        },
        {
            "id": "con_456",
            "name": "google-oauth2",
            "display_name": "Google",
            "strategy": "google-oauth2",
            "is_domain_connection": false,
            "enabled_clients": ["client_123"]
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/connections"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&connections_response))
        .mount(&server)
        .await;

    let connections = client
        .connections()
        .list(None)
        .await
        .expect("Failed to list connections");

    assert_eq!(connections.len(), 2);
    assert_eq!(connections[0].id, "con_123");
    assert_eq!(connections[0].name, "Username-Password-Authentication");
    assert_eq!(connections[0].strategy, "auth0");
    assert_eq!(connections[1].id, "con_456");
    assert_eq!(connections[1].strategy, "google-oauth2");
}

#[tokio::test]
async fn test_list_connections_with_params() {
    let (server, client) = setup_mock_server().await;

    let connections_response = serde_json::json!([
        {
            "id": "con_123",
            "name": "Username-Password-Authentication",
            "strategy": "auth0"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/connections"))
        .and(query_param("page", "0"))
        .and(query_param("per_page", "10"))
        .and(query_param("strategy", "auth0"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&connections_response))
        .mount(&server)
        .await;

    let params = ListConnectionsParams {
        page: Some(0),
        per_page: Some(10),
        strategy: Some("auth0".to_string()),
        ..Default::default()
    };

    let connections = client
        .connections()
        .list(Some(params))
        .await
        .expect("Failed to list connections with params");

    assert_eq!(connections.len(), 1);
    assert_eq!(connections[0].strategy, "auth0");
}

#[tokio::test]
async fn test_list_connections_by_name() {
    let (server, client) = setup_mock_server().await;

    let connections_response = serde_json::json!([
        {
            "id": "con_456",
            "name": "google-oauth2",
            "strategy": "google-oauth2"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/connections"))
        .and(query_param("name", "google-oauth2"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&connections_response))
        .mount(&server)
        .await;

    let params = ListConnectionsParams {
        name: Some("google-oauth2".to_string()),
        ..Default::default()
    };

    let connections = client
        .connections()
        .list(Some(params))
        .await
        .expect("Failed to list connections by name");

    assert_eq!(connections.len(), 1);
    assert_eq!(connections[0].name, "google-oauth2");
}

#[tokio::test]
async fn test_get_connection_by_id() {
    let (server, client) = setup_mock_server().await;

    let connection_response = serde_json::json!({
        "id": "con_123",
        "name": "Username-Password-Authentication",
        "display_name": "Database Connection",
        "strategy": "auth0",
        "realms": ["Username-Password-Authentication"],
        "is_domain_connection": false,
        "enabled_clients": ["client_123", "client_456"],
        "metadata": {
            "custom_field": "custom_value"
        },
        "options": {
            "passwordPolicy": "good",
            "requires_username": false,
            "brute_force_protection": true
        }
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/connections/con_123"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&connection_response))
        .mount(&server)
        .await;

    let connection = client
        .connections()
        .get("con_123")
        .await
        .expect("Failed to get connection");

    assert_eq!(connection.id, "con_123");
    assert_eq!(connection.name, "Username-Password-Authentication");
    assert_eq!(
        connection.display_name,
        Some("Database Connection".to_string())
    );
    assert_eq!(connection.strategy, "auth0");
    assert_eq!(connection.is_domain_connection, Some(false));

    let enabled_clients = connection.enabled_clients.expect("Expected enabled_clients");
    assert_eq!(enabled_clients.len(), 2);

    let options = connection.options.expect("Expected options");
    assert_eq!(options["passwordPolicy"], "good");
}

#[tokio::test]
async fn test_get_connection_not_found() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/connections/con_nonexistent"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "statusCode": 404,
            "error": "Not Found",
            "message": "The connection does not exist."
        })))
        .mount(&server)
        .await;

    let result = client.connections().get("con_nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_connection() {
    let (server, client) = setup_mock_server().await;

    let connection_response = serde_json::json!({
        "id": "con_new",
        "name": "new-database-connection",
        "display_name": "New Database",
        "strategy": "auth0",
        "enabled_clients": ["client_123"]
    });

    let request = CreateConnectionRequest {
        name: "new-database-connection".to_string(),
        strategy: "auth0".to_string(),
        display_name: Some("New Database".to_string()),
        enabled_clients: Some(vec!["client_123".to_string()]),
        ..Default::default()
    };

    Mock::given(method("POST"))
        .and(path("/api/v2/connections"))
        .and(bearer_token("test_token"))
        .and(body_json(serde_json::json!({
            "name": "new-database-connection",
            "strategy": "auth0",
            "display_name": "New Database",
            "enabled_clients": ["client_123"]
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(&connection_response))
        .mount(&server)
        .await;

    let connection = client
        .connections()
        .create(request)
        .await
        .expect("Failed to create connection");

    assert_eq!(connection.id, "con_new");
    assert_eq!(connection.name, "new-database-connection");
    assert_eq!(connection.strategy, "auth0");
}

#[tokio::test]
async fn test_create_connection_with_options() {
    let (server, client) = setup_mock_server().await;

    let connection_response = serde_json::json!({
        "id": "con_new",
        "name": "custom-database",
        "strategy": "auth0",
        "options": {
            "passwordPolicy": "excellent",
            "requires_username": true
        }
    });

    let options = serde_json::json!({
        "passwordPolicy": "excellent",
        "requires_username": true
    });

    let request = CreateConnectionRequest {
        name: "custom-database".to_string(),
        strategy: "auth0".to_string(),
        options: Some(options.clone()),
        ..Default::default()
    };

    Mock::given(method("POST"))
        .and(path("/api/v2/connections"))
        .and(bearer_token("test_token"))
        .and(body_json(serde_json::json!({
            "name": "custom-database",
            "strategy": "auth0",
            "options": options
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(&connection_response))
        .mount(&server)
        .await;

    let connection = client
        .connections()
        .create(request)
        .await
        .expect("Failed to create connection with options");

    assert_eq!(connection.id, "con_new");

    let conn_options = connection.options.expect("Expected options");
    assert_eq!(conn_options["passwordPolicy"], "excellent");
}

#[tokio::test]
async fn test_create_connection_conflict() {
    let (server, client) = setup_mock_server().await;

    let request = CreateConnectionRequest {
        name: "existing-connection".to_string(),
        strategy: "auth0".to_string(),
        ..Default::default()
    };

    Mock::given(method("POST"))
        .and(path("/api/v2/connections"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
            "statusCode": 409,
            "error": "Conflict",
            "message": "A connection with the same name already exists."
        })))
        .mount(&server)
        .await;

    let result = client.connections().create(request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_connection_validation_error() {
    let (server, client) = setup_mock_server().await;

    let request = CreateConnectionRequest {
        name: "".to_string(),
        strategy: "auth0".to_string(),
        ..Default::default()
    };

    Mock::given(method("POST"))
        .and(path("/api/v2/connections"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "statusCode": 400,
            "error": "Bad Request",
            "message": "Payload validation error: 'name' is required"
        })))
        .mount(&server)
        .await;

    let result = client.connections().create(request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_connection() {
    let (server, client) = setup_mock_server().await;

    let connection_response = serde_json::json!({
        "id": "con_123",
        "name": "Username-Password-Authentication",
        "display_name": "Updated Display Name",
        "strategy": "auth0",
        "enabled_clients": ["client_123", "client_789"]
    });

    let request = UpdateConnectionRequest {
        display_name: Some("Updated Display Name".to_string()),
        enabled_clients: Some(vec!["client_123".to_string(), "client_789".to_string()]),
        ..Default::default()
    };

    Mock::given(method("PATCH"))
        .and(path("/api/v2/connections/con_123"))
        .and(bearer_token("test_token"))
        .and(body_json(serde_json::json!({
            "display_name": "Updated Display Name",
            "enabled_clients": ["client_123", "client_789"]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(&connection_response))
        .mount(&server)
        .await;

    let connection = client
        .connections()
        .update("con_123", request)
        .await
        .expect("Failed to update connection");

    assert_eq!(
        connection.display_name,
        Some("Updated Display Name".to_string())
    );

    let enabled_clients = connection.enabled_clients.expect("Expected enabled_clients");
    assert_eq!(enabled_clients.len(), 2);
}

#[tokio::test]
async fn test_update_connection_options() {
    let (server, client) = setup_mock_server().await;

    let connection_response = serde_json::json!({
        "id": "con_123",
        "name": "Username-Password-Authentication",
        "strategy": "auth0",
        "options": {
            "passwordPolicy": "excellent",
            "brute_force_protection": true
        }
    });

    let options = serde_json::json!({
        "passwordPolicy": "excellent",
        "brute_force_protection": true
    });

    let request = UpdateConnectionRequest {
        options: Some(options.clone()),
        ..Default::default()
    };

    Mock::given(method("PATCH"))
        .and(path("/api/v2/connections/con_123"))
        .and(bearer_token("test_token"))
        .and(body_json(serde_json::json!({
            "options": options
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(&connection_response))
        .mount(&server)
        .await;

    let connection = client
        .connections()
        .update("con_123", request)
        .await
        .expect("Failed to update connection options");

    let conn_options = connection.options.expect("Expected options");
    assert_eq!(conn_options["passwordPolicy"], "excellent");
}

#[tokio::test]
async fn test_delete_connection() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v2/connections/con_123"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let result = client.connections().delete("con_123").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_connection_not_found() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v2/connections/con_nonexistent"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "statusCode": 404,
            "error": "Not Found",
            "message": "The connection does not exist."
        })))
        .mount(&server)
        .await;

    let result = client.connections().delete("con_nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_connections_unauthorized() {
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
        .and(path("/api/v2/connections"))
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

    let result = client.connections().list(None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_connections_rate_limited() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/connections"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(429).set_body_json(serde_json::json!({
            "statusCode": 429,
            "error": "Too Many Requests",
            "message": "Rate limit exceeded"
        })))
        .mount(&server)
        .await;

    let result = client.connections().list(None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_connection_with_special_characters_in_id() {
    let (server, client) = setup_mock_server().await;

    let connection_response = serde_json::json!({
        "id": "con/with/slashes",
        "name": "Special Connection",
        "strategy": "auth0"
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/connections/con%2Fwith%2Fslashes"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&connection_response))
        .mount(&server)
        .await;

    let connection = client
        .connections()
        .get("con/with/slashes")
        .await
        .expect("Failed to get connection with special characters");

    assert_eq!(connection.id, "con/with/slashes");
}
