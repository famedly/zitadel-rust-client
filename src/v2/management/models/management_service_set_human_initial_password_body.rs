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
pub struct ManagementServiceSetHumanInitialPasswordBody {
	#[serde(rename = "password")]
	password: Option<String>,
}

impl ManagementServiceSetHumanInitialPasswordBody {
	pub fn new() -> ManagementServiceSetHumanInitialPasswordBody {
		ManagementServiceSetHumanInitialPasswordBody { password: None }
	}

	pub fn set_password(&mut self, password: String) {
		self.password = Some(password);
	}

	pub fn with_password(
		mut self,
		password: String,
	) -> ManagementServiceSetHumanInitialPasswordBody {
		self.password = Some(password);
		self
	}

	pub fn password(&self) -> Option<&String> {
		self.password.as_ref()
	}

	pub fn reset_password(&mut self) {
		self.password = None;
	}
}
