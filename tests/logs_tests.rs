use auth0_mgmt_api::{ListLogsParams, ManagementClient};
use wiremock::matchers::{bearer_token, method, path, query_param};
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
async fn test_list_logs() {
    let (server, client) = setup_mock_server().await;

    let logs_response = serde_json::json!([
        {
            "log_id": "90020211201185132572079310688835925971249535794074878050",
            "type": "s",
            "date": "2023-11-15T10:30:00.000Z",
            "client_id": "test_client",
            "client_name": "Test Application",
            "ip": "192.168.1.1",
            "user_id": "auth0|123456789",
            "user_name": "test@example.com",
            "connection": "Username-Password-Authentication",
            "description": "Successful login"
        },
        {
            "log_id": "90020211201185132572079310688835925971249535794074878051",
            "type": "f",
            "date": "2023-11-15T10:31:00.000Z",
            "client_id": "test_client",
            "ip": "192.168.1.2",
            "description": "Failed login"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/logs"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&logs_response))
        .mount(&server)
        .await;

    let logs = client.logs().list(None).await.expect("Failed to list logs");

    assert_eq!(logs.len(), 2);
    assert_eq!(
        logs[0].log_id,
        "90020211201185132572079310688835925971249535794074878050"
    );
    assert_eq!(logs[0].event_type, "s");
    assert_eq!(logs[0].user_name, Some("test@example.com".to_string()));
    assert_eq!(logs[1].event_type, "f");
}

#[tokio::test]
async fn test_list_logs_with_params() {
    let (server, client) = setup_mock_server().await;

    let logs_response = serde_json::json!([
        {
            "log_id": "test_log_id",
            "type": "s",
            "date": "2023-11-15T10:30:00.000Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/logs"))
        .and(query_param("page", "0"))
        .and(query_param("per_page", "10"))
        .and(query_param("q", "type:s"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&logs_response))
        .mount(&server)
        .await;

    let params = ListLogsParams {
        page: Some(0),
        per_page: Some(10),
        q: Some("type:s".to_string()),
        ..Default::default()
    };

    let logs = client
        .logs()
        .list(Some(params))
        .await
        .expect("Failed to list logs with params");

    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].log_id, "test_log_id");
}

#[tokio::test]
async fn test_list_logs_with_checkpoint() {
    let (server, client) = setup_mock_server().await;

    let logs_response = serde_json::json!([
        {
            "log_id": "newer_log_id",
            "type": "s"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v2/logs"))
        .and(query_param("from", "starting_log_id"))
        .and(query_param("take", "100"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&logs_response))
        .mount(&server)
        .await;

    let params = ListLogsParams {
        from: Some("starting_log_id".to_string()),
        take: Some(100),
        ..Default::default()
    };

    let logs = client
        .logs()
        .list(Some(params))
        .await
        .expect("Failed to list logs with checkpoint");

    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].log_id, "newer_log_id");
}

#[tokio::test]
async fn test_get_log_by_id() {
    let (server, client) = setup_mock_server().await;

    let log_response = serde_json::json!({
        "log_id": "specific_log_id",
        "type": "s",
        "date": "2023-11-15T10:30:00.000Z",
        "client_id": "test_client",
        "client_name": "Test Application",
        "ip": "192.168.1.1",
        "user_agent": "Mozilla/5.0",
        "user_id": "auth0|123456789",
        "user_name": "test@example.com",
        "connection": "Username-Password-Authentication",
        "connection_id": "con_123",
        "description": "Successful login",
        "hostname": "test.auth0.com",
        "location_info": {
            "country_code": "US",
            "country_name": "United States",
            "city_name": "San Francisco",
            "latitude": 37.7749,
            "longitude": -122.4194,
            "time_zone": "America/Los_Angeles"
        }
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/logs/specific_log_id"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&log_response))
        .mount(&server)
        .await;

    let log = client
        .logs()
        .get("specific_log_id")
        .await
        .expect("Failed to get log");

    assert_eq!(log.log_id, "specific_log_id");
    assert_eq!(log.event_type, "s");
    assert_eq!(log.user_name, Some("test@example.com".to_string()));
    assert_eq!(log.connection_id, Some("con_123".to_string()));

    let location = log.location_info.expect("Expected location_info");
    assert_eq!(location.country_code, Some("US".to_string()));
    assert_eq!(location.city_name, Some("San Francisco".to_string()));
}

#[tokio::test]
async fn test_get_log_with_url_encoded_id() {
    let (server, client) = setup_mock_server().await;

    let log_response = serde_json::json!({
        "log_id": "log/with/slashes",
        "type": "s"
    });

    Mock::given(method("GET"))
        .and(path("/api/v2/logs/log%2Fwith%2Fslashes"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&log_response))
        .mount(&server)
        .await;

    let log = client
        .logs()
        .get("log/with/slashes")
        .await
        .expect("Failed to get log with special characters");

    assert_eq!(log.log_id, "log/with/slashes");
}
