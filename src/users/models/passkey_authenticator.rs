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
pub enum PasskeyAuthenticator {
	#[serde(rename = "PASSKEY_AUTHENTICATOR_UNSPECIFIED")]
	Unspecified,
	#[serde(rename = "PASSKEY_AUTHENTICATOR_PLATFORM")]
	Platform,
	#[serde(rename = "PASSKEY_AUTHENTICATOR_CROSS_PLATFORM")]
	CrossPlatform,
}

impl std::fmt::Display for PasskeyAuthenticator {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Unspecified => write!(f, "PASSKEY_AUTHENTICATOR_UNSPECIFIED"),
			Self::Platform => write!(f, "PASSKEY_AUTHENTICATOR_PLATFORM"),
			Self::CrossPlatform => write!(f, "PASSKEY_AUTHENTICATOR_CROSS_PLATFORM"),
		}
	}
}

impl Default for PasskeyAuthenticator {
	fn default() -> PasskeyAuthenticator {
		Self::Unspecified
	}
}
