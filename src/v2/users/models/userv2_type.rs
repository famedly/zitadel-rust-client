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
pub enum Userv2Type {
	#[serde(rename = "TYPE_UNSPECIFIED")]
	Unspecified,
	#[serde(rename = "TYPE_HUMAN")]
	Human,
	#[serde(rename = "TYPE_MACHINE")]
	Machine,
}

impl std::fmt::Display for Userv2Type {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Unspecified => write!(f, "TYPE_UNSPECIFIED"),
			Self::Human => write!(f, "TYPE_HUMAN"),
			Self::Machine => write!(f, "TYPE_MACHINE"),
		}
	}
}

impl Default for Userv2Type {
	fn default() -> Userv2Type {
		Self::Unspecified
	}
}
