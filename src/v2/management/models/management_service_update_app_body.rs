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
pub struct ManagementServiceUpdateAppBody {
	#[serde(rename = "name")]
	name: String,
}

impl ManagementServiceUpdateAppBody {
	pub fn new(name: String) -> ManagementServiceUpdateAppBody {
		ManagementServiceUpdateAppBody { name: name }
	}

	pub fn set_name(&mut self, name: String) {
		self.name = name;
	}

	pub fn with_name(mut self, name: String) -> ManagementServiceUpdateAppBody {
		self.name = name;
		self
	}

	pub fn name(&self) -> &String {
		&self.name
	}
}
