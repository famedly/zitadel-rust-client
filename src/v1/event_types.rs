#![allow(non_snake_case, missing_docs)]

use super::{event_types_imports::*, event_types_manual::*};

/// `action.added` event, internal/repository/action/action.go AddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActionAdded {
	#[serde(rename = "name")]
	pub Name: String,
	#[serde(rename = "script")]
	pub Script: Option<String>,
	#[serde(rename = "timeout")]
	pub Timeout: Option<Duration>,
	#[serde(rename = "allowedToFail")]
	pub AllowedToFail: bool,
}

/// `action.changed` event, internal/repository/action/action.go ChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActionChanged {
	#[serde(rename = "name")]
	pub Name: Option<String>,
	#[serde(rename = "script")]
	pub Script: Option<String>,
	#[serde(rename = "timeout")]
	pub Timeout: Option<Duration>,
	#[serde(rename = "allowedToFail")]
	pub AllowedToFail: Option<bool>,
}

/// `action.deactivated` event, internal/repository/action/action.go
/// DeactivatedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActionDeactivated {}

/// `action.reactivated` event, internal/repository/action/action.go
/// ReactivatedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActionReactivated {}

/// `action.removed` event, internal/repository/action/action.go RemovedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActionRemoved {}

/// `auth_request.added` event, internal/repository/authrequest/auth_request.go
/// AddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthRequestAdded {
	#[serde(rename = "login_client")]
	pub LoginClient: String,
	#[serde(rename = "client_id")]
	pub ClientID: String,
	#[serde(rename = "redirect_uri")]
	pub RedirectURI: String,
	#[serde(rename = "state")]
	pub State: Option<String>,
	#[serde(rename = "nonce")]
	pub Nonce: Option<String>,
	#[serde(rename = "scope")]
	pub Scope: Option<Vec<String>>,
	#[serde(rename = "audience")]
	pub Audience: Option<Vec<String>>,
	#[serde(rename = "response_type")]
	pub ResponseType: Option<OIDCResponseType>,
	#[serde(rename = "response_mode")]
	pub ResponseMode: Option<OIDCResponseMode>,
	#[serde(rename = "code_challenge")]
	pub CodeChallenge: Option<OIDCCodeChallenge>,
	#[serde(rename = "prompt")]
	pub Prompt: Option<Vec<Prompt>>,
	#[serde(rename = "ui_locales")]
	pub UILocales: Option<Vec<String>>,
	#[serde(rename = "max_age")]
	pub MaxAge: Option<Duration>,
	#[serde(rename = "login_hint")]
	pub LoginHint: Option<String>,
	#[serde(rename = "hint_user_id")]
	pub HintUserID: Option<String>,
	#[serde(rename = "need_refresh_token")]
	pub NeedRefreshToken: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeAddedEvent {}

/// `auth_request.code.added` event,
/// internal/repository/authrequest/auth_request.go CodeAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthRequestCodeAdded {}

/// `auth_request.code.exchanged` event,
/// internal/repository/authrequest/auth_request.go CodeExchangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthRequestCodeExchanged {}

/// `auth_request.failed` event, internal/repository/authrequest/auth_request.go
/// FailedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthRequestFailed {
	#[serde(rename = "reason")]
	pub Reason: Option<OIDCErrorReason>,
}

/// `auth_request.session.linked` event,
/// internal/repository/authrequest/auth_request.go SessionLinkedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthRequestSessionLinked {
	#[serde(rename = "session_id")]
	pub SessionID: String,
	#[serde(rename = "user_id")]
	pub UserID: String,
	#[serde(rename = "auth_time")]
	pub AuthTime: Time,
	#[serde(rename = "auth_methods")]
	pub AuthMethods: Vec<UserAuthMethodType>,
}

/// `auth_request.succeeded` event,
/// internal/repository/authrequest/auth_request.go SucceededEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthRequestSucceeded {}

/// `device.authorization.added` event,
/// internal/repository/deviceauth/device_auth.go AddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceAuthorizationAdded {
	pub ClientID: String,
	pub DeviceCode: String,
	pub UserCode: String,
	pub Expires: Time,
	pub Scopes: Vec<String>,
	pub Audience: Vec<String>,
	pub State: DeviceAuthState,
	pub NeedRefreshToken: bool,
}

/// `device.authorization.approved` event,
/// internal/repository/deviceauth/device_auth.go ApprovedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceAuthorizationApproved {
	pub UserID: String,
	pub UserOrgID: String,
	pub UserAuthMethods: Vec<UserAuthMethodType>,
	pub AuthTime: Time,
	pub PreferredLanguage: Tag,
	pub UserAgent: UserAgent,
	pub SessionID: String,
}

/// `device.authorization.canceled` event,
/// internal/repository/deviceauth/device_auth.go CanceledEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceAuthorizationCanceled {
	pub Reason: DeviceAuthCanceled,
}

/// `device.authorization.done` event,
/// internal/repository/deviceauth/device_auth.go DoneEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceAuthorizationDone {}

/// `execution.removed` event, internal/repository/execution/execution.go
/// RemovedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecutionRemoved {}

/// `execution.set` event, internal/repository/execution/execution.go SetEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecutionSet {
	#[serde(rename = "targets")]
	pub Targets: Vec<String>,
	#[serde(rename = "includes")]
	pub Includes: Vec<String>,
}

/// `iam.idp.config.added` event, internal/repository/instance/idp_config.go
/// IDPConfigAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IamIdpConfigAdded {
	#[serde(flatten)]
	pub IDPConfigAddedEvent: idpconfig::IDPConfigAddedEvent,
}

/// `iam.idp.config.changed` event, internal/repository/instance/idp_config.go
/// IDPConfigChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IamIdpConfigChanged {
	#[serde(flatten)]
	pub IDPConfigChangedEvent: idpconfig::IDPConfigChangedEvent,
}

/// `iam.idp.config.deactivated` event,
/// internal/repository/instance/idp_config.go IDPConfigDeactivatedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IamIdpConfigDeactivated {
	#[serde(flatten)]
	pub IDPConfigDeactivatedEvent: idpconfig::IDPConfigDeactivatedEvent,
}

/// `iam.idp.config.reactivated` event,
/// internal/repository/instance/idp_config.go IDPConfigReactivatedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IamIdpConfigReactivated {
	#[serde(flatten)]
	pub IDPConfigReactivatedEvent: idpconfig::IDPConfigReactivatedEvent,
}

/// `iam.idp.config.removed` event, internal/repository/instance/idp_config.go
/// IDPConfigRemovedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IamIdpConfigRemoved {
	#[serde(flatten)]
	pub IDPConfigRemovedEvent: idpconfig::IDPConfigRemovedEvent,
}

/// `iam.idp.jwt.config.added` event,
/// internal/repository/instance/idp_jwt_config.go IDPJWTConfigAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IamIdpJwtConfigAdded {
	#[serde(flatten)]
	pub JWTConfigAddedEvent: idpconfig::JWTConfigAddedEvent,
}

/// `iam.idp.jwt.config.changed` event,
/// internal/repository/instance/idp_jwt_config.go IDPJWTConfigChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IamIdpJwtConfigChanged {
	#[serde(flatten)]
	pub JWTConfigChangedEvent: idpconfig::JWTConfigChangedEvent,
}

/// `iam.idp.oidc.config.added` event,
/// internal/repository/instance/idp_oidc_config.go IDPOIDCConfigAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IamIdpOidcConfigAdded {
	#[serde(flatten)]
	pub OIDCConfigAddedEvent: idpconfig::OIDCConfigAddedEvent,
}

/// `iam.idp.oidc.config.changed` event,
/// internal/repository/instance/idp_oidc_config.go IDPOIDCConfigChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IamIdpOidcConfigChanged {
	#[serde(flatten)]
	pub OIDCConfigChangedEvent: idpconfig::OIDCConfigChangedEvent,
}

/// `idpintent.failed` event, internal/repository/idpintent/intent.go
/// FailedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdpintentFailed {
	#[serde(rename = "reason")]
	pub Reason: Option<String>,
}

/// `idpintent.ldap.succeeded` event, internal/repository/idpintent/intent.go
/// LDAPSucceededEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdpintentLdapSucceeded {
	#[serde(rename = "idpUser")]
	pub IDPUser: Vec<u8>,
	#[serde(rename = "idpUserId")]
	pub IDPUserID: Option<String>,
	#[serde(rename = "idpUserName")]
	pub IDPUserName: Option<String>,
	#[serde(rename = "userId")]
	pub UserID: Option<String>,
	#[serde(rename = "user")]
	pub EntryAttributes: Option<Map<String, String>>,
}

/// `idpintent.saml.requested` event, internal/repository/idpintent/intent.go
/// SAMLRequestEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdpintentSamlRequested {
	#[serde(rename = "requestId")]
	pub RequestID: String,
}

/// `idpintent.saml.succeeded` event, internal/repository/idpintent/intent.go
/// SAMLSucceededEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdpintentSamlSucceeded {
	#[serde(rename = "idpUser")]
	pub IDPUser: Vec<u8>,
	#[serde(rename = "idpUserId")]
	pub IDPUserID: Option<String>,
	#[serde(rename = "idpUserName")]
	pub IDPUserName: Option<String>,
	#[serde(rename = "userId")]
	pub UserID: Option<String>,
	#[serde(rename = "assertion")]
	pub Assertion: Option<CryptoValue>,
}

/// `idpintent.started` event, internal/repository/idpintent/intent.go
/// StartedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdpintentStarted {
	#[serde(rename = "successURL")]
	pub SuccessURL: URL,
	#[serde(rename = "failureURL")]
	pub FailureURL: URL,
	#[serde(rename = "idpId")]
	pub IDPID: String,
}

/// `idpintent.succeeded` event, internal/repository/idpintent/intent.go
/// SucceededEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdpintentSucceeded {
	#[serde(rename = "idpUser")]
	pub IDPUser: Vec<u8>,
	#[serde(rename = "idpUserId")]
	pub IDPUserID: Option<String>,
	#[serde(rename = "idpUserName")]
	pub IDPUserName: Option<String>,
	#[serde(rename = "userId")]
	pub UserID: Option<String>,
	#[serde(rename = "idpAccessToken")]
	pub IDPAccessToken: Option<CryptoValue>,
	#[serde(rename = "idpIdToken")]
	pub IDPIDToken: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SAMLSucceededEvent {
	#[serde(rename = "idpUser")]
	pub IDPUser: Vec<u8>,
	#[serde(rename = "idpUserId")]
	pub IDPUserID: Option<String>,
	#[serde(rename = "idpUserName")]
	pub IDPUserName: Option<String>,
	#[serde(rename = "userId")]
	pub UserID: Option<String>,
	#[serde(rename = "assertion")]
	pub Assertion: Option<CryptoValue>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LDAPSucceededEvent {
	#[serde(rename = "idpUser")]
	pub IDPUser: Vec<u8>,
	#[serde(rename = "idpUserId")]
	pub IDPUserID: Option<String>,
	#[serde(rename = "idpUserName")]
	pub IDPUserName: Option<String>,
	#[serde(rename = "userId")]
	pub UserID: Option<String>,
	#[serde(rename = "user")]
	pub EntryAttributes: Option<Map<String, String>>,
}

/// `instance.idp.apple.added` event, internal/repository/instance/idp.go
/// AppleIDPAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpAppleAdded {
	#[serde(flatten)]
	pub AppleIDPAddedEvent: idp::AppleIDPAddedEvent,
}

/// `instance.idp.apple.changed` event, internal/repository/instance/idp.go
/// AppleIDPChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpAppleChanged {
	#[serde(flatten)]
	pub AppleIDPChangedEvent: idp::AppleIDPChangedEvent,
}

/// `instance.idp.azure.added` event, internal/repository/instance/idp.go
/// AzureADIDPAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpAzureAdded {
	#[serde(flatten)]
	pub AzureADIDPAddedEvent: idp::AzureADIDPAddedEvent,
}

/// `instance.idp.azure.changed` event, internal/repository/instance/idp.go
/// AzureADIDPChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpAzureChanged {
	#[serde(flatten)]
	pub AzureADIDPChangedEvent: idp::AzureADIDPChangedEvent,
}

/// `instance.idp.github.added` event, internal/repository/instance/idp.go
/// GitHubIDPAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpGithubAdded {
	#[serde(flatten)]
	pub GitHubIDPAddedEvent: idp::GitHubIDPAddedEvent,
}

/// `instance.idp.github.changed` event, internal/repository/instance/idp.go
/// GitHubIDPChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpGithubChanged {
	#[serde(flatten)]
	pub GitHubIDPChangedEvent: idp::GitHubIDPChangedEvent,
}

/// `instance.idp.github_enterprise.added` event,
/// internal/repository/instance/idp.go GitHubEnterpriseIDPAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpGithubEnterpriseAdded {
	#[serde(flatten)]
	pub GitHubEnterpriseIDPAddedEvent: idp::GitHubEnterpriseIDPAddedEvent,
}

/// `instance.idp.github_enterprise.changed` event,
/// internal/repository/instance/idp.go GitHubEnterpriseIDPChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpGithubEnterpriseChanged {
	#[serde(flatten)]
	pub GitHubEnterpriseIDPChangedEvent: idp::GitHubEnterpriseIDPChangedEvent,
}

/// `instance.idp.gitlab.added` event, internal/repository/instance/idp.go
/// GitLabIDPAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpGitlabAdded {
	#[serde(flatten)]
	pub GitLabIDPAddedEvent: idp::GitLabIDPAddedEvent,
}

/// `instance.idp.gitlab.changed` event, internal/repository/instance/idp.go
/// GitLabIDPChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpGitlabChanged {
	#[serde(flatten)]
	pub GitLabIDPChangedEvent: idp::GitLabIDPChangedEvent,
}

/// `instance.idp.gitlab_self_hosted.added` event,
/// internal/repository/instance/idp.go GitLabSelfHostedIDPAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpGitlabSelfHostedAdded {
	#[serde(flatten)]
	pub GitLabSelfHostedIDPAddedEvent: idp::GitLabSelfHostedIDPAddedEvent,
}

/// `instance.idp.gitlab_self_hosted.changed` event,
/// internal/repository/instance/idp.go GitLabSelfHostedIDPChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpGitlabSelfHostedChanged {
	#[serde(flatten)]
	pub GitLabSelfHostedIDPChangedEvent: idp::GitLabSelfHostedIDPChangedEvent,
}

/// `instance.idp.google.added` event, internal/repository/instance/idp.go
/// GoogleIDPAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpGoogleAdded {
	#[serde(flatten)]
	pub GoogleIDPAddedEvent: idp::GoogleIDPAddedEvent,
}

/// `instance.idp.google.changed` event, internal/repository/instance/idp.go
/// GoogleIDPChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpGoogleChanged {
	#[serde(flatten)]
	pub GoogleIDPChangedEvent: idp::GoogleIDPChangedEvent,
}

/// `instance.idp.jwt.added` event, internal/repository/instance/idp.go
/// JWTIDPAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpJwtAdded {
	#[serde(flatten)]
	pub JWTIDPAddedEvent: idp::JWTIDPAddedEvent,
}

/// `instance.idp.jwt.changed` event, internal/repository/instance/idp.go
/// JWTIDPChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpJwtChanged {
	#[serde(flatten)]
	pub JWTIDPChangedEvent: idp::JWTIDPChangedEvent,
}

/// `instance.idp.ldap.v2.added` event, internal/repository/instance/idp.go
/// LDAPIDPAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpLdapV2Added {
	#[serde(flatten)]
	pub LDAPIDPAddedEvent: idp::LDAPIDPAddedEvent,
}

/// `instance.idp.ldap.v2.changed` event, internal/repository/instance/idp.go
/// LDAPIDPChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpLdapV2Changed {
	#[serde(flatten)]
	pub LDAPIDPChangedEvent: idp::LDAPIDPChangedEvent,
}

/// `instance.idp.oauth.added` event, internal/repository/instance/idp.go
/// OAuthIDPAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpOauthAdded {
	#[serde(flatten)]
	pub OAuthIDPAddedEvent: idp::OAuthIDPAddedEvent,
}

/// `instance.idp.oauth.changed` event, internal/repository/instance/idp.go
/// OAuthIDPChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpOauthChanged {
	#[serde(flatten)]
	pub OAuthIDPChangedEvent: idp::OAuthIDPChangedEvent,
}

/// `instance.idp.oidc.added` event, internal/repository/instance/idp.go
/// OIDCIDPAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpOidcAdded {
	#[serde(flatten)]
	pub OIDCIDPAddedEvent: idp::OIDCIDPAddedEvent,
}

/// `instance.idp.oidc.changed` event, internal/repository/instance/idp.go
/// OIDCIDPChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpOidcChanged {
	#[serde(flatten)]
	pub OIDCIDPChangedEvent: idp::OIDCIDPChangedEvent,
}

/// `instance.idp.oidc.migrated.azure` event,
/// internal/repository/instance/idp.go OIDCIDPMigratedAzureADEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpOidcMigratedAzure {
	#[serde(flatten)]
	pub OIDCIDPMigratedAzureADEvent: idp::OIDCIDPMigratedAzureADEvent,
}

/// `instance.idp.oidc.migrated.google` event,
/// internal/repository/instance/idp.go OIDCIDPMigratedGoogleEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpOidcMigratedGoogle {
	#[serde(flatten)]
	pub OIDCIDPMigratedGoogleEvent: idp::OIDCIDPMigratedGoogleEvent,
}

/// `instance.idp.removed` event, internal/repository/instance/idp.go
/// IDPRemovedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpRemoved {
	#[serde(flatten)]
	pub RemovedEvent: idp::RemovedEvent,
}

/// `instance.idp.saml.added` event, internal/repository/instance/idp.go
/// SAMLIDPAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpSamlAdded {
	#[serde(flatten)]
	pub SAMLIDPAddedEvent: idp::SAMLIDPAddedEvent,
}

/// `instance.idp.saml.changed` event, internal/repository/instance/idp.go
/// SAMLIDPChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceIdpSamlChanged {
	#[serde(flatten)]
	pub SAMLIDPChangedEvent: idp::SAMLIDPChangedEvent,
}

/// `org.idp.config.added` event, internal/repository/idpconfig/idp_config.go
/// IDPConfigAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrgIdpConfigAdded {
	#[serde(rename = "idpConfigId")]
	pub ConfigID: String,
	#[serde(rename = "name")]
	pub Name: Option<String>,
	#[serde(rename = "idpType")]
	pub Typ: Option<IDPConfigType>,
	#[serde(rename = "stylingType")]
	pub StylingType: Option<IDPConfigStylingType>,
	#[serde(rename = "autoRegister")]
	pub AutoRegister: Option<bool>,
}

/// `org.idp.config.changed` event, internal/repository/idpconfig/idp_config.go
/// IDPConfigChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrgIdpConfigChanged {
	#[serde(rename = "idpConfigId")]
	pub ConfigID: String,
	#[serde(rename = "name")]
	pub Name: Option<String>,
	#[serde(rename = "stylingType")]
	pub StylingType: Option<IDPConfigStylingType>,
	#[serde(rename = "autoRegister")]
	pub AutoRegister: Option<bool>,
}

/// `org.idp.config.deactivated` event,
/// internal/repository/idpconfig/idp_config.go IDPConfigDeactivatedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrgIdpConfigDeactivated {
	#[serde(rename = "idpConfigId")]
	pub ConfigID: String,
}

/// `org.idp.config.reactivated` event,
/// internal/repository/idpconfig/idp_config.go IDPConfigReactivatedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrgIdpConfigReactivated {
	#[serde(rename = "idpConfigId")]
	pub ConfigID: String,
}

/// `org.idp.config.removed` event, internal/repository/idpconfig/idp_config.go
/// IDPConfigRemovedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrgIdpConfigRemoved {
	#[serde(rename = "idpConfigId")]
	pub ConfigID: String,
}

/// `user.deactivated` event, internal/repository/user/user.go
/// UserDeactivatedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserDeactivated {}

/// `user.domain.claimed` event, internal/repository/user/user.go
/// DomainClaimedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserDomainClaimed {
	#[serde(rename = "userName")]
	pub UserName: String,
	#[serde(rename = "triggerOrigin")]
	pub TriggeredAtOrigin: Option<String>,
}

/// `user.domain.claimed.sent` event, internal/repository/user/user.go
/// DomainClaimedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserDomainClaimedSent {
	#[serde(rename = "userName")]
	pub UserName: String,
	#[serde(rename = "triggerOrigin")]
	pub TriggeredAtOrigin: Option<String>,
}

/// `user.grant.added` event, internal/repository/usergrant/user_grant.go
/// UserGrantAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserGrantAdded {
	#[serde(rename = "userId")]
	pub UserID: Option<String>,
	#[serde(rename = "projectId")]
	pub ProjectID: Option<String>,
	#[serde(rename = "grantId")]
	pub ProjectGrantID: Option<String>,
	#[serde(rename = "roleKeys")]
	pub RoleKeys: Option<Vec<String>>,
}

/// `user.grant.cascade.changed` event,
/// internal/repository/usergrant/user_grant.go UserGrantCascadeChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserGrantCascadeChanged {
	#[serde(rename = "roleKeys")]
	pub RoleKeys: Option<Vec<String>>,
}

/// `user.grant.cascade.removed` event,
/// internal/repository/usergrant/user_grant.go UserGrantCascadeRemovedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserGrantCascadeRemoved {}

/// `user.grant.changed` event, internal/repository/usergrant/user_grant.go
/// UserGrantChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserGrantChanged {
	#[serde(rename = "roleKeys")]
	pub RoleKeys: Vec<String>,
}

/// `user.grant.deactivated` event, internal/repository/usergrant/user_grant.go
/// UserGrantDeactivatedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserGrantDeactivated {}

/// `user.grant.reactivated` event, internal/repository/usergrant/user_grant.go
/// UserGrantReactivatedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserGrantReactivated {}

/// `user.grant.removed` event, internal/repository/usergrant/user_grant.go
/// UserGrantRemovedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserGrantRemoved {}

/// `user.human.added` event, internal/repository/user/human.go HumanAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserHumanAdded {
	#[serde(rename = "userName")]
	pub UserName: String,
	#[serde(rename = "firstName")]
	pub FirstName: Option<String>,
	#[serde(rename = "lastName")]
	pub LastName: Option<String>,
	#[serde(rename = "nickName")]
	pub NickName: Option<String>,
	#[serde(rename = "displayName")]
	pub DisplayName: Option<String>,
	#[serde(rename = "preferredLanguage")]
	pub PreferredLanguage: Option<Tag>,
	#[serde(rename = "gender")]
	pub Gender: Option<Gender>,
	#[serde(rename = "email")]
	pub EmailAddress: Option<EmailAddress>,
	#[serde(rename = "phone")]
	pub PhoneNumber: Option<PhoneNumber>,
	#[serde(rename = "country")]
	pub Country: Option<String>,
	#[serde(rename = "locality")]
	pub Locality: Option<String>,
	#[serde(rename = "postalCode")]
	pub PostalCode: Option<String>,
	#[serde(rename = "region")]
	pub Region: Option<String>,
	#[serde(rename = "streetAddress")]
	pub StreetAddress: Option<String>,
	// New events only use EncodedHash. However, the secret field
	// is preserved to handle events older than the switch to Passwap.
	#[serde(rename = "secret")]
	pub Secret: Option<CryptoValue>,
	#[serde(rename = "encodedHash")]
	pub EncodedHash: Option<String>,
	#[serde(rename = "changeRequired")]
	pub ChangeRequired: Option<bool>,
}

/// `user.human.initialization.check.failed` event,
/// internal/repository/user/human.go HumanInitializedCheckFailedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserHumanInitializationCheckFailed {}

/// `user.human.initialization.check.succeeded` event,
/// internal/repository/user/human.go HumanInitializedCheckSucceededEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserHumanInitializationCheckSucceeded {}

/// `user.human.initialization.code.added` event,
/// internal/repository/user/human.go HumanInitialCodeAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserHumanInitializationCodeAdded {
	#[serde(rename = "code")]
	pub Code: Option<CryptoValue>,
	#[serde(rename = "expiry")]
	pub Expiry: Option<Duration>,
	#[serde(rename = "triggerOrigin")]
	pub TriggeredAtOrigin: Option<String>,
	#[serde(rename = "authRequestID")]
	pub AuthRequestID: Option<String>,
}

/// `user.human.initialization.code.sent` event,
/// internal/repository/user/human.go HumanInitialCodeSentEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserHumanInitializationCodeSent {}

/// `user.human.selfregistered` event, internal/repository/user/human.go
/// HumanRegisteredEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserHumanSelfregistered {
	#[serde(rename = "userName")]
	pub UserName: String,
	#[serde(rename = "firstName")]
	pub FirstName: Option<String>,
	#[serde(rename = "lastName")]
	pub LastName: Option<String>,
	#[serde(rename = "nickName")]
	pub NickName: Option<String>,
	#[serde(rename = "displayName")]
	pub DisplayName: Option<String>,
	#[serde(rename = "preferredLanguage")]
	pub PreferredLanguage: Option<Tag>,
	#[serde(rename = "gender")]
	pub Gender: Option<Gender>,
	#[serde(rename = "email")]
	pub EmailAddress: Option<EmailAddress>,
	#[serde(rename = "phone")]
	pub PhoneNumber: Option<PhoneNumber>,
	#[serde(rename = "country")]
	pub Country: Option<String>,
	#[serde(rename = "locality")]
	pub Locality: Option<String>,
	#[serde(rename = "postalCode")]
	pub PostalCode: Option<String>,
	#[serde(rename = "region")]
	pub Region: Option<String>,
	#[serde(rename = "streetAddress")]
	pub StreetAddress: Option<String>,
	// New events only use EncodedHash. However, the secret field
	// is preserved to handle events older than the switch to Passwap.
	#[serde(rename = "secret")]
	pub Secret: Option<CryptoValue>, // legacy
	#[serde(rename = "encodedHash")]
	pub EncodedHash: Option<String>,
	#[serde(rename = "changeRequired")]
	pub ChangeRequired: Option<bool>,
	#[serde(rename = "userAgentID")]
	pub UserAgentID: Option<String>,
}

/// `user.human.signed.out` event, internal/repository/user/human.go
/// HumanSignedOutEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserHumanSignedOut {
	#[serde(rename = "userAgentID")]
	pub UserAgentID: String,
}

/// `user.impersonated` event, internal/repository/user/user.go
/// UserImpersonatedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserImpersonated {
	#[serde(rename = "applicationId")]
	pub ApplicationID: Option<String>,
	#[serde(rename = "actor")]
	pub Actor: Option<TokenActor>,
}

/// `user.locked` event, internal/repository/user/user.go UserLockedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserLocked {}

/// `user.reactivated` event, internal/repository/user/user.go
/// UserReactivatedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserReactivated {}

/// `user.removed` event, internal/repository/user/user.go UserRemovedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserRemoved {}

/// `user.token.added` event, internal/repository/user/user.go
/// UserTokenAddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTokenAdded {
	#[serde(rename = "tokenId")]
	pub TokenID: Option<String>,
	#[serde(rename = "applicationId")]
	pub ApplicationID: Option<String>,
	#[serde(rename = "userAgentId")]
	pub UserAgentID: Option<String>,
	#[serde(rename = "refreshTokenID")]
	pub RefreshTokenID: Option<String>,
	#[serde(rename = "audience")]
	pub Audience: Option<Vec<String>>,
	#[serde(rename = "scopes")]
	pub Scopes: Option<Vec<String>>,
	#[serde(rename = "authMethodsReferences")]
	pub AuthMethodsReferences: Option<Vec<String>>,
	#[serde(rename = "authTime")]
	pub AuthTime: Option<Time>,
	#[serde(rename = "expiration")]
	pub Expiration: Option<Time>,
	#[serde(rename = "preferredLanguage")]
	pub PreferredLanguage: Option<String>,
	#[serde(rename = "reason")]
	pub Reason: Option<TokenReason>,
	#[serde(rename = "actor")]
	pub Actor: Option<TokenActor>,
}

/// `user.token.removed` event, internal/repository/user/user.go
/// UserTokenRemovedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTokenRemoved {
	#[serde(rename = "tokenId")]
	pub TokenID: String,
}

/// `user.token.v2.added` event, internal/repository/user/user.go
/// UserTokenV2AddedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTokenV2Added {
	#[serde(rename = "tokenId")]
	pub TokenID: Option<String>,
}

/// `user.unlocked` event, internal/repository/user/user.go UserUnlockedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserUnlocked {}

/// `user.username.changed` event, internal/repository/user/user.go
/// UsernameChangedEvent
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserUsernameChanged {
	#[serde(rename = "userName")]
	pub UserName: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ZitadelEvent {
	ActionAdded(ActionAdded),
	ActionChanged(ActionChanged),
	ActionDeactivated(ActionDeactivated),
	ActionReactivated(ActionReactivated),
	ActionRemoved(ActionRemoved),
	AuthRequestAdded(AuthRequestAdded),
	AuthRequestCodeAdded(AuthRequestCodeAdded),
	AuthRequestCodeExchanged(AuthRequestCodeExchanged),
	AuthRequestFailed(AuthRequestFailed),
	AuthRequestSessionLinked(AuthRequestSessionLinked),
	AuthRequestSucceeded(AuthRequestSucceeded),
	DeviceAuthorizationAdded(DeviceAuthorizationAdded),
	DeviceAuthorizationApproved(DeviceAuthorizationApproved),
	DeviceAuthorizationCanceled(DeviceAuthorizationCanceled),
	DeviceAuthorizationDone(DeviceAuthorizationDone),
	ExecutionRemoved(ExecutionRemoved),
	ExecutionSet(ExecutionSet),
	IamIdpConfigAdded(IamIdpConfigAdded),
	IamIdpConfigChanged(IamIdpConfigChanged),
	IamIdpConfigDeactivated(IamIdpConfigDeactivated),
	IamIdpConfigReactivated(IamIdpConfigReactivated),
	IamIdpConfigRemoved(IamIdpConfigRemoved),
	IamIdpJwtConfigAdded(IamIdpJwtConfigAdded),
	IamIdpJwtConfigChanged(IamIdpJwtConfigChanged),
	IamIdpOidcConfigAdded(IamIdpOidcConfigAdded),
	IamIdpOidcConfigChanged(IamIdpOidcConfigChanged),
	IdpintentFailed(IdpintentFailed),
	IdpintentLdapSucceeded(IdpintentLdapSucceeded),
	IdpintentSamlRequested(IdpintentSamlRequested),
	IdpintentSamlSucceeded(IdpintentSamlSucceeded),
	IdpintentStarted(IdpintentStarted),
	IdpintentSucceeded(IdpintentSucceeded),
	InstanceIdpAppleAdded(InstanceIdpAppleAdded),
	InstanceIdpAppleChanged(InstanceIdpAppleChanged),
	InstanceIdpAzureAdded(InstanceIdpAzureAdded),
	InstanceIdpAzureChanged(InstanceIdpAzureChanged),
	InstanceIdpGithubAdded(InstanceIdpGithubAdded),
	InstanceIdpGithubChanged(InstanceIdpGithubChanged),
	InstanceIdpGithubEnterpriseAdded(InstanceIdpGithubEnterpriseAdded),
	InstanceIdpGithubEnterpriseChanged(InstanceIdpGithubEnterpriseChanged),
	InstanceIdpGitlabAdded(InstanceIdpGitlabAdded),
	InstanceIdpGitlabChanged(InstanceIdpGitlabChanged),
	InstanceIdpGitlabSelfHostedAdded(InstanceIdpGitlabSelfHostedAdded),
	InstanceIdpGitlabSelfHostedChanged(InstanceIdpGitlabSelfHostedChanged),
	InstanceIdpGoogleAdded(InstanceIdpGoogleAdded),
	InstanceIdpGoogleChanged(InstanceIdpGoogleChanged),
	InstanceIdpJwtAdded(InstanceIdpJwtAdded),
	InstanceIdpJwtChanged(InstanceIdpJwtChanged),
	InstanceIdpLdapV2Added(InstanceIdpLdapV2Added),
	InstanceIdpLdapV2Changed(InstanceIdpLdapV2Changed),
	InstanceIdpOauthAdded(InstanceIdpOauthAdded),
	InstanceIdpOauthChanged(InstanceIdpOauthChanged),
	InstanceIdpOidcAdded(InstanceIdpOidcAdded),
	InstanceIdpOidcChanged(InstanceIdpOidcChanged),
	InstanceIdpOidcMigratedAzure(InstanceIdpOidcMigratedAzure),
	InstanceIdpOidcMigratedGoogle(InstanceIdpOidcMigratedGoogle),
	InstanceIdpRemoved(InstanceIdpRemoved),
	InstanceIdpSamlAdded(InstanceIdpSamlAdded),
	InstanceIdpSamlChanged(InstanceIdpSamlChanged),
	OrgIdpConfigAdded(OrgIdpConfigAdded),
	OrgIdpConfigChanged(OrgIdpConfigChanged),
	OrgIdpConfigDeactivated(OrgIdpConfigDeactivated),
	OrgIdpConfigReactivated(OrgIdpConfigReactivated),
	OrgIdpConfigRemoved(OrgIdpConfigRemoved),
	UserDeactivated(UserDeactivated),
	UserDomainClaimed(UserDomainClaimed),
	UserDomainClaimedSent(UserDomainClaimedSent),
	UserGrantAdded(UserGrantAdded),
	UserGrantCascadeChanged(UserGrantCascadeChanged),
	UserGrantCascadeRemoved(UserGrantCascadeRemoved),
	UserGrantChanged(UserGrantChanged),
	UserGrantDeactivated(UserGrantDeactivated),
	UserGrantReactivated(UserGrantReactivated),
	UserGrantRemoved(UserGrantRemoved),
	UserHumanAdded(UserHumanAdded),
	UserHumanInitializationCheckFailed(UserHumanInitializationCheckFailed),
	UserHumanInitializationCheckSucceeded(UserHumanInitializationCheckSucceeded),
	UserHumanInitializationCodeAdded(UserHumanInitializationCodeAdded),
	UserHumanInitializationCodeSent(UserHumanInitializationCodeSent),
	UserHumanSelfregistered(UserHumanSelfregistered),
	UserHumanSignedOut(UserHumanSignedOut),
	UserImpersonated(UserImpersonated),
	UserLocked(UserLocked),
	UserReactivated(UserReactivated),
	UserRemoved(UserRemoved),
	UserTokenAdded(UserTokenAdded),
	UserTokenRemoved(UserTokenRemoved),
	UserTokenV2Added(UserTokenV2Added),
	UserUnlocked(UserUnlocked),
	UserUsernameChanged(UserUsernameChanged),
}
