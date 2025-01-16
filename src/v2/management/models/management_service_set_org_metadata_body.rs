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
pub struct ManagementServiceSetOrgMetadataBody {
	/// The value has to be base64 encoded.
	#[serde(rename = "value")]
	value: Option<String>,
}

impl ManagementServiceSetOrgMetadataBody {
	pub fn new() -> ManagementServiceSetOrgMetadataBody {
		ManagementServiceSetOrgMetadataBody { value: None }
	}

	pub fn set_value(&mut self, value: String) {
		self.value = Some(value);
	}

	pub fn with_value(mut self, value: String) -> ManagementServiceSetOrgMetadataBody {
		self.value = Some(value);
		self
	}

	pub fn value(&self) -> Option<&String> {
		self.value.as_ref()
	}

	pub fn reset_value(&mut self) {
		self.value = None;
	}
}
