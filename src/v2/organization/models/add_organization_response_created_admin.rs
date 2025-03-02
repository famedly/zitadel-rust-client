/*
 * Organization Service
 *
 * This API is intended to manage organizations in a ZITADEL instance.
 *
 * OpenAPI spec version: 2.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

use crate::v2::organization::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddOrganizationResponseCreatedAdmin {
	#[serde(rename = "userId")]
	user_id: Option<String>,
	#[serde(rename = "emailCode")]
	email_code: Option<String>,
	#[serde(rename = "phoneCode")]
	phone_code: Option<String>,
}

impl AddOrganizationResponseCreatedAdmin {
	pub fn new() -> AddOrganizationResponseCreatedAdmin {
		AddOrganizationResponseCreatedAdmin { user_id: None, email_code: None, phone_code: None }
	}

	pub fn set_user_id(&mut self, user_id: String) {
		self.user_id = Some(user_id);
	}

	pub fn with_user_id(mut self, user_id: String) -> AddOrganizationResponseCreatedAdmin {
		self.user_id = Some(user_id);
		self
	}

	pub fn user_id(&self) -> Option<&String> {
		self.user_id.as_ref()
	}

	pub fn reset_user_id(&mut self) {
		self.user_id = None;
	}

	pub fn set_email_code(&mut self, email_code: String) {
		self.email_code = Some(email_code);
	}

	pub fn with_email_code(mut self, email_code: String) -> AddOrganizationResponseCreatedAdmin {
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

	pub fn with_phone_code(mut self, phone_code: String) -> AddOrganizationResponseCreatedAdmin {
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
