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

use crate::users::models;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VerifyTotpRegistrationResponse {
	#[serde(rename = "details")]
	details: Option<models::Details>,
}

impl VerifyTotpRegistrationResponse {
	pub fn new() -> VerifyTotpRegistrationResponse {
		VerifyTotpRegistrationResponse { details: None }
	}

	pub fn set_details(&mut self, details: models::Details) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: models::Details) -> VerifyTotpRegistrationResponse {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::Details> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}
}
