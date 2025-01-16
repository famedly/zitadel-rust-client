/*
 * Management API
 *
 * The management API is as the name states the interface where systems can
 * mutate IAM objects like organizations, projects, clients, users and so on
 * if they have the necessary access rights.
 *
 * OpenAPI spec version: 1.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

use crate::v2::management::models;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct V1GetUserByLoginNameGlobalResponse {
	#[serde(rename = "user")]
	user: Option<models::V1User>,
}

impl V1GetUserByLoginNameGlobalResponse {
	pub fn new() -> V1GetUserByLoginNameGlobalResponse {
		V1GetUserByLoginNameGlobalResponse { user: None }
	}

	pub fn set_user(&mut self, user: models::V1User) {
		self.user = Some(user);
	}

	pub fn with_user(mut self, user: models::V1User) -> V1GetUserByLoginNameGlobalResponse {
		self.user = Some(user);
		self
	}

	pub fn user(&self) -> Option<&models::V1User> {
		self.user.as_ref()
	}

	pub fn reset_user(&mut self) {
		self.user = None;
	}
}
