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
pub struct Zitadeluserv1FirstNameQuery {
	#[serde(rename = "firstName")]
	first_name: Option<String>,
	/// defines which text equality method is used
	#[serde(rename = "method")]
	method: Option<models::V1TextQueryMethod>,
}

impl Zitadeluserv1FirstNameQuery {
	pub fn new() -> Zitadeluserv1FirstNameQuery {
		Zitadeluserv1FirstNameQuery { first_name: None, method: None }
	}

	pub fn set_first_name(&mut self, first_name: String) {
		self.first_name = Some(first_name);
	}

	pub fn with_first_name(mut self, first_name: String) -> Zitadeluserv1FirstNameQuery {
		self.first_name = Some(first_name);
		self
	}

	pub fn first_name(&self) -> Option<&String> {
		self.first_name.as_ref()
	}

	pub fn reset_first_name(&mut self) {
		self.first_name = None;
	}

	pub fn set_method(&mut self, method: models::V1TextQueryMethod) {
		self.method = Some(method);
	}

	pub fn with_method(mut self, method: models::V1TextQueryMethod) -> Zitadeluserv1FirstNameQuery {
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
