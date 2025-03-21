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
pub struct V1UserGrantProjectNameQuery {
	#[serde(rename = "projectName")]
	project_name: Option<String>,
	#[serde(rename = "method")]
	method: Option<models::V1TextQueryMethod>,
}

impl V1UserGrantProjectNameQuery {
	pub fn new() -> V1UserGrantProjectNameQuery {
		V1UserGrantProjectNameQuery { project_name: None, method: None }
	}

	pub fn set_project_name(&mut self, project_name: String) {
		self.project_name = Some(project_name);
	}

	pub fn with_project_name(mut self, project_name: String) -> V1UserGrantProjectNameQuery {
		self.project_name = Some(project_name);
		self
	}

	pub fn project_name(&self) -> Option<&String> {
		self.project_name.as_ref()
	}

	pub fn reset_project_name(&mut self) {
		self.project_name = None;
	}

	pub fn set_method(&mut self, method: models::V1TextQueryMethod) {
		self.method = Some(method);
	}

	pub fn with_method(mut self, method: models::V1TextQueryMethod) -> V1UserGrantProjectNameQuery {
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
