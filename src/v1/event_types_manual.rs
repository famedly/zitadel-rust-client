#![allow(non_snake_case, missing_docs, clippy::missing_docs_in_private_items)]

pub use std::net::IpAddr as IP;

pub use serde::{self, Deserialize, Serialize};
use serde_repr::*;
pub use time::OffsetDateTime as Time;
pub use url::Url as URL;

pub type Tag = String;
pub type Header = std::collections::HashMap<String, String>;
pub type Map<K, V> = std::collections::HashMap<K, V>;

// FIXME: go:generate types

// CryptoValue found in:  internal/crypto/crypto.go
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CryptoValue {
	pub CryptoType: String,
	pub Algorithm: String,
	pub KeyID: String,
	pub Encrypted: Vec<u8>,
}

// DeviceAuthCanceled found in: internal/domain/device_auth.go
pub type DeviceAuthCanceled = String;

// DeviceAuthState found in: internal/domain/device_auth.go
//go:generate stringer -type=DeviceAuthState -linecomment
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i32)]
pub enum DeviceAuthState {
	DeviceAuthStateUndefined = 0,
	DeviceAuthStateInitiated,
	DeviceAuthStateApproved,
	DeviceAuthStateDenied,
	DeviceAuthStateExpired,
	DeviceAuthStateDone,
}

// Duration found in: internal/database/type.go
pub type Duration = std::time::Duration; // time.Duration FIXME

// EmailAddress found in: internal/domain/human_email.go
pub type EmailAddress = String;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i32)]
pub enum Gender {
	GenderUnspecified = 0,
	GenderFemale = 1,
	GenderMale = 2,
	GenderDiverse = 3,
}

// IDPConfigStylingType found in: internal/domain/idp_config.go
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i32)]
pub enum IDPConfigStylingType {
	IDPConfigStylingTypeUnspecified = 0,
	IDPConfigStylingTypeGoogle = 1,
}

// IDPConfigType found in: internal/domain/idp_config.go
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i32)]
pub enum IDPConfigType {
	IDPConfigTypeOIDC = 0,
	IDPConfigTypeSAML = 1,
	IDPConfigTypeJWT = 2,

	// count is for validation
	// idpConfigTypeCount
	IDPConfigTypeUnspecified = -1,
}

// OIDCCodeChallenge found in:  internal/domain/oidc_code_challenge.go
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OIDCCodeChallenge {
	pub Challenge: String,
	pub Method: OIDCCodeChallengeMethod,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i32)]
pub enum OIDCCodeChallengeMethod {
	CodeChallengeMethodPlain = 0,
	CodeChallengeMethodS256 = 1,
}

// OIDCErrorReason found in: internal/domain/oidc_error_reason.go
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i32)]
pub enum OIDCErrorReason {
	OIDCErrorReasonUnspecified = 0,
	OIDCErrorReasonInvalidRequest,
	OIDCErrorReasonUnauthorizedClient,
	OIDCErrorReasonAccessDenied,
	OIDCErrorReasonUnsupportedResponseType,
	OIDCErrorReasonInvalidScope,
	OIDCErrorReasonServerError,
	OIDCErrorReasonTemporaryUnavailable,
	OIDCErrorReasonInteractionRequired,
	OIDCErrorReasonLoginRequired,
	OIDCErrorReasonAccountSelectionRequired,
	OIDCErrorReasonConsentRequired,
	OIDCErrorReasonInvalidRequestURI,
	OIDCErrorReasonInvalidRequestObject,
	OIDCErrorReasonRequestNotSupported,
	OIDCErrorReasonRequestURINotSupported,
	OIDCErrorReasonRegistrationNotSupported,
	OIDCErrorReasonInvalidGrant,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i32)]
pub enum OIDCResponseMode {
	OIDCResponseModeUnspecified = 0,
	OIDCResponseModeQuery,
	OIDCResponseModeFragment,
	OIDCResponseModeFormPost,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i32)]
pub enum OIDCResponseType {
	OIDCResponseTypeCode = 0,
	OIDCResponseTypeIDToken,
	OIDCResponseTypeIDTokenToken,
}

// PhoneNumber found in: internal/domain/human_phone.go
pub type PhoneNumber = String;

// Prompt found in: internal/domain/auth_request.go
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i32)]
pub enum Prompt {
	PromptUnspecified = 0,
	PromptNone,
	PromptLogin,
	PromptConsent,
	PromptSelectAccount,
	PromptCreate,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenActor {
	pub actor: Option<Box<TokenActor>>,
	pub user_id: Option<String>,
	pub issuer: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i32)]
pub enum TokenReason {
	TokenReasonUnspecified = 0,
	TokenReasonAuthRequest,
	TokenReasonRefresh,
	TokenReasonJWTProfile,
	TokenReasonClientCredentials,
	TokenReasonExchange,
	TokenReasonImpersonation,
}

// UserAgent found in: internal/domain/user_agent.go
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserAgent {
	#[serde(rename = "fingerprint_id")]
	pub FingerprintID: Option<String>,
	#[serde(rename = "ip")]
	pub IP: Option<IP>,
	#[serde(rename = "description")]
	pub Description: Option<String>,
	#[serde(rename = "header")]
	pub Header: Option<Header>,
}

// UserAuthMethodType found in: internal/domain/user.go
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i32)]
pub enum UserAuthMethodType {
	UserAuthMethodTypeUnspecified = 0,
	UserAuthMethodTypeTOTP,
	UserAuthMethodTypeU2F,
	UserAuthMethodTypePasswordless,
	UserAuthMethodTypePassword,
	UserAuthMethodTypeIDP,
	UserAuthMethodTypeOTPSMS,
	UserAuthMethodTypeOTPEmail,
	UserAuthMethodTypeOTP, // generic OTP when parsing AMR from OIDC
	UserAuthMethodTypePrivateKey,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum AutoLinkingOption {
	AutoLinkingOptionUnspecified = 0,
	AutoLinkingOptionUsername = 1,
	AutoLinkingOptionEmail = 2,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum SAMLNameIDFormat {
	SAMLNameIDFormatUnspecified = 0,
	SAMLNameIDFormatEmailAddress,
	SAMLNameIDFormatPersistent,
	SAMLNameIDFormatTransient,
}

// LDAPAttributeChanges found in: internal/repository/idp/go
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LDAPAttributeChanges {
	#[serde(rename = "idAttribute")]
	pub IDAttribute: Option<String>,
	#[serde(rename = "firstNameAttribute")]
	pub FirstNameAttribute: Option<String>,
	#[serde(rename = "lastNameAttribute")]
	pub LastNameAttribute: Option<String>,
	#[serde(rename = "displayNameAttribute")]
	pub DisplayNameAttribute: Option<String>,
	#[serde(rename = "nickNameAttribute")]
	pub NickNameAttribute: Option<String>,
	#[serde(rename = "preferredUsernameAttribute")]
	pub PreferredUsernameAttribute: Option<String>,
	#[serde(rename = "emailAttribute")]
	pub EmailAttribute: Option<String>,
	#[serde(rename = "emailVerifiedAttribute")]
	pub EmailVerifiedAttribute: Option<String>,
	#[serde(rename = "phoneAttribute")]
	pub PhoneAttribute: Option<String>,
	#[serde(rename = "phoneVerifiedAttribute")]
	pub PhoneVerifiedAttribute: Option<String>,
	#[serde(rename = "preferredLanguageAttribute")]
	pub PreferredLanguageAttribute: Option<String>,
	#[serde(rename = "avatarURLAttribute")]
	pub AvatarURLAttribute: Option<String>,
	#[serde(rename = "profileAttribute")]
	pub ProfileAttribute: Option<String>,
}

// LDAPAttributes found in: internal/repository/idp/go
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LDAPAttributes {
	#[serde(rename = "idAttribute")]
	pub IDAttribute: Option<String>,
	#[serde(rename = "firstNameAttribute")]
	pub FirstNameAttribute: Option<String>,
	#[serde(rename = "lastNameAttribute")]
	pub LastNameAttribute: Option<String>,
	#[serde(rename = "displayNameAttribute")]
	pub DisplayNameAttribute: Option<String>,
	#[serde(rename = "nickNameAttribute")]
	pub NickNameAttribute: Option<String>,
	#[serde(rename = "preferredUsernameAttribute")]
	pub PreferredUsernameAttribute: Option<String>,
	#[serde(rename = "emailAttribute")]
	pub EmailAttribute: Option<String>,
	#[serde(rename = "emailVerifiedAttribute")]
	pub EmailVerifiedAttribute: Option<String>,
	#[serde(rename = "phoneAttribute")]
	pub PhoneAttribute: Option<String>,
	#[serde(rename = "phoneVerifiedAttribute")]
	pub PhoneVerifiedAttribute: Option<String>,
	#[serde(rename = "preferredLanguageAttribute")]
	pub PreferredLanguageAttribute: Option<String>,
	#[serde(rename = "avatarURLAttribute")]
	pub AvatarURLAttribute: Option<String>,
	#[serde(rename = "profileAttribute")]
	pub ProfileAttribute: Option<String>,
}

// OIDCMappingField found in: internal/domain/oidc_mapping_field.go
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i32)]
pub enum OIDCMappingField {
	OIDCMappingFieldUnspecified = 0,
	OIDCMappingFieldPreferredLoginName,
	OIDCMappingFieldEmail,
}
