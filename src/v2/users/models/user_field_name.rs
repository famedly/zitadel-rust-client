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
pub enum UserFieldName {
	#[serde(rename = "USER_FIELD_NAME_UNSPECIFIED")]
	Unspecified,
	#[serde(rename = "USER_FIELD_NAME_USER_NAME")]
	UserName,
	#[serde(rename = "USER_FIELD_NAME_FIRST_NAME")]
	FirstName,
	#[serde(rename = "USER_FIELD_NAME_LAST_NAME")]
	LastName,
	#[serde(rename = "USER_FIELD_NAME_NICK_NAME")]
	NickName,
	#[serde(rename = "USER_FIELD_NAME_DISPLAY_NAME")]
	DisplayName,
	#[serde(rename = "USER_FIELD_NAME_EMAIL")]
	Email,
	#[serde(rename = "USER_FIELD_NAME_STATE")]
	State,
	#[serde(rename = "USER_FIELD_NAME_TYPE")]
	Type,
	#[serde(rename = "USER_FIELD_NAME_CREATION_DATE")]
	CreationDate,
}

impl std::fmt::Display for UserFieldName {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Unspecified => write!(f, "USER_FIELD_NAME_UNSPECIFIED"),
			Self::UserName => write!(f, "USER_FIELD_NAME_USER_NAME"),
			Self::FirstName => write!(f, "USER_FIELD_NAME_FIRST_NAME"),
			Self::LastName => write!(f, "USER_FIELD_NAME_LAST_NAME"),
			Self::NickName => write!(f, "USER_FIELD_NAME_NICK_NAME"),
			Self::DisplayName => write!(f, "USER_FIELD_NAME_DISPLAY_NAME"),
			Self::Email => write!(f, "USER_FIELD_NAME_EMAIL"),
			Self::State => write!(f, "USER_FIELD_NAME_STATE"),
			Self::Type => write!(f, "USER_FIELD_NAME_TYPE"),
			Self::CreationDate => write!(f, "USER_FIELD_NAME_CREATION_DATE"),
		}
	}
}

impl Default for UserFieldName {
	fn default() -> UserFieldName {
		Self::Unspecified
	}
}
