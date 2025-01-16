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
pub struct ManagementServiceUpdateProjectGrantBody {
	#[serde(rename = "roleKeys")]
	role_keys: Option<Vec<String>>,
}

impl ManagementServiceUpdateProjectGrantBody {
	pub fn new() -> ManagementServiceUpdateProjectGrantBody {
		ManagementServiceUpdateProjectGrantBody { role_keys: None }
	}

	pub fn set_role_keys(&mut self, role_keys: Vec<String>) {
		self.role_keys = Some(role_keys);
	}

	pub fn with_role_keys(
		mut self,
		role_keys: Vec<String>,
	) -> ManagementServiceUpdateProjectGrantBody {
		self.role_keys = Some(role_keys);
		self
	}

	pub fn role_keys(&self) -> Option<&Vec<String>> {
		self.role_keys.as_ref()
	}

	pub fn reset_role_keys(&mut self) {
		self.role_keys = None;
	}
}
