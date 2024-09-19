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
pub struct UpdateHumanUserRequest {
	#[serde(rename = "username")]
	username: Option<String>,
	#[serde(rename = "profile")]
	profile: Option<models::SetHumanProfile>,
	#[serde(rename = "email")]
	email: Option<models::SetHumanEmail>,
	#[serde(rename = "phone")]
	phone: Option<models::SetHumanPhone>,
	#[serde(rename = "password")]
	password: Option<models::UpdateHumanUserPassword>,
}

impl UpdateHumanUserRequest {
	pub fn new() -> UpdateHumanUserRequest {
		UpdateHumanUserRequest {
			username: None,
			profile: None,
			email: None,
			phone: None,
			password: None,
		}
	}

	pub fn set_username(&mut self, username: String) {
		self.username = Some(username);
	}

	pub fn with_username(mut self, username: String) -> UpdateHumanUserRequest {
		self.username = Some(username);
		self
	}

	pub fn username(&self) -> Option<&String> {
		self.username.as_ref()
	}

	pub fn reset_username(&mut self) {
		self.username = None;
	}

	pub fn set_profile(&mut self, profile: models::SetHumanProfile) {
		self.profile = Some(profile);
	}

	pub fn with_profile(mut self, profile: models::SetHumanProfile) -> UpdateHumanUserRequest {
		self.profile = Some(profile);
		self
	}

	pub fn profile(&self) -> Option<&models::SetHumanProfile> {
		self.profile.as_ref()
	}

	pub fn set_email(&mut self, email: models::SetHumanEmail) {
		self.email = Some(email);
	}

	pub fn with_email(mut self, email: models::SetHumanEmail) -> UpdateHumanUserRequest {
		self.email = Some(email);
		self
	}

	pub fn email(&self) -> Option<&models::SetHumanEmail> {
		self.email.as_ref()
	}

	pub fn set_phone(&mut self, phone: models::SetHumanPhone) {
		self.phone = Some(phone);
	}

	pub fn with_phone(mut self, phone: models::SetHumanPhone) -> UpdateHumanUserRequest {
		self.phone = Some(phone);
		self
	}

	pub fn phone(&self) -> Option<&models::SetHumanPhone> {
		self.phone.as_ref()
	}

	pub fn reset_phone(&mut self) {
		self.phone = None;
	}

	pub fn set_password(&mut self, password: models::UpdateHumanUserPassword) {
		self.password = Some(password);
	}

	pub fn with_password(
		mut self,
		password: models::UpdateHumanUserPassword,
	) -> UpdateHumanUserRequest {
		self.password = Some(password);
		self
	}

	pub fn password(&self) -> Option<&models::UpdateHumanUserPassword> {
		self.password.as_ref()
	}
}