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
pub struct V1AddHumanUserRequestPhone {
	#[serde(rename = "phone")]
	phone: Option<String>,
	#[serde(rename = "isPhoneVerified")]
	is_phone_verified: Option<bool>,
}

impl V1AddHumanUserRequestPhone {
	pub fn new() -> V1AddHumanUserRequestPhone {
		V1AddHumanUserRequestPhone { phone: None, is_phone_verified: None }
	}

	pub fn set_phone(&mut self, phone: String) {
		self.phone = Some(phone);
	}

	pub fn with_phone(mut self, phone: String) -> V1AddHumanUserRequestPhone {
		self.phone = Some(phone);
		self
	}

	pub fn phone(&self) -> Option<&String> {
		self.phone.as_ref()
	}

	pub fn reset_phone(&mut self) {
		self.phone = None;
	}

	pub fn set_is_phone_verified(&mut self, is_phone_verified: bool) {
		self.is_phone_verified = Some(is_phone_verified);
	}

	pub fn with_is_phone_verified(mut self, is_phone_verified: bool) -> V1AddHumanUserRequestPhone {
		self.is_phone_verified = Some(is_phone_verified);
		self
	}

	pub fn is_phone_verified(&self) -> Option<&bool> {
		self.is_phone_verified.as_ref()
	}

	pub fn reset_is_phone_verified(&mut self) {
		self.is_phone_verified = None;
	}
}
