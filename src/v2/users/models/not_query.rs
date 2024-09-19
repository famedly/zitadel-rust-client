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
pub struct NotQuery {
	/// the sub query to negate (NOT)
	#[serde(rename = "query")]
	query: Option<Box<models::SearchQuery>>,
}

impl NotQuery {
	/// Negate the sub-condition.
	pub fn new() -> NotQuery {
		NotQuery { query: None }
	}

	pub fn set_query(&mut self, query: models::SearchQuery) {
		self.query = Some(Box::new(query));
	}

	pub fn with_query(mut self, query: models::SearchQuery) -> NotQuery {
		self.query = Some(Box::new(query));
		self
	}

	pub fn query(&self) -> Option<&models::SearchQuery> {
		self.query.as_deref()
	}

	pub fn reset_query(&mut self) {
		self.query = None;
	}
}
