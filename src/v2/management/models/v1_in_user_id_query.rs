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
pub struct V1InUserIdQuery {
	/// the ids of the users to include
	#[serde(rename = "userIds")]
	user_ids: Option<Vec<String>>,
}

impl V1InUserIdQuery {
	pub fn new() -> V1InUserIdQuery {
		V1InUserIdQuery { user_ids: None }
	}

	pub fn set_user_ids(&mut self, user_ids: Vec<String>) {
		self.user_ids = Some(user_ids);
	}

	pub fn with_user_ids(mut self, user_ids: Vec<String>) -> V1InUserIdQuery {
		self.user_ids = Some(user_ids);
		self
	}

	pub fn user_ids(&self) -> Option<&Vec<String>> {
		self.user_ids.as_ref()
	}

	pub fn reset_user_ids(&mut self) {
		self.user_ids = None;
	}
}
