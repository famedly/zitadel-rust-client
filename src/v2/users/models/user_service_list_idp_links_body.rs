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

use crate::v2::users::{models, pagination::PaginationRequest};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserServiceListIdpLinksBodyOuter {
	#[serde(rename = "query")]
	query: Option<models::ListQuery>,
}

impl UserServiceListIdpLinksBodyOuter {
	pub fn new() -> UserServiceListIdpLinksBodyOuter {
		UserServiceListIdpLinksBodyOuter { query: None }
	}

	pub fn with_query(mut self, query: models::ListQuery) -> UserServiceListIdpLinksBodyOuter {
		self.query = Some(query);
		self
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserServiceListIdpLinksBody {
	asc: Option<bool>,
	page_size: usize,
}

impl UserServiceListIdpLinksBody {
	pub fn new() -> Self {
		Self { asc: None, page_size: 100 }
	}

	pub fn set_asc(&mut self, asc: bool) {
		self.asc = Some(asc);
	}

	pub fn with_asc(mut self, asc: bool) -> Self {
		self.asc = Some(asc);
		self
	}

	pub fn asc(&self) -> Option<&bool> {
		self.asc.as_ref()
	}

	pub fn reset_asc(&mut self) {
		self.asc = None;
	}

	pub fn set_page_size(&mut self, page_size: usize) {
		self.page_size = page_size;
	}

	pub fn with_page_size(mut self, page_size: usize) -> Self {
		self.page_size = page_size;
		self
	}

	pub fn page_size(&self) -> usize {
		self.page_size
	}
}

impl PaginationRequest for UserServiceListIdpLinksBody {
	type Item = UserServiceListIdpLinksBodyOuter;
	fn to_paginated_request(&self, page: usize, page_size: usize) -> Self::Item {
		UserServiceListIdpLinksBodyOuter::new().with_query(
			models::ListQuery::new()
				.with_limit(page_size)
				.with_offset((page * page_size).to_string())
				.with_asc(self.asc.unwrap_or_default()),
		)
	}
}
