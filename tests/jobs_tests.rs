use std::time::Duration;

use auth0_mgmt_api::{
    Auth0Error, ClientId, ConnectionId, ExportUsersField, ExportUsersRequest, ImportUsersRequest,
    JobErrors, JobFileFormat, JobId, JobStatus, JobType, ManagementClient, UserId,
    VerificationEmailRequest,
};
use wiremock::matchers::{
    bearer_token, body_json, body_string_contains, header_regex, method, path,
};
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
async fn test_export_users() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/api/v2/jobs/users-exports"))
        .and(bearer_token("test_token"))
        .and(body_json(serde_json::json!({
            "connection_id": "con_123",
            "format": "csv",
            "limit": 100,
            "fields": [{ "name": "email", "export_as": "Email Address" }]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "job_export",
            "type": "users_export",
            "status": "pending",
            "connection_id": "con_123",
            "format": "csv"
        })))
        .mount(&server)
        .await;

    let job = client
        .jobs()
        .export_users(ExportUsersRequest {
            connection_id: Some(ConnectionId::new("con_123")),
            format: Some(JobFileFormat::Csv),
            limit: Some(100),
            fields: Some(vec![ExportUsersField {
                name: "email".to_string(),
                export_as: Some("Email Address".to_string()),
            }]),
        })
        .await
        .expect("Failed to create user export job");

    assert_eq!(job.id, "job_export");
    assert_eq!(job.job_type, JobType::UsersExport);
    assert_eq!(job.format, Some(JobFileFormat::Csv));
}

#[tokio::test]
async fn test_import_users_as_multipart() {
    let (server, client) = setup_mock_server().await;
    let users = r#"[{"email":"user@example.com"}]"#;

    Mock::given(method("POST"))
        .and(path("/api/v2/jobs/users-imports"))
        .and(bearer_token("test_token"))
        .and(header_regex(
            "content-type",
            "^multipart/form-data; boundary=.+",
        ))
        .and(body_string_contains("name=\"users\""))
        .and(body_string_contains("filename=\"users.json\""))
        .and(body_string_contains(users))
        .and(body_string_contains("name=\"connection_id\""))
        .and(body_string_contains("con_123"))
        .and(body_string_contains("name=\"upsert\""))
        .and(body_string_contains("true"))
        .and(body_string_contains("name=\"external_id\""))
        .and(body_string_contains("migration-42"))
        .and(body_string_contains("name=\"send_completion_email\""))
        .and(body_string_contains("false"))
        .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({
            "id": "job_import",
            "type": "users_import",
            "status": "pending",
            "created_at": "2026-07-27T12:00:00.000Z",
            "connection_id": "con_123",
            "external_id": "migration-42"
        })))
        .mount(&server)
        .await;

    let job = client
        .jobs()
        .import_users(ImportUsersRequest {
            users: users.as_bytes().to_vec(),
            connection_id: ConnectionId::new("con_123"),
            upsert: Some(true),
            external_id: Some("migration-42".to_string()),
            send_completion_email: Some(false),
        })
        .await
        .expect("Failed to create user import job");

    assert_eq!(job.id, "job_import");
    assert_eq!(job.external_id.as_deref(), Some("migration-42"));
}

#[tokio::test]
async fn test_send_verification_email() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/api/v2/jobs/verification-email"))
        .and(bearer_token("test_token"))
        .and(body_json(serde_json::json!({
            "user_id": "auth0|123",
            "client_id": "client_123",
            "organization_id": "org_123"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "job_verify",
            "type": "verification_email",
            "status": "completed"
        })))
        .mount(&server)
        .await;

    let job = client
        .jobs()
        .send_verification_email(VerificationEmailRequest {
            user_id: UserId::new("auth0|123"),
            client_id: Some(ClientId::new("client_123")),
            identity: None,
            organization_id: Some("org_123".to_string()),
        })
        .await
        .expect("Failed to send verification email");

    assert_eq!(job.status, JobStatus::Completed);
}

#[tokio::test]
async fn test_get_job() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/jobs/job%2F123"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "job/123",
            "type": "users_import",
            "status": "completed",
            "percentage_done": 100,
            "summary": { "failed": 1, "updated": 2, "inserted": 3, "total": 6 }
        })))
        .mount(&server)
        .await;

    let job = client
        .jobs()
        .get(JobId::new("job/123"))
        .await
        .expect("Failed to get job");

    assert_eq!(job.percentage_done, Some(100));
    assert_eq!(job.summary.expect("Expected summary").failed, Some(1));
}

#[tokio::test]
async fn test_get_import_job_errors() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/jobs/job_import/errors"))
        .and(bearer_token("test_token"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!([{
                "user": { "email": "invalid" },
                "errors": [{ "code": "INVALID_EMAIL", "message": "Invalid email", "path": "email" }]
            }])),
        )
        .mount(&server)
        .await;

    let errors = client
        .jobs()
        .get_errors(JobId::new("job_import"))
        .await
        .expect("Failed to get job errors")
        .expect("Expected job errors");

    match errors {
        JobErrors::Import(errors) => {
            assert_eq!(errors.len(), 1);
            assert_eq!(
                errors[0].errors.as_ref().expect("Expected errors")[0]
                    .code
                    .as_deref(),
                Some("INVALID_EMAIL")
            );
        }
        JobErrors::Generic(_) => panic!("Expected import errors"),
    }
}

#[tokio::test]
async fn test_get_generic_job_error() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/jobs/job_failed/errors"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "job_failed",
            "type": "users_import",
            "status": "failed",
            "connection_id": "con_123",
            "status_details": "The import file could not be processed"
        })))
        .mount(&server)
        .await;

    let errors = client
        .jobs()
        .get_errors(JobId::new("job_failed"))
        .await
        .expect("Failed to get job error")
        .expect("Expected job error");

    match errors {
        JobErrors::Generic(error) => {
            assert_eq!(error.status, JobStatus::Failed);
            assert_eq!(
                error.status_details.as_deref(),
                Some("The import file could not be processed")
            );
        }
        JobErrors::Import(_) => panic!("Expected generic job error"),
    }
}

#[tokio::test]
async fn test_get_job_errors_with_no_content() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/jobs/job_clean/errors"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let errors = client
        .jobs()
        .get_errors(JobId::new("job_clean"))
        .await
        .expect("Failed to get job errors");

    assert!(errors.is_none());
}

#[tokio::test]
async fn test_wait_for_completion_polls_until_finished() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/jobs/job_export"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "job_export",
            "type": "users_export",
            "status": "processing",
            "percentage_done": 50
        })))
        .up_to_n_times(2)
        .expect(2)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v2/jobs/job_export"))
        .and(bearer_token("test_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "job_export",
            "type": "users_export",
            "status": "completed",
            "location": "https://example.com/export.json.gz"
        })))
        // One call from the poller and one from the follow-up get below.
        .expect(2)
        .mount(&server)
        .await;

    let created = JobId::new("job_export");
    let job = client
        .jobs()
        .wait_for_completion(created, Duration::from_millis(5), Duration::from_secs(5))
        .await
        .expect("Failed to wait for job");

    assert_eq!(job.status, JobStatus::Completed);
    assert_eq!(
        job.location.as_deref(),
        Some("https://example.com/export.json.gz")
    );

    // The returned ID can be passed straight back to the API.
    let again = client.jobs().get(job.id).await;
    assert!(again.is_ok());
}

#[tokio::test]
async fn test_wait_for_completion_returns_failed_jobs() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/jobs/job_failed"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "job_failed",
            "type": "users_import",
            "status": "failed"
        })))
        .mount(&server)
        .await;

    let job = client
        .jobs()
        .wait_for_completion(
            JobId::new("job_failed"),
            Duration::from_millis(5),
            Duration::from_secs(5),
        )
        .await
        .expect("A failed job is still a finished job");

    assert_eq!(job.status, JobStatus::Failed);
    assert_eq!(job.job_type, JobType::UsersImport);
}

#[tokio::test]
async fn test_wait_for_completion_times_out() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/jobs/job_slow"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "job_slow",
            "type": "users_import",
            "status": "some_new_status"
        })))
        .mount(&server)
        .await;

    let error = client
        .jobs()
        .wait_for_completion(
            JobId::new("job_slow"),
            Duration::from_millis(5),
            Duration::from_millis(30),
        )
        .await
        .expect_err("Expected a timeout");

    match error {
        Auth0Error::JobTimeout {
            job_id,
            last_status,
        } => {
            assert_eq!(job_id, "job_slow");
            assert_eq!(last_status, "some_new_status");
        }
        other => panic!("Expected JobTimeout, got {other:?}"),
    }
}
