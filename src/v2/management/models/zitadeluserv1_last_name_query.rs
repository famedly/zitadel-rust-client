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
pub struct Zitadeluserv1LastNameQuery {
	#[serde(rename = "lastName")]
	last_name: Option<String>,
	/// defines which text equality method is used
	#[serde(rename = "method")]
	method: Option<models::V1TextQueryMethod>,
}

impl Zitadeluserv1LastNameQuery {
	pub fn new() -> Zitadeluserv1LastNameQuery {
		Zitadeluserv1LastNameQuery { last_name: None, method: None }
	}

	pub fn set_last_name(&mut self, last_name: String) {
		self.last_name = Some(last_name);
	}

	pub fn with_last_name(mut self, last_name: String) -> Zitadeluserv1LastNameQuery {
		self.last_name = Some(last_name);
		self
	}

	pub fn last_name(&self) -> Option<&String> {
		self.last_name.as_ref()
	}

	pub fn reset_last_name(&mut self) {
		self.last_name = None;
	}

	pub fn set_method(&mut self, method: models::V1TextQueryMethod) {
		self.method = Some(method);
	}

	pub fn with_method(mut self, method: models::V1TextQueryMethod) -> Zitadeluserv1LastNameQuery {
		self.method = Some(method);
		self
	}

	pub fn method(&self) -> Option<&models::V1TextQueryMethod> {
		self.method.as_ref()
	}

	pub fn reset_method(&mut self) {
		self.method = None;
	}
}
