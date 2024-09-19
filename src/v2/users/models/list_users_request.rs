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

use crate::v2::users::models;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListUsersRequest {
	#[serde(rename = "query")]
	query: Option<models::ListQuery>,
	#[serde(rename = "sortingColumn")]
	sorting_column: Option<models::UserFieldName>,
	#[serde(rename = "queries")]
	queries: Option<Vec<models::SearchQuery>>,
}

impl ListUsersRequest {
	pub fn new() -> ListUsersRequest {
		ListUsersRequest { query: None, sorting_column: None, queries: None }
	}

	pub fn set_query(&mut self, query: models::ListQuery) {
		self.query = Some(query);
	}

	pub fn with_query(mut self, query: models::ListQuery) -> ListUsersRequest {
		self.query = Some(query);
		self
	}

	pub fn query(&self) -> Option<&models::ListQuery> {
		self.query.as_ref()
	}

	pub fn reset_query(&mut self) {
		self.query = None;
	}

	pub fn set_sorting_column(&mut self, sorting_column: models::UserFieldName) {
		self.sorting_column = Some(sorting_column);
	}

	pub fn with_sorting_column(
		mut self,
		sorting_column: models::UserFieldName,
	) -> ListUsersRequest {
		self.sorting_column = Some(sorting_column);
		self
	}

	pub fn sorting_column(&self) -> Option<&models::UserFieldName> {
		self.sorting_column.as_ref()
	}

	pub fn reset_sorting_column(&mut self) {
		self.sorting_column = None;
	}

	pub fn set_queries(&mut self, queries: Vec<models::SearchQuery>) {
		self.queries = Some(queries);
	}

	pub fn with_queries(mut self, queries: Vec<models::SearchQuery>) -> ListUsersRequest {
		self.queries = Some(queries);
		self
	}

	pub fn queries(&self) -> Option<&Vec<models::SearchQuery>> {
		self.queries.as_ref()
	}

	pub fn reset_queries(&mut self) {
		self.queries = None;
	}
}