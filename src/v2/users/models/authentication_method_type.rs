/*
 * User Service
 *
 * This API is intended to manage users in a ZITADEL instance.
 *
 * OpenAPI spec version: 2.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthenticationMethodType {
	#[serde(rename = "AUTHENTICATION_METHOD_TYPE_UNSPECIFIED")]
	Unspecified,
	#[serde(rename = "AUTHENTICATION_METHOD_TYPE_PASSWORD")]
	Password,
	#[serde(rename = "AUTHENTICATION_METHOD_TYPE_PASSKEY")]
	Passkey,
	#[serde(rename = "AUTHENTICATION_METHOD_TYPE_IDP")]
	Idp,
	#[serde(rename = "AUTHENTICATION_METHOD_TYPE_TOTP")]
	Totp,
	#[serde(rename = "AUTHENTICATION_METHOD_TYPE_U2F")]
	U2F,
	#[serde(rename = "AUTHENTICATION_METHOD_TYPE_OTP_SMS")]
	OtpSms,
	#[serde(rename = "AUTHENTICATION_METHOD_TYPE_OTP_EMAIL")]
	OtpEmail,
}

impl std::fmt::Display for AuthenticationMethodType {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Unspecified => write!(f, "AUTHENTICATION_METHOD_TYPE_UNSPECIFIED"),
			Self::Password => write!(f, "AUTHENTICATION_METHOD_TYPE_PASSWORD"),
			Self::Passkey => write!(f, "AUTHENTICATION_METHOD_TYPE_PASSKEY"),
			Self::Idp => write!(f, "AUTHENTICATION_METHOD_TYPE_IDP"),
			Self::Totp => write!(f, "AUTHENTICATION_METHOD_TYPE_TOTP"),
			Self::U2F => write!(f, "AUTHENTICATION_METHOD_TYPE_U2F"),
			Self::OtpSms => write!(f, "AUTHENTICATION_METHOD_TYPE_OTP_SMS"),
			Self::OtpEmail => write!(f, "AUTHENTICATION_METHOD_TYPE_OTP_EMAIL"),
		}
	}
}

impl Default for AuthenticationMethodType {
	fn default() -> AuthenticationMethodType {
		Self::Unspecified
	}
}