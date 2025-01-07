use serde::{Deserialize, Serialize};

use crate::v2::{pagination::PaginationRequest, users::models};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub(crate) struct ListUserMetadataRequestOuter {
	#[serde(rename = "query")]
	query: Option<models::ListQuery>,
	#[serde(rename = "queries")]
	queries: Option<Vec<models::KeyQuery>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListUserMetadataRequest {
	asc: Option<bool>,
	queries: Vec<models::KeyQuery>,
	page_size: usize,
}

impl ListUserMetadataRequestOuter {
	pub fn new() -> ListUserMetadataRequestOuter {
		ListUserMetadataRequestOuter { query: None, queries: None }
	}

	pub fn with_query(mut self, query: models::ListQuery) -> ListUserMetadataRequestOuter {
		self.query = Some(query);
		self
	}

	pub fn with_queries(mut self, queries: Vec<models::KeyQuery>) -> ListUserMetadataRequestOuter {
		self.queries = Some(queries);
		self
	}
}

impl ListUserMetadataRequest {
	pub fn new(queries: Vec<models::KeyQuery>) -> ListUserMetadataRequest {
		ListUserMetadataRequest { asc: None, queries, page_size: 100 }
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

	pub fn set_queries(&mut self, queries: Vec<models::KeyQuery>) {
		self.queries = queries;
	}

	pub fn with_queries(mut self, queries: Vec<models::KeyQuery>) -> Self {
		self.queries = queries;
		self
	}

	pub fn queries(&self) -> &Vec<models::KeyQuery> {
		&self.queries
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

impl PaginationRequest for ListUserMetadataRequest {
	type Item = ListUserMetadataRequestOuter;
	fn to_paginated_request(&self, page: usize, page_size: usize) -> Self::Item {
		let page = models::ListQuery::new()
			.with_limit(page_size)
			.with_offset((page * page_size).to_string())
			.with_asc(self.asc.unwrap_or_default());
		ListUserMetadataRequestOuter::new().with_query(page).with_queries(self.queries.clone())
	}
}
