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
pub struct V1GetProjectByIdResponse {
	#[serde(rename = "project")]
	project: Option<models::V1Project>,
}

impl V1GetProjectByIdResponse {
	pub fn new() -> V1GetProjectByIdResponse {
		V1GetProjectByIdResponse { project: None }
	}

	pub fn set_project(&mut self, project: models::V1Project) {
		self.project = Some(project);
	}

	pub fn with_project(mut self, project: models::V1Project) -> V1GetProjectByIdResponse {
		self.project = Some(project);
		self
	}

	pub fn project(&self) -> Option<&models::V1Project> {
		self.project.as_ref()
	}

	pub fn reset_project(&mut self) {
		self.project = None;
	}
}
