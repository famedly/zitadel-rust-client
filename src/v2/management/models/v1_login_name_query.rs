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
pub struct V1LoginNameQuery {
	#[serde(rename = "loginName")]
	login_name: Option<String>,
	/// defines which text equality method is used
	#[serde(rename = "method")]
	method: Option<models::V1TextQueryMethod>,
}

impl V1LoginNameQuery {
	pub fn new() -> V1LoginNameQuery {
		V1LoginNameQuery { login_name: None, method: None }
	}

	pub fn set_login_name(&mut self, login_name: String) {
		self.login_name = Some(login_name);
	}

	pub fn with_login_name(mut self, login_name: String) -> V1LoginNameQuery {
		self.login_name = Some(login_name);
		self
	}

	pub fn login_name(&self) -> Option<&String> {
		self.login_name.as_ref()
	}

	pub fn reset_login_name(&mut self) {
		self.login_name = None;
	}

	pub fn set_method(&mut self, method: models::V1TextQueryMethod) {
		self.method = Some(method);
	}

	pub fn with_method(mut self, method: models::V1TextQueryMethod) -> V1LoginNameQuery {
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
