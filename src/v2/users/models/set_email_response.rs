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
pub struct SetEmailResponse {
	#[serde(rename = "details")]
	details: Option<models::Details>,
	#[serde(rename = "verificationCode")]
	verification_code: Option<String>,
}

impl SetEmailResponse {
	pub fn new() -> SetEmailResponse {
		SetEmailResponse { details: None, verification_code: None }
	}

	pub fn set_details(&mut self, details: models::Details) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: models::Details) -> SetEmailResponse {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::Details> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_verification_code(&mut self, verification_code: String) {
		self.verification_code = Some(verification_code);
	}

	pub fn with_verification_code(mut self, verification_code: String) -> SetEmailResponse {
		self.verification_code = Some(verification_code);
		self
	}

	pub fn verification_code(&self) -> Option<&String> {
		self.verification_code.as_ref()
	}

	pub fn reset_verification_code(&mut self) {
		self.verification_code = None;
	}
}
