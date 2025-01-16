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
pub struct V1ListAppsResponse {
	#[serde(rename = "details")]
	details: Option<models::V1ListDetails>,
	#[serde(rename = "result")]
	result: Option<Vec<models::V1App>>,
}

impl V1ListAppsResponse {
	pub fn new() -> V1ListAppsResponse {
		V1ListAppsResponse { details: None, result: None }
	}

	pub fn set_details(&mut self, details: models::V1ListDetails) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: models::V1ListDetails) -> V1ListAppsResponse {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::V1ListDetails> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_result(&mut self, result: Vec<models::V1App>) {
		self.result = Some(result);
	}

	pub fn with_result(mut self, result: Vec<models::V1App>) -> V1ListAppsResponse {
		self.result = Some(result);
		self
	}

	pub fn result(&self) -> Option<&Vec<models::V1App>> {
		self.result.as_ref()
	}

	pub fn reset_result(&mut self) {
		self.result = None;
	}
}
