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
pub enum UserState {
	#[serde(rename = "USER_STATE_UNSPECIFIED")]
	Unspecified,
	#[serde(rename = "USER_STATE_ACTIVE")]
	Active,
	#[serde(rename = "USER_STATE_INACTIVE")]
	Inactive,
	#[serde(rename = "USER_STATE_DELETED")]
	Deleted,
	#[serde(rename = "USER_STATE_LOCKED")]
	Locked,
	#[serde(rename = "USER_STATE_INITIAL")]
	Initial,
}

impl std::fmt::Display for UserState {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Unspecified => write!(f, "USER_STATE_UNSPECIFIED"),
			Self::Active => write!(f, "USER_STATE_ACTIVE"),
			Self::Inactive => write!(f, "USER_STATE_INACTIVE"),
			Self::Deleted => write!(f, "USER_STATE_DELETED"),
			Self::Locked => write!(f, "USER_STATE_LOCKED"),
			Self::Initial => write!(f, "USER_STATE_INITIAL"),
		}
	}
}

impl Default for UserState {
	fn default() -> UserState {
		Self::Unspecified
	}
}
