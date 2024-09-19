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
pub enum AccessTokenType {
	#[serde(rename = "ACCESS_TOKEN_TYPE_BEARER")]
	Bearer,
	#[serde(rename = "ACCESS_TOKEN_TYPE_JWT")]
	Jwt,
}

impl std::fmt::Display for AccessTokenType {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Bearer => write!(f, "ACCESS_TOKEN_TYPE_BEARER"),
			Self::Jwt => write!(f, "ACCESS_TOKEN_TYPE_JWT"),
		}
	}
}

impl Default for AccessTokenType {
	fn default() -> AccessTokenType {
		Self::Bearer
	}
}