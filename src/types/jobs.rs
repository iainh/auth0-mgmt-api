use serde::{Deserialize, Serialize};

use super::{ClientId, ConnectionId, JobId, UserId};

string_enum! {
    /// File format for a user export job.
    ///
    /// See the [Auth0 Create Export Users Job documentation](https://auth0.com/docs/api/management/v2/jobs/post-users-exports).
    pub enum JobFileFormat {
        Json => "json",
        Csv => "csv",
    }
}

string_enum! {
    /// Progress state of an asynchronous job.
    ///
    /// Auth0 documents the status as a free-form string, so values this crate
    /// does not know are kept in [`JobStatus::Other`].
    pub enum JobStatus {
        Pending => "pending",
        Processing => "processing",
        Completed => "completed",
        Failed => "failed",
    }
}

impl JobStatus {
    /// Whether the job has stopped running, successfully or not.
    ///
    /// Unrecognized statuses are treated as still running.
    pub fn is_finished(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed)
    }
}

string_enum! {
    /// Kind of asynchronous job.
    pub enum JobType {
        UsersImport => "users_import",
        UsersExport => "users_export",
        VerificationEmail => "verification_email",
    }
}

/// A user profile field to include in a CSV export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportUsersField {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_as: Option<String>,
}

/// Request payload for creating a user export job.
///
/// The `fields` option applies to CSV exports. Auth0 uses its predefined field
/// set when it is omitted.
///
/// See the [Auth0 Create Export Users Job documentation](https://auth0.com/docs/api/management/v2/jobs/post-users-exports).
#[derive(Debug, Clone, Default, Serialize)]
pub struct ExportUsersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<ConnectionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<JobFileFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<ExportUsersField>>,
}

/// Request data for a multipart user import job.
///
/// `users` must contain an Auth0 bulk-import JSON document and is uploaded as
/// the multipart `users` file. Auth0 limits this file to 500 KB.
///
/// See the [Auth0 Create Import Users Job documentation](https://auth0.com/docs/api/management/v2/jobs/post-users-imports)
/// and [bulk user import format](https://auth0.com/docs/manage-users/user-migration/bulk-user-imports).
#[derive(Debug, Clone)]
pub struct ImportUsersRequest {
    pub users: Vec<u8>,
    pub connection_id: ConnectionId,
    pub upsert: Option<bool>,
    pub external_id: Option<String>,
    pub send_completion_email: Option<bool>,
}

/// Identity details used when sending verification for a non-primary database identity.
///
/// `user_id` is the provider-specific identity ID, such as `abc123` for the
/// identity behind `google-oauth2|abc123`, so it is not a [`UserId`].
#[derive(Debug, Clone, Serialize)]
pub struct VerificationEmailIdentity {
    pub user_id: String,
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<ConnectionId>,
}

/// Request payload for an email verification job.
///
/// See the [Auth0 Send Verification Email documentation](https://auth0.com/docs/api/management/v2/jobs/post-verification-email).
#[derive(Debug, Clone, Serialize)]
pub struct VerificationEmailRequest {
    pub user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ClientId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<VerificationEmailIdentity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

/// Summary counters for an import job.
#[derive(Debug, Clone, Deserialize)]
pub struct JobSummary {
    pub failed: Option<u64>,
    pub updated: Option<u64>,
    pub inserted: Option<u64>,
    pub total: Option<u64>,
}

/// An asynchronous Auth0 job.
///
/// Fields vary by job type and progress. For example, a completed export job
/// supplies `location`, while an import job supplies `summary`.
///
/// See the [Auth0 Get a Job documentation](https://auth0.com/docs/api/management/v2/jobs/get-jobs-by-id).
#[derive(Debug, Clone, Deserialize)]
pub struct Job {
    pub id: JobId,
    #[serde(rename = "type")]
    pub job_type: JobType,
    pub status: JobStatus,
    pub created_at: Option<String>,
    pub connection_id: Option<ConnectionId>,
    pub external_id: Option<String>,
    pub location: Option<String>,
    pub percentage_done: Option<u64>,
    pub time_left_seconds: Option<u64>,
    pub format: Option<JobFileFormat>,
    pub status_details: Option<String>,
    pub summary: Option<JobSummary>,
}

/// One validation error for a user in a bulk import.
#[derive(Debug, Clone, Deserialize)]
pub struct JobImportErrorDetail {
    pub code: Option<String>,
    pub message: Option<String>,
    pub path: Option<String>,
}

/// Errors associated with one user record in a bulk import.
#[derive(Debug, Clone, Deserialize)]
pub struct JobImportError {
    pub user: Option<serde_json::Value>,
    pub errors: Option<Vec<JobImportErrorDetail>>,
}

/// A job-level failure rather than a per-user import validation failure.
#[derive(Debug, Clone, Deserialize)]
pub struct GenericJobError {
    pub id: JobId,
    #[serde(rename = "type")]
    pub job_type: JobType,
    pub status: JobStatus,
    pub created_at: Option<String>,
    pub connection_id: Option<ConnectionId>,
    pub status_details: Option<String>,
}

/// Error response for an asynchronous job.
///
/// Import validation failures are returned as a list. A job-level failure is
/// returned as a job object with `status_details`.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum JobErrors {
    Import(Vec<JobImportError>),
    Generic(GenericJobError),
}
