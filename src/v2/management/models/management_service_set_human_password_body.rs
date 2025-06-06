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
pub struct ManagementServiceSetHumanPasswordBody {
	#[serde(rename = "password")]
	password: String,
	#[serde(rename = "noChangeRequired")]
	no_change_required: Option<bool>,
}

impl ManagementServiceSetHumanPasswordBody {
	pub fn new(password: String) -> ManagementServiceSetHumanPasswordBody {
		ManagementServiceSetHumanPasswordBody { password: password, no_change_required: None }
	}

	pub fn set_password(&mut self, password: String) {
		self.password = password;
	}

	pub fn with_password(mut self, password: String) -> ManagementServiceSetHumanPasswordBody {
		self.password = password;
		self
	}

	pub fn password(&self) -> &String {
		&self.password
	}

	pub fn set_no_change_required(&mut self, no_change_required: bool) {
		self.no_change_required = Some(no_change_required);
	}

	pub fn with_no_change_required(
		mut self,
		no_change_required: bool,
	) -> ManagementServiceSetHumanPasswordBody {
		self.no_change_required = Some(no_change_required);
		self
	}

	pub fn no_change_required(&self) -> Option<&bool> {
		self.no_change_required.as_ref()
	}

	pub fn reset_no_change_required(&mut self) {
		self.no_change_required = None;
	}
}
