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
pub struct V1AuthFactorU2F {
	#[serde(rename = "id")]
	id: Option<String>,
	#[serde(rename = "name")]
	name: Option<String>,
}

impl V1AuthFactorU2F {
	pub fn new() -> V1AuthFactorU2F {
		V1AuthFactorU2F { id: None, name: None }
	}

	pub fn set_id(&mut self, id: String) {
		self.id = Some(id);
	}

	pub fn with_id(mut self, id: String) -> V1AuthFactorU2F {
		self.id = Some(id);
		self
	}

	pub fn id(&self) -> Option<&String> {
		self.id.as_ref()
	}

	pub fn reset_id(&mut self) {
		self.id = None;
	}

	pub fn set_name(&mut self, name: String) {
		self.name = Some(name);
	}

	pub fn with_name(mut self, name: String) -> V1AuthFactorU2F {
		self.name = Some(name);
		self
	}

	pub fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	pub fn reset_name(&mut self) {
		self.name = None;
	}
}
