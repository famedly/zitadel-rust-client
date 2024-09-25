/*
 * User Service
 *
 * This API is intended to manage users in a ZITADEL instance.
 *
 * OpenAPI spec version: 2.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{Deserialize, Serialize};

use crate::v2::users::{models, pagination::PaginationResponse};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ListUsersResponse {
	#[serde(rename = "details")]
	details: Option<models::ListDetails>,
	#[serde(rename = "sortingColumn")]
	sorting_column: Option<models::UserFieldName>,
	#[serde(rename = "result")]
	result: Option<Vec<models::User>>,
}

impl ListUsersResponse {
	pub fn new() -> ListUsersResponse {
		ListUsersResponse { details: None, sorting_column: None, result: None }
	}

	pub fn set_details(&mut self, details: models::ListDetails) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: models::ListDetails) -> ListUsersResponse {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::ListDetails> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_sorting_column(&mut self, sorting_column: models::UserFieldName) {
		self.sorting_column = Some(sorting_column);
	}

	pub fn with_sorting_column(
		mut self,
		sorting_column: models::UserFieldName,
	) -> ListUsersResponse {
		self.sorting_column = Some(sorting_column);
		self
	}

	pub fn sorting_column(&self) -> Option<&models::UserFieldName> {
		self.sorting_column.as_ref()
	}

	pub fn reset_sorting_column(&mut self) {
		self.sorting_column = None;
	}

	pub fn set_result(&mut self, result: Vec<models::User>) {
		self.result = Some(result);
	}

	pub fn with_result(mut self, result: Vec<models::User>) -> ListUsersResponse {
		self.result = Some(result);
		self
	}

	pub fn result(&self) -> Option<&Vec<models::User>> {
		self.result.as_ref()
	}

	pub fn reset_result(&mut self) {
		self.result = None;
	}
}

impl PaginationResponse for ListUsersResponse {
	type Item = models::User;
	fn take_result(&mut self) -> Option<Vec<Self::Item>> {
		self.result.take()
	}
}
