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
pub struct V1ListUsersResponse {
	#[serde(rename = "details")]
	details: Option<models::V1ListDetails>,
	#[serde(rename = "sortingColumn")]
	sorting_column: Option<models::V1UserFieldName>,
	#[serde(rename = "result")]
	result: Option<Vec<models::V1User>>,
}

impl V1ListUsersResponse {
	pub fn new() -> V1ListUsersResponse {
		V1ListUsersResponse { details: None, sorting_column: None, result: None }
	}

	pub fn set_details(&mut self, details: models::V1ListDetails) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: models::V1ListDetails) -> V1ListUsersResponse {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::V1ListDetails> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_sorting_column(&mut self, sorting_column: models::V1UserFieldName) {
		self.sorting_column = Some(sorting_column);
	}

	pub fn with_sorting_column(
		mut self,
		sorting_column: models::V1UserFieldName,
	) -> V1ListUsersResponse {
		self.sorting_column = Some(sorting_column);
		self
	}

	pub fn sorting_column(&self) -> Option<&models::V1UserFieldName> {
		self.sorting_column.as_ref()
	}

	pub fn reset_sorting_column(&mut self) {
		self.sorting_column = None;
	}

	pub fn set_result(&mut self, result: Vec<models::V1User>) {
		self.result = Some(result);
	}

	pub fn with_result(mut self, result: Vec<models::V1User>) -> V1ListUsersResponse {
		self.result = Some(result);
		self
	}

	pub fn result(&self) -> Option<&Vec<models::V1User>> {
		self.result.as_ref()
	}

	pub fn reset_result(&mut self) {
		self.result = None;
	}
}
