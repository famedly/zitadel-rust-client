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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HumanEmail {
	#[serde(rename = "email")]
	email: Option<String>,
	#[serde(rename = "isVerified")]
	is_verified: Option<bool>,
}

impl HumanEmail {
	pub fn new() -> HumanEmail {
		HumanEmail { email: None, is_verified: None }
	}

	pub fn set_email(&mut self, email: String) {
		self.email = Some(email);
	}

	pub fn with_email(mut self, email: String) -> HumanEmail {
		self.email = Some(email);
		self
	}

	pub fn email(&self) -> Option<&String> {
		self.email.as_ref()
	}

	pub fn reset_email(&mut self) {
		self.email = None;
	}

	pub fn set_is_verified(&mut self, is_verified: bool) {
		self.is_verified = Some(is_verified);
	}

	pub fn with_is_verified(mut self, is_verified: bool) -> HumanEmail {
		self.is_verified = Some(is_verified);
		self
	}

	pub fn is_verified(&self) -> Option<&bool> {
		self.is_verified.as_ref()
	}

	pub fn reset_is_verified(&mut self) {
		self.is_verified = None;
	}
}
