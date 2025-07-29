// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::v2::users::models;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ListUserMetadataRequest {
	#[serde(rename = "query")]
	query: Option<models::ListQuery>,
	#[serde(rename = "queries")]
	queries: Option<Vec<models::KeyQuery>>,
}

impl ListUserMetadataRequest {
	pub fn new() -> ListUserMetadataRequest {
		ListUserMetadataRequest { query: None, queries: None }
	}

	pub fn with_query(mut self, query: models::ListQuery) -> ListUserMetadataRequest {
		self.query = Some(query);
		self
	}

	pub fn with_queries(mut self, queries: Vec<models::KeyQuery>) -> ListUserMetadataRequest {
		self.queries = Some(queries);
		self
	}
}
