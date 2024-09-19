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

use crate::v2::users::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeQuery {
	/// the type of the user
	#[serde(rename = "type")]
	user_type: models::Userv2Type,
}

impl TypeQuery {
	/// Query for users with a specific type.
	pub fn new(user_type: models::Userv2Type) -> TypeQuery {
		TypeQuery { user_type }
	}

	pub fn set_user_type(&mut self, user_type: models::Userv2Type) {
		self.user_type = user_type;
	}

	pub fn with_user_type(mut self, user_type: models::Userv2Type) -> TypeQuery {
		self.user_type = user_type;
		self
	}

	pub fn user_type(&self) -> &models::Userv2Type {
		&self.user_type
	}
}