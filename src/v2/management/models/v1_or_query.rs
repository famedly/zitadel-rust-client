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
pub struct V1OrQuery {
	/// the sub queries to 'OR'
	#[serde(rename = "queries")]
	queries: Option<Vec<models::Zitadeluserv1SearchQuery>>,
}

impl V1OrQuery {
	pub fn new() -> V1OrQuery {
		V1OrQuery { queries: None }
	}

	pub fn set_queries(&mut self, queries: Vec<models::Zitadeluserv1SearchQuery>) {
		self.queries = Some(queries);
	}

	pub fn with_queries(mut self, queries: Vec<models::Zitadeluserv1SearchQuery>) -> V1OrQuery {
		self.queries = Some(queries);
		self
	}

	pub fn queries(&self) -> Option<&Vec<models::Zitadeluserv1SearchQuery>> {
		self.queries.as_ref()
	}

	pub fn reset_queries(&mut self) {
		self.queries = None;
	}
}
