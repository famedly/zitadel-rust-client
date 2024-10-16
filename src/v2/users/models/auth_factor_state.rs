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
pub enum AuthFactorState {
	#[serde(rename = "AUTH_FACTOR_STATE_UNSPECIFIED")]
	Unspecified,
	#[serde(rename = "AUTH_FACTOR_STATE_NOT_READY")]
	NotReady,
	#[serde(rename = "AUTH_FACTOR_STATE_READY")]
	Ready,
	#[serde(rename = "AUTH_FACTOR_STATE_REMOVED")]
	Removed,
}

impl std::fmt::Display for AuthFactorState {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Unspecified => write!(f, "AUTH_FACTOR_STATE_UNSPECIFIED"),
			Self::NotReady => write!(f, "AUTH_FACTOR_STATE_NOT_READY"),
			Self::Ready => write!(f, "AUTH_FACTOR_STATE_READY"),
			Self::Removed => write!(f, "AUTH_FACTOR_STATE_REMOVED"),
		}
	}
}

impl Default for AuthFactorState {
	fn default() -> AuthFactorState {
		Self::Unspecified
	}
}
