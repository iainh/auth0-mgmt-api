use serde::{Deserialize, Serialize};

/// Application type in Auth0.
///
/// Specifies the type of application being created or modified.
/// See the [Auth0 Application Types documentation](https://auth0.com/docs/applications/application-settings)
/// for detailed information about each type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppType {
    /// Regular web application (web backend + frontend)
    #[serde(rename = "regular_web")]
    RegularWeb,
    /// Single Page Application (SPA)
    #[serde(rename = "spa")]
    Spa,
    /// Native mobile or desktop application
    #[serde(rename = "native")]
    Native,
    /// Machine-to-machine application
    #[serde(rename = "non_interactive")]
    NonInteractive,
}

/// OAuth 2.0 grant type.
///
/// Specifies the grant type for obtaining access tokens.
/// See the [Auth0 Grant Types documentation](https://auth0.com/docs/get-started/authentication-and-authorization-flow)
/// for detailed information about each grant type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    /// Authorization Code flow
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    /// Implicit flow (deprecated)
    #[serde(rename = "implicit")]
    Implicit,
    /// Client Credentials flow (machine-to-machine)
    #[serde(rename = "client_credentials")]
    ClientCredentials,
    /// Resource Owner Password flow
    #[serde(rename = "password")]
    Password,
    /// Refresh Token flow
    #[serde(rename = "refresh_token")]
    RefreshToken,
    /// Device Authorization flow
    #[serde(rename = "urn:ietf:params:oauth:grant-type:device_code")]
    DeviceCode,
    /// SAML assertion
    #[serde(rename = "urn:ietf:params:oauth:grant-type:saml2-bearer")]
    SamlBearer,
}

/// Connection strategy type.
///
/// Defines the authentication strategy for a connection.
/// See the [Auth0 Connections documentation](https://auth0.com/docs/connections)
/// for detailed information about each strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectionStrategy {
    /// Auth0 Database connection
    #[serde(rename = "auth0")]
    Auth0Database,
    /// Google OAuth 2.0
    #[serde(rename = "google-oauth2")]
    GoogleOAuth2,
    /// GitHub
    #[serde(rename = "github")]
    GitHub,
    /// LinkedIn
    #[serde(rename = "linkedin")]
    LinkedIn,
    /// Facebook
    #[serde(rename = "facebook")]
    Facebook,
    /// Windows Live/Microsoft
    #[serde(rename = "windowslive")]
    WindowsLive,
    /// Active Directory Federation Services
    #[serde(rename = "adfs")]
    Adfs,
    /// SAML
    #[serde(rename = "samlp")]
    Saml,
    /// Azure AD / Entra ID
    #[serde(rename = "waad")]
    AzureAd,
    /// Okta
    #[serde(rename = "okta")]
    Okta,
    /// Ping Identity
    #[serde(rename = "ping7")]
    PingIdentity,
    /// OneLogin
    #[serde(rename = "onelogin")]
    OneLogin,
    /// Salesforce
    #[serde(rename = "salesforce")]
    Salesforce,
    /// Custom database connection
    #[serde(rename = "custom")]
    Custom,
    /// OIDC-compliant provider
    #[serde(rename = "oidc")]
    Oidc,
}

/// Token endpoint authentication method.
///
/// Specifies how the application authenticates at the token endpoint.
/// See the [Auth0 Application Credentials](https://auth0.com/docs/applications/application-settings)
/// for detailed information about authentication methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenAuthMethod {
    /// No authentication (public clients)
    #[serde(rename = "none")]
    None,
    /// Client credentials in HTTP Basic Authentication header
    #[serde(rename = "client_secret_basic")]
    ClientSecretBasic,
    /// Client credentials in request body
    #[serde(rename = "client_secret_post")]
    ClientSecretPost,
    /// Client assertion (JWT) signed with client secret
    #[serde(rename = "client_secret_jwt")]
    ClientSecretJwt,
    /// Client assertion (JWT) signed with private key
    #[serde(rename = "private_key_jwt")]
    PrivateKeyJwt,
}

/// Organization usage setting.
///
/// Specifies whether the application can be used within organizations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationUsage {
    /// Application cannot be used in organizations
    #[serde(rename = "deny")]
    Deny,
    /// Application can be used in organizations
    #[serde(rename = "allow")]
    Allow,
    /// Application must be used within an organization
    #[serde(rename = "require")]
    Require,
}

/// Organization require behavior.
///
/// Specifies how the organization parameter is handled in authentication flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationRequireBehavior {
    /// No organization prompt
    #[serde(rename = "no_prompt")]
    NoPrompt,
    /// Prompt for organization at login
    #[serde(rename = "pre_login_prompt")]
    PreLoginPrompt,
    /// Prompt for organization after successful authentication
    #[serde(rename = "post_login_prompt")]
    PostLoginPrompt,
}
