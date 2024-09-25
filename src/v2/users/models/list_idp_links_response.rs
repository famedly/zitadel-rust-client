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
pub struct ListIdpLinksResponse {
	#[serde(rename = "details")]
	details: Option<models::ListDetails>,
	#[serde(rename = "result")]
	result: Option<Vec<models::IdpLink>>,
}

impl ListIdpLinksResponse {
	pub fn new() -> ListIdpLinksResponse {
		ListIdpLinksResponse { details: None, result: None }
	}

	pub fn set_details(&mut self, details: models::ListDetails) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: models::ListDetails) -> ListIdpLinksResponse {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::ListDetails> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_result(&mut self, result: Vec<models::IdpLink>) {
		self.result = Some(result);
	}

	pub fn with_result(mut self, result: Vec<models::IdpLink>) -> ListIdpLinksResponse {
		self.result = Some(result);
		self
	}

	pub fn result(&self) -> Option<&Vec<models::IdpLink>> {
		self.result.as_ref()
	}

	pub fn reset_result(&mut self) {
		self.result = None;
	}
}

impl PaginationResponse for ListIdpLinksResponse {
	type Item = models::IdpLink;
	fn take_result(&mut self) -> Option<Vec<Self::Item>> {
		self.result.take()
	}
}
