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

use crate::users::models;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserNameQuery {
	#[serde(rename = "userName")]
	user_name: String,
	/// defines which text equality method is used
	#[serde(rename = "method")]
	method: Option<models::TextQueryMethod>,
}

impl UserNameQuery {
	/// Query for users with a specific user name.
	pub fn new(user_name: String) -> UserNameQuery {
		UserNameQuery { user_name, method: None }
	}

	pub fn set_user_name(&mut self, user_name: String) {
		self.user_name = user_name;
	}

	pub fn with_user_name(mut self, user_name: String) -> UserNameQuery {
		self.user_name = user_name;
		self
	}

	pub fn user_name(&self) -> &String {
		&self.user_name
	}

	pub fn set_method(&mut self, method: models::TextQueryMethod) {
		self.method = Some(method);
	}

	pub fn with_method(mut self, method: models::TextQueryMethod) -> UserNameQuery {
		self.method = Some(method);
		self
	}

	pub fn method(&self) -> Option<&models::TextQueryMethod> {
		self.method.as_ref()
	}

	pub fn reset_method(&mut self) {
		self.method = None;
	}
}
