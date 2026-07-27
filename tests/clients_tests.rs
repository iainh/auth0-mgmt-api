use auth0_mgmt_api::{
    AppType, ClientCredentialAlgorithm, ClientCredentialId, ClientCredentialType, ClientId,
    CreateClientCredentialRequest, CreateClientRequest, ListClientConnectionsParams,
    ListClientsParams, ManagementClient, Patch, UpdateClientCredentialRequest, UpdateClientRequest,
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
async fn test_list_clients() {
    let (server, client) = setup_mock_server().await;

    let clients_response = serde_json::json!([
        {
            "client_id": "client_123",
            "name": "Test Application",
            "description": "A test application",
            "app_type": "spa",
            "is_first_party": true,
            "oidc_conformant": true,
            "callbacks": ["https://example.com/callback"],
            "grant_types": ["authorization_code", "refresh_token"]
        },
        {
            "client_id": "client_456",
            "name": "Another Application",
            "app_type": "regular_web",
            "is_first_party": false
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/clients"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&clients_response))
        .mount(&server)
        .await;

    let clients = client
        .clients()
        .list(None)
        .await
        .expect("Failed to list clients");

    assert_eq!(clients.len(), 2);
    assert_eq!(clients[0].client_id, "client_123");
    assert_eq!(clients[0].name, Some("Test Application".to_string()));
    assert_eq!(clients[0].app_type, Some(AppType::Spa));
    assert_eq!(clients[0].is_first_party, Some(true));
    assert_eq!(clients[1].client_id, "client_456");
    assert_eq!(clients[1].is_first_party, Some(false));
}

#[tokio::test]
async fn test_list_clients_with_params() {
    let (server, client) = setup_mock_server().await;

    let clients_response = serde_json::json!([
        {
            "client_id": "client_123",
            "name": "Test SPA",
            "app_type": "spa"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/clients"))
        .and(query_param("page", "0"))
        .and(query_param("per_page", "10"))
        .and(query_param("app_type", "spa"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&clients_response))
        .mount(&server)
        .await;

    let params = ListClientsParams {
        page: Some(0),
        per_page: Some(10),
        app_type: Some(AppType::Spa),
        ..Default::default()
    };

    let clients = client
        .clients()
        .list(Some(params))
        .await
        .expect("Failed to list clients with params");

    assert_eq!(clients.len(), 1);
    assert_eq!(clients[0].app_type, Some(AppType::Spa));
}

#[tokio::test]
async fn test_list_clients_first_party_filter() {
    let (server, client) = setup_mock_server().await;

    let clients_response = serde_json::json!([
        {
            "client_id": "client_123",
            "name": "First Party App",
            "is_first_party": true
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/clients"))
        .and(query_param("is_first_party", "true"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&clients_response))
        .mount(&server)
        .await;

    let params = ListClientsParams {
        is_first_party: Some(true),
        ..Default::default()
    };

    let clients = client
        .clients()
        .list(Some(params))
        .await
        .expect("Failed to list first party clients");

    assert_eq!(clients.len(), 1);
    assert_eq!(clients[0].is_first_party, Some(true));
}

#[tokio::test]
async fn test_get_client_by_id() {
    let (server, client) = setup_mock_server().await;

    let client_response = serde_json::json!({
        "client_id": "client_123",
        "tenant": "test-tenant",
        "name": "Test Application",
        "description": "A test application",
        "global": false,
        "client_secret": "secret_value",
        "app_type": "spa",
        "logo_uri": "https://example.com/logo.png",
        "is_first_party": true,
        "oidc_conformant": true,
        "callbacks": ["https://example.com/callback"],
        "allowed_origins": ["https://example.com"],
        "web_origins": ["https://example.com"],
        "allowed_logout_urls": ["https://example.com/logout"],
        "grant_types": ["authorization_code", "refresh_token"],
        "token_endpoint_auth_method": "client_secret_post",
        "sso": true,
        "cross_origin_auth": false
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/clients/client_123"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&client_response))
        .mount(&server)
        .await;

    let app = client
        .clients()
        .get(ClientId::new("client_123"))
        .await
        .expect("Failed to get client");

    assert_eq!(app.client_id, "client_123");
    assert_eq!(app.name, Some("Test Application".to_string()));
    assert_eq!(app.tenant, Some("test-tenant".to_string()));
    assert_eq!(app.client_secret, Some("secret_value".to_string()));
    assert_eq!(app.sso, Some(true));

    let callbacks = app.callbacks.expect("Expected callbacks");
    assert_eq!(callbacks, vec!["https://example.com/callback"]);
}

#[tokio::test]
async fn test_get_client_not_found() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/clients/nonexistent_client"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "statusCode": 404,
            "error": "Not Found",
            "message": "The client does not exist."
        })))
        .mount(&server)
        .await;

    let result = client
        .clients()
        .get(ClientId::new("nonexistent_client"))
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_client() {
    let (server, client) = setup_mock_server().await;

    let client_response = serde_json::json!({
        "client_id": "new_client_id",
        "name": "New Application",
        "app_type": "spa",
        "oidc_conformant": true,
        "callbacks": ["https://newapp.example.com/callback"]
    });

    let request = CreateClientRequest {
        name: "New Application".to_string(),
        app_type: Some(AppType::Spa),
        oidc_conformant: Some(true),
        callbacks: Some(vec!["https://newapp.example.com/callback".to_string()]),
        ..Default::default()
    };

    Mock::given(method("POST"))
        .and(path("/api/v2/clients"))
        .and(bearer_token("test_token"))
        .and(body_json(serde_json::json!({
            "name": "New Application",
            "app_type": "spa",
            "oidc_conformant": true,
            "callbacks": ["https://newapp.example.com/callback"]
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(&client_response))
        .mount(&server)
        .await;

    let app = client
        .clients()
        .create(request)
        .await
        .expect("Failed to create client");

    assert_eq!(app.client_id, "new_client_id");
    assert_eq!(app.name, Some("New Application".to_string()));
    assert_eq!(app.app_type, Some(AppType::Spa));
}

#[tokio::test]
async fn test_create_client_validation_error() {
    let (server, client) = setup_mock_server().await;

    let request = CreateClientRequest {
        name: "".to_string(),
        ..Default::default()
    };

    Mock::given(method("POST"))
        .and(path("/api/v2/clients"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "statusCode": 400,
            "error": "Bad Request",
            "message": "Payload validation error: 'name' is required"
        })))
        .mount(&server)
        .await;

    let result = client.clients().create(request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_client() {
    let (server, client) = setup_mock_server().await;

    let client_response = serde_json::json!({
        "client_id": "client_123",
        "name": "Updated Application",
        "description": "Updated description",
        "callbacks": ["https://updated.example.com/callback"]
    });

    let request = UpdateClientRequest {
        name: Some("Updated Application".to_string()),
        description: Some("Updated description".to_string()),
        callbacks: Some(vec!["https://updated.example.com/callback".to_string()]),
        ..Default::default()
    };

    Mock::given(method("PATCH"))
        .and(path("/api/v2/clients/client_123"))
        .and(bearer_token("test_token"))
        .and(body_json(serde_json::json!({
            "name": "Updated Application",
            "description": "Updated description",
            "callbacks": ["https://updated.example.com/callback"]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(&client_response))
        .mount(&server)
        .await;

    let app = client
        .clients()
        .update(ClientId::new("client_123"), request)
        .await
        .expect("Failed to update client");

    assert_eq!(app.name, Some("Updated Application".to_string()));
    assert_eq!(app.description, Some("Updated description".to_string()));
}

#[tokio::test]
async fn test_delete_client() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v2/clients/client_123"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let result = client.clients().delete(ClientId::new("client_123")).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_client_not_found() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v2/clients/nonexistent"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "statusCode": 404,
            "error": "Not Found",
            "message": "The client does not exist."
        })))
        .mount(&server)
        .await;

    let result = client.clients().delete(ClientId::new("nonexistent")).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_rotate_client_secret() {
    let (server, client) = setup_mock_server().await;

    let client_response = serde_json::json!({
        "client_id": "client_123",
        "name": "Test Application",
        "client_secret": "new_rotated_secret"
    });

    Mock::given(method("POST"))
        .and(path("/api/v2/clients/client_123/rotate-secret"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&client_response))
        .mount(&server)
        .await;

    let app = client
        .clients()
        .rotate_secret(ClientId::new("client_123"))
        .await
        .expect("Failed to rotate client secret");

    assert_eq!(app.client_id, "client_123");
    assert_eq!(app.client_secret, Some("new_rotated_secret".to_string()));
}

#[tokio::test]
async fn test_rotate_client_secret_not_found() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/api/v2/clients/nonexistent/rotate-secret"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "statusCode": 404,
            "error": "Not Found",
            "message": "The client does not exist."
        })))
        .mount(&server)
        .await;

    let result = client
        .clients()
        .rotate_secret(ClientId::new("nonexistent"))
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_clients_unauthorized() {
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
        .and(path("/api/v2/clients"))
        .and(bearer_token("invalid_token"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "statusCode": 401,
            "error": "Unauthorized",
            "message": "Invalid token"
        })))
        .mount(&server)
        .await;

    let mgmt_client = ManagementClient::builder()
        .domain(server.uri())
        .client_id("test_client_id")
        .client_secret("test_client_secret")
        .build()
        .expect("Failed to build client");

    let result = mgmt_client.clients().list(None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_clients_rate_limited() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/clients"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(429).set_body_json(serde_json::json!({
            "statusCode": 429,
            "error": "Too Many Requests",
            "message": "Rate limit exceeded"
        })))
        .mount(&server)
        .await;

    let result = client.clients().list(None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_client_with_special_characters_in_id() {
    let (server, client) = setup_mock_server().await;

    let client_response = serde_json::json!({
        "client_id": "client/with/slashes",
        "name": "Special Client"
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/clients/client%2Fwith%2Fslashes"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&client_response))
        .mount(&server)
        .await;

    let app = client
        .clients()
        .get(ClientId::new("client/with/slashes"))
        .await
        .expect("Failed to get client with special characters");

    assert_eq!(app.client_id, "client/with/slashes");
}

#[tokio::test]
async fn test_list_client_credentials() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/clients/client_123/credentials"))
        .and(bearer_token("test_token"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!([{
                "id": "cred_123",
                "name": "Production signing key",
                "kid": "key-id-12345",
                "alg": "RS256",
                "credential_type": "public_key",
                "created_at": "2026-07-27T12:00:00.000Z"
            }])),
        )
        .mount(&server)
        .await;

    let credentials = client
        .clients()
        .list_credentials(ClientId::new("client_123"))
        .await
        .expect("Failed to list client credentials");

    assert_eq!(credentials.len(), 1);
    assert_eq!(credentials[0].id.as_deref(), Some("cred_123"));
    assert_eq!(
        credentials[0].credential_type,
        Some(ClientCredentialType::PublicKey)
    );
}

#[tokio::test]
async fn test_get_client_credential_with_encoded_ids() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/clients/client%2F123/credentials/cred%2F456"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "cred/456",
            "alg": "PS256",
            "credential_type": "x509_cert",
            "thumbprint_sha256": "thumbprint"
        })))
        .mount(&server)
        .await;

    let credential = client
        .clients()
        .get_credential(
            ClientId::new("client/123"),
            ClientCredentialId::new("cred/456"),
        )
        .await
        .expect("Failed to get client credential");

    assert_eq!(credential.id.as_deref(), Some("cred/456"));
    assert_eq!(credential.alg, Some(ClientCredentialAlgorithm::PS256));
}

#[tokio::test]
async fn test_create_client_credential() {
    let (server, client) = setup_mock_server().await;
    let pem = "-----BEGIN PUBLIC KEY-----\nkey-data\n-----END PUBLIC KEY-----";

    Mock::given(method("POST"))
        .and(path("/api/v2/clients/client_123/credentials"))
        .and(bearer_token("test_token"))
        .and(body_json(serde_json::json!({
            "credential_type": "public_key",
            "name": "Production signing key",
            "pem": pem,
            "alg": "RS384",
            "parse_expiry_from_cert": false,
            "expires_at": "2027-07-27T12:00:00.000Z",
            "kid": "key-id-12345"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "cred_new",
            "name": "Production signing key",
            "kid": "key-id-12345",
            "alg": "RS384",
            "credential_type": "public_key",
            "expires_at": "2027-07-27T12:00:00.000Z"
        })))
        .mount(&server)
        .await;

    let credential = client
        .clients()
        .create_credential(
            ClientId::new("client_123"),
            CreateClientCredentialRequest {
                credential_type: ClientCredentialType::PublicKey,
                name: Some("Production signing key".to_string()),
                subject_dn: None,
                pem: Some(pem.to_string()),
                alg: Some(ClientCredentialAlgorithm::RS384),
                parse_expiry_from_cert: Some(false),
                expires_at: Some("2027-07-27T12:00:00.000Z".to_string()),
                kid: Some("key-id-12345".to_string()),
            },
        )
        .await
        .expect("Failed to create client credential");

    assert_eq!(credential.id.as_deref(), Some("cred_new"));
}

#[tokio::test]
async fn test_update_client_credential_can_clear_expiration() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("PATCH"))
        .and(path("/api/v2/clients/client_123/credentials/cred_123"))
        .and(bearer_token("test_token"))
        .and(body_json(serde_json::json!({ "expires_at": null })))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "cred_123",
            "alg": "RS256",
            "credential_type": "public_key"
        })))
        .mount(&server)
        .await;

    let credential = client
        .clients()
        .update_credential(
            ClientId::new("client_123"),
            ClientCredentialId::new("cred_123"),
            UpdateClientCredentialRequest {
                expires_at: Patch::Null,
            },
        )
        .await
        .expect("Failed to update client credential");

    assert_eq!(credential.id.as_deref(), Some("cred_123"));
    assert_eq!(credential.expires_at, None);
}

#[tokio::test]
async fn test_delete_client_credential() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/api/v2/clients/client_123/credentials/cred_123"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    client
        .clients()
        .delete_credential(
            ClientId::new("client_123"),
            ClientCredentialId::new("cred_123"),
        )
        .await
        .expect("Failed to delete client credential");
}

#[tokio::test]
async fn test_list_connections_enabled_for_client() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/clients/client_123/connections"))
        .and(query_param("strategy", "auth0"))
        .and(query_param("strategy", "google-oauth2"))
        .and(query_param("from", "checkpoint/token"))
        .and(query_param("take", "25"))
        .and(query_param("fields", "id,name,strategy"))
        .and(query_param("include_fields", "true"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "connections": [{
                "id": "con_123",
                "name": "Username-Password-Authentication",
                "strategy": "auth0",
                "authentication": { "active": true }
            }],
            "next": "next-checkpoint"
        })))
        .mount(&server)
        .await;

    let page = client
        .clients()
        .list_connections(
            ClientId::new("client_123"),
            Some(ListClientConnectionsParams {
                strategy: Some(vec!["auth0".to_string(), "google-oauth2".to_string()]),
                from: Some("checkpoint/token".to_string()),
                take: Some(25),
                fields: Some("id,name,strategy".to_string()),
                include_fields: Some(true),
            }),
        )
        .await
        .expect("Failed to list enabled connections");

    assert_eq!(page.connections.len(), 1);
    assert_eq!(page.connections[0].id.as_deref(), Some("con_123"));
    assert_eq!(page.next.as_deref(), Some("next-checkpoint"));
}
