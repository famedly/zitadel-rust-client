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
pub struct ManagementServiceUpdateActionBody {
	#[serde(rename = "name")]
	name: String,
	/// Javascript code that should be executed
	#[serde(rename = "script")]
	script: String,
	/// after which time the action will be terminated if not finished
	#[serde(rename = "timeout")]
	timeout: Option<String>,
	/// when true, the next action will be called even if this action fails
	#[serde(rename = "allowedToFail")]
	allowed_to_fail: Option<bool>,
}

impl ManagementServiceUpdateActionBody {
	pub fn new(name: String, script: String) -> ManagementServiceUpdateActionBody {
		ManagementServiceUpdateActionBody { name, script, timeout: None, allowed_to_fail: None }
	}

	pub fn set_name(&mut self, name: String) {
		self.name = name;
	}

	pub fn with_name(mut self, name: String) -> ManagementServiceUpdateActionBody {
		self.name = name;
		self
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn set_script(&mut self, script: String) {
		self.script = script;
	}

	pub fn with_script(mut self, script: String) -> ManagementServiceUpdateActionBody {
		self.script = script;
		self
	}

	pub fn script(&self) -> &String {
		&self.script
	}

	pub fn set_timeout(&mut self, timeout: String) {
		self.timeout = Some(timeout);
	}

	pub fn with_timeout(mut self, timeout: String) -> ManagementServiceUpdateActionBody {
		self.timeout = Some(timeout);
		self
	}

	pub fn timeout(&self) -> Option<&String> {
		self.timeout.as_ref()
	}

	pub fn reset_timeout(&mut self) {
		self.timeout = None;
	}

	pub fn set_allowed_to_fail(&mut self, allowed_to_fail: bool) {
		self.allowed_to_fail = Some(allowed_to_fail);
	}

	pub fn with_allowed_to_fail(
		mut self,
		allowed_to_fail: bool,
	) -> ManagementServiceUpdateActionBody {
		self.allowed_to_fail = Some(allowed_to_fail);
		self
	}

	pub fn allowed_to_fail(&self) -> Option<&bool> {
		self.allowed_to_fail.as_ref()
	}

	pub fn reset_allowed_to_fail(&mut self) {
		self.allowed_to_fail = None;
	}
}
