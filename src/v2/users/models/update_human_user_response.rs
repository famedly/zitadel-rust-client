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
pub struct UpdateHumanUserResponse {
	#[serde(rename = "details")]
	details: Option<models::Details>,
	#[serde(rename = "emailCode")]
	email_code: Option<String>,
	#[serde(rename = "phoneCode")]
	phone_code: Option<String>,
}

impl UpdateHumanUserResponse {
	pub fn new() -> UpdateHumanUserResponse {
		UpdateHumanUserResponse { details: None, email_code: None, phone_code: None }
	}

	pub fn set_details(&mut self, details: models::Details) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: models::Details) -> UpdateHumanUserResponse {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::Details> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_email_code(&mut self, email_code: String) {
		self.email_code = Some(email_code);
	}

	pub fn with_email_code(mut self, email_code: String) -> UpdateHumanUserResponse {
		self.email_code = Some(email_code);
		self
	}

	pub fn email_code(&self) -> Option<&String> {
		self.email_code.as_ref()
	}

	pub fn reset_email_code(&mut self) {
		self.email_code = None;
	}

	pub fn set_phone_code(&mut self, phone_code: String) {
		self.phone_code = Some(phone_code);
	}

	pub fn with_phone_code(mut self, phone_code: String) -> UpdateHumanUserResponse {
		self.phone_code = Some(phone_code);
		self
	}

	pub fn phone_code(&self) -> Option<&String> {
		self.phone_code.as_ref()
	}

	pub fn reset_phone_code(&mut self) {
		self.phone_code = None;
	}
}