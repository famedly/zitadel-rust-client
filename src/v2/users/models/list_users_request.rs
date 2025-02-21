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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ListUsersRequest {
	#[serde(rename = "query")]
	query: Option<models::ListQuery>,
	#[serde(rename = "sortingColumn")]
	sorting_column: Option<models::UserFieldName>,
	#[serde(rename = "queries")]
	queries: Option<Vec<models::SearchQuery>>,
}

impl ListUsersRequest {
	pub fn new() -> Self {
		ListUsersRequest { query: None, sorting_column: None, queries: None }
	}

	pub fn with_query(mut self, query: models::ListQuery) -> ListUsersRequest {
		self.query = Some(query);
		self
	}

	pub fn set_sorting_column(&mut self, sorting_column: models::UserFieldName) {
		self.sorting_column = Some(sorting_column);
	}

	pub fn with_queries(mut self, queries: Vec<models::SearchQuery>) -> ListUsersRequest {
		self.queries = Some(queries);
		self
	}
}
