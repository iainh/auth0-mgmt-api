use reqwest::multipart::{Form, Part};

use crate::client::ManagementClient;
use crate::error::Result;
use crate::types::JobId;
use crate::types::jobs::{
    ExportUsersRequest, ImportUsersRequest, Job, JobErrors, VerificationEmailRequest,
};

/// API operations for asynchronous Auth0 jobs.
///
/// Supports bulk user export and import, verification emails, and job status
/// and error retrieval.
pub struct JobsApi<'a> {
    client: &'a ManagementClient,
}

impl<'a> JobsApi<'a> {
    pub(crate) fn new(client: &'a ManagementClient) -> Self {
        Self { client }
    }

    /// Create a user export job.
    ///
    /// Requires the `read:users` scope. Poll [`Self::get`] until the job is
    /// complete, then use the temporary download URL in [`Job::location`].
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2/jobs/post-users-exports>
    pub async fn export_users(&self, request: ExportUsersRequest) -> Result<Job> {
        let url = self.client.base_url().join("api/v2/jobs/users-exports")?;
        self.client.post(url, &request).await
    }

    /// Create a user import job using a multipart file upload.
    ///
    /// Requires the `create:users` scope. The user document must follow Auth0's
    /// [bulk import format](https://auth0.com/docs/manage-users/user-migration/bulk-user-imports)
    /// and must not exceed 500 KB.
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2/jobs/post-users-imports>
    pub async fn import_users(&self, request: ImportUsersRequest) -> Result<Job> {
        let url = self.client.base_url().join("api/v2/jobs/users-imports")?;
        let users = Part::bytes(request.users)
            .file_name("users.json")
            .mime_str("application/json")?;
        let mut form = Form::new()
            .part("users", users)
            .text("connection_id", request.connection_id);

        if let Some(upsert) = request.upsert {
            form = form.text("upsert", upsert.to_string());
        }
        if let Some(external_id) = request.external_id {
            form = form.text("external_id", external_id);
        }
        if let Some(send_completion_email) = request.send_completion_email {
            form = form.text("send_completion_email", send_completion_email.to_string());
        }

        self.client.post_multipart(url, form).await
    }

    /// Send an email address verification email to a user.
    ///
    /// Requires the `update:users` scope.
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2/jobs/post-verification-email>
    pub async fn send_verification_email(&self, request: VerificationEmailRequest) -> Result<Job> {
        let url = self
            .client
            .base_url()
            .join("api/v2/jobs/verification-email")?;
        self.client.post(url, &request).await
    }

    /// Get an asynchronous job by ID.
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2/jobs/get-jobs-by-id>
    pub async fn get(&self, id: JobId) -> Result<Job> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/jobs/{}", urlencoding::encode(id.as_str())))?;
        self.client.get(url).await
    }

    /// Get error details for a completed job.
    ///
    /// Returns `None` when Auth0 responds with `204 No Content`, indicating
    /// that no errors are available.
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2/jobs/get-errors>
    pub async fn get_errors(&self, id: JobId) -> Result<Option<JobErrors>> {
        let url = self.client.base_url().join(&format!(
            "api/v2/jobs/{}/errors",
            urlencoding::encode(id.as_str())
        ))?;
        self.client.get_optional(url).await
    }
}
