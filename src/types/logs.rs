use serde::{Deserialize, Serialize};

/// Represents an Auth0 log entry.
///
/// Log entries contain information about authentication and management API events,
/// including user logins, sign-ups, failures, and administrative actions.
///
/// See the [Auth0 Logs documentation](https://auth0.com/docs/logs)
/// for detailed information about log types and event details.
#[derive(Debug, Clone, Deserialize)]
pub struct LogEvent {
    pub log_id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth0_client: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_info: Option<LocationInfo>,
}

impl LogEvent {
    /// Return Auth0's human-readable name for this event type.
    ///
    /// The original code remains available in [`Self::event_type`] for filtering
    /// and storage. If Auth0 introduces a code this crate does not yet know,
    /// this method returns the code itself so log processing remains
    /// forward-compatible.
    ///
    /// See the [Auth0 Log Event Type Codes documentation](https://auth0.com/docs/deploy-monitor/logs/log-event-type-codes).
    pub fn event_name(&self) -> &str {
        log_event_name(&self.event_type).unwrap_or(&self.event_type)
    }

    /// Return the most useful readable description available for this event.
    ///
    /// Auth0's event-specific `description` is preferred when present and
    /// non-empty. Otherwise this falls back to the mapped event name. This is
    /// useful for displaying events without exposing terse codes such as `f`,
    /// `sapi`, or `gd_auth_failed` to users.
    pub fn readable_description(&self) -> &str {
        self.description
            .as_deref()
            .filter(|description| !description.trim().is_empty())
            .unwrap_or_else(|| self.event_name())
    }
}

/// Look up Auth0's human-readable name for a tenant log event code.
///
/// Returns `None` for unknown codes. Auth0 adds event types over time, so
/// callers should preserve the original code and handle this case. [`LogEvent::event_name`]
/// provides that fallback automatically.
///
/// The mapping follows Auth0's
/// [Log Event Type Codes](https://auth0.com/docs/deploy-monitor/logs/log-event-type-codes)
/// and [Tenant Log Catalog](https://auth0.com/docs/tenant-logs).
pub fn log_event_name(code: &str) -> Option<&'static str> {
    Some(match code {
        "acls_summary" => "Network ACLs Summary",
        "actions_execution_failed" => "Actions Execution Failed",
        "acul_sdk_notice" => "ACUL SDK Notice",
        "api_limit" => "Rate Limit on the Authentication or Management APIs",
        "api_limit_warning" => "Rate Limit Warning on the Authentication or Management APIs",
        "appi" => "Notice for API Peak Performance initiated",
        "ciba_exchange_failed" => "Failed CIBA Exchange",
        "ciba_exchange_succeeded" => "Successful CIBA Exchange",
        "ciba_start_failed" => "Failed CIBA Start",
        "ciba_start_succeeded" => "Successful CIBA Start",
        "cls" => "Code/Link Sent",
        "cs" => "Code Sent",
        "custom_domain_verification_failed" => "Custom Domain Verification Failed",
        "depnote" => "Deprecation Notice",
        "f" => "Failed Login",
        "fc" => "Failed by Connector",
        "fce" => "Failed Change Email",
        "fco" => "Failed by CORS",
        "fcoa" => "Failed cross-origin authentication",
        "fcp" => "Failed Change Password",
        "fcph" => "Failed Post Change Password Hook",
        "fcpn" => "Failed Change Phone Number",
        "fcpr" => "Failed Change Password Request",
        "fcpro" => "Failed Connector Provisioning",
        "fcu" => "Failed Change Username",
        "fd" => "Failed Delegation",
        "fdeac" => "Failed Device Activation",
        "fdeaz" => "Failed Device Authorization Request",
        "fdecc" => "User Canceled Device Confirmation",
        "fdu" => "Failed User Deletion",
        "feacft" | "feccft" | "fecte" | "fede" | "fens" | "feoobft" | "feotpft" | "fepft"
        | "fepotpft" | "fercft" | "ferrt" | "fertft" | "feta" => "Failed Exchange",
        "federated_logout_failed" => "Failed Federated Logout",
        "fi" => "Failed invite accept",
        "flo" => "Failed Logout",
        "flows_execution_completed" => "Flows Execution Completed",
        "flows_execution_failed" => "Flows Execution Failed",
        "fn" => "Failed Sending Notification",
        "forms_submission_failed" => "Forms Submission Failed",
        "forms_submission_succeeded" => "Forms Submission Succeeded",
        "fp" => "Failed Login (Incorrect Password)",
        "fpar" => "Failed Pushed Authorization Request",
        "fpurh" => "Failed Post User Registration Hook",
        "fs" => "Failed Signup",
        "fsa" => "Failed Silent Auth",
        "fu" => "Failed Login (Invalid Email/Username)",
        "fui" => "Failed users import",
        "fv" => "Failed Verification Email",
        "fvr" => "Failed Verification Email Request",
        "gd_auth_email_verification" => "Email Verification Confirmed",
        "gd_auth_fail_email_verification" => "Email Verification Failed",
        "gd_auth_failed" => "MFA Auth failed",
        "gd_auth_rejected" => "MFA Auth rejected",
        "gd_auth_succeed" => "MFA Auth success",
        "gd_enrollment_complete" => "MFA enrollment complete",
        "gd_otp_rate_limit_exceed" => "Too many MFA failures",
        "gd_recovery_failed" => "Recovery failed",
        "gd_recovery_rate_limit_exceed" => "Multi-factor recovery code has failed too many times",
        "gd_recovery_succeed" => "MFA recovery success",
        "gd_send_email" => "MFA Email Sent",
        "gd_send_email_verification" => "Email Verification Sent",
        "gd_send_email_verification_failure" => "Email Verification Failed",
        "gd_send_pn" => "Push notification sent",
        "gd_send_pn_failure" => "Error Sending MFA Push Notification",
        "gd_send_sms" => "MFA SMS Sent",
        "gd_send_sms_failure" => "Error Sending MFA SMS",
        "gd_send_voice" => "MFA voice call success",
        "gd_send_voice_failure" => "MFA voice call failed",
        "gd_start_auth" => "Second factor started",
        "gd_start_enroll" => "MFA Enroll started",
        "gd_start_enroll_failed" => "MFA Enrollment Failed",
        "gd_tenant_update" => "Guardian tenant update",
        "gd_unenroll" => "Unenroll device account",
        "gd_update_device_account" => "Update device account",
        "gd_webauthn_challenge_failed" | "gd_webauthn_enrollment_failed" => {
            "WebAuthn browser error"
        }
        "idjag_exchange_failed" | "jwt_bearer_exchange_failed" => "Failed Exchange",
        "idjag_exchange_succeeded" => "Success Exchange",
        "kms_key_management_failure" => "Failed KMS API Operation",
        "kms_key_management_success" => "Success KMS API Operation",
        "kms_key_state_changed" => "KMS Key State Change",
        "limit_delegation" => "Too Many Calls to /delegation",
        "limit_mu" => "Blocked IP Address",
        "limit_sul" | "limit_wc" => "Blocked Account",
        "mfar" => "MFA Required",
        "mgmt_api_read" => "Management API read Operation",
        "my_account_authentication_method_failed" => {
            "Failed authentication method operation in My Account API"
        }
        "my_account_authentication_method_succeeded" => {
            "Successful authentication method operation in My Account API"
        }
        "oidc_backchannel_logout_failed" => "Failed OIDC Back-Channel Logout request",
        "oidc_backchannel_logout_succeeded" => "Successful OIDC Back-Channel Logout request",
        "organization_member_added" => "Organization Member Added",
        "passkey_challenge_failed" => "Passkey Challenge Failed",
        "passkey_challenge_started" => "Passkey Challenge Started",
        "passkey_register_failed" => "Passkey Registration Failed",
        "passkey_register_started" => "Passkey Registration Started",
        "pla" => "Pre-login assessment",
        "pwd_leak" => "Breached password",
        "reset_pwd_leak" => "Breached Password on Reset",
        "resource_cleanup" => "Success Resource Cleanup",
        "rich_consents_access_error" => "Rich Consents Access Error",
        "s" => "Success Login",
        "sapi" => "Success API Operation",
        "sce" => "Success Change Email",
        "scoa" => "Success cross-origin authentication",
        "scp" => "Success Change Password",
        "scpn" => "Success Change Phone Number",
        "scpr" => "Success Change Password Request",
        "scu" => "Success Change Username",
        "scv" => "Success Credential Validation",
        "sd" => "Success Delegation",
        "sdu" => "Success User Deletion",
        "seacft" | "seccft" | "secte" | "sede" | "sens" | "seoobft" | "seotpft" | "sepft"
        | "sepkoobft" | "sepkotpft" | "sepkrcft" | "sercft" | "sertft" | "seta" => {
            "Success Exchange"
        }
        "si" => "Successfully accepted a user invite",
        "signup_pwd_leak" => "Breached Password on Signup",
        "slo" => "Success Logout",
        "srrt" => "Success Revocation",
        "ss" => "Success Signup",
        "ss_sso_failure" => "Failed SS-SSO Operation",
        "ss_sso_info" => "Information from an SS-SSO Operation",
        "ss_sso_success" => "Success SS-SSO Operation",
        "ssa" => "Success Silent Auth",
        "sscim" => "Successful SCIM Operation",
        "sui" => "Successfully imported users",
        "sv" => "Success Verification Email",
        "svr" => "Success Verification Email Request",
        "too_many_records" => "Max Amount of Authenticators",
        "ublkdu" => "User login block released",
        "universal_logout_failed" => "Failed Universal Logout request",
        "universal_logout_succeeded" => "Successful Universal Logout request",
        "w" => "Warning During Login",
        "wn" => "Warning Sending Notification",
        "wum" => "Warning User Management",
        _ => return None,
    })
}

/// Geographic location information from log entries.
///
/// Contains country, city, and timezone information inferred from the user's IP address.
#[derive(Debug, Clone, Deserialize)]
pub struct LocationInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_code: Option<String>,
}

/// Query parameters for listing log entries.
///
/// See the [Auth0 List Logs documentation](https://auth0.com/docs/api/management/v2#!/Logs/get_logs)
/// for detailed information about available filters and search options.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListLogsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fields: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take: Option<u32>,
}

/// Paginated response for log list operations.
///
/// Returned when `include_totals` is set to `true` in list parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct LogsPage {
    /// List of log events in this page.
    pub logs: Vec<LogEvent>,
    /// Starting index of this page (zero-based).
    pub start: u32,
    /// Maximum number of results per page.
    pub limit: u32,
    /// Total number of log events matching the query.
    pub total: u32,
}
