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
pub struct V1AddHumanUserRequestEmail {
	#[serde(rename = "email")]
	email: Option<String>,
	#[serde(rename = "isEmailVerified")]
	is_email_verified: Option<bool>,
}

impl V1AddHumanUserRequestEmail {
	pub fn new() -> V1AddHumanUserRequestEmail {
		V1AddHumanUserRequestEmail { email: None, is_email_verified: None }
	}

	pub fn set_email(&mut self, email: String) {
		self.email = Some(email);
	}

	pub fn with_email(mut self, email: String) -> V1AddHumanUserRequestEmail {
		self.email = Some(email);
		self
	}

	pub fn email(&self) -> Option<&String> {
		self.email.as_ref()
	}

	pub fn reset_email(&mut self) {
		self.email = None;
	}

	pub fn set_is_email_verified(&mut self, is_email_verified: bool) {
		self.is_email_verified = Some(is_email_verified);
	}

	pub fn with_is_email_verified(mut self, is_email_verified: bool) -> V1AddHumanUserRequestEmail {
		self.is_email_verified = Some(is_email_verified);
		self
	}

	pub fn is_email_verified(&self) -> Option<&bool> {
		self.is_email_verified.as_ref()
	}

	pub fn reset_is_email_verified(&mut self) {
		self.is_email_verified = None;
	}
}
