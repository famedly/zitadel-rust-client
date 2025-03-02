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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateHumanUserPassword {
	#[serde(rename = "password")]
	password: Option<models::Password>,
	#[serde(rename = "hashedPassword")]
	hashed_password: Option<models::HashedPassword>,
	#[serde(rename = "currentPassword")]
	current_password: String,
	/// \"the verification code generated during password reset request\"
	#[serde(rename = "verificationCode")]
	verification_code: String,
}

impl UpdateHumanUserPassword {
	pub fn new(current_password: String, verification_code: String) -> UpdateHumanUserPassword {
		UpdateHumanUserPassword {
			password: None,
			hashed_password: None,
			current_password,
			verification_code,
		}
	}

	pub fn set_password(&mut self, password: models::Password) {
		self.password = Some(password);
	}

	pub fn with_password(mut self, password: models::Password) -> UpdateHumanUserPassword {
		self.password = Some(password);
		self
	}

	pub fn password(&self) -> Option<&models::Password> {
		self.password.as_ref()
	}

	pub fn reset_password(&mut self) {
		self.password = None;
	}

	pub fn set_hashed_password(&mut self, hashed_password: models::HashedPassword) {
		self.hashed_password = Some(hashed_password);
	}

	pub fn with_hashed_password(
		mut self,
		hashed_password: models::HashedPassword,
	) -> UpdateHumanUserPassword {
		self.hashed_password = Some(hashed_password);
		self
	}

	pub fn hashed_password(&self) -> Option<&models::HashedPassword> {
		self.hashed_password.as_ref()
	}

	pub fn reset_hashed_password(&mut self) {
		self.hashed_password = None;
	}

	pub fn set_current_password(&mut self, current_password: String) {
		self.current_password = current_password;
	}

	pub fn with_current_password(mut self, current_password: String) -> UpdateHumanUserPassword {
		self.current_password = current_password;
		self
	}

	pub fn current_password(&self) -> &String {
		&self.current_password
	}

	pub fn set_verification_code(&mut self, verification_code: String) {
		self.verification_code = verification_code;
	}

	pub fn with_verification_code(mut self, verification_code: String) -> UpdateHumanUserPassword {
		self.verification_code = verification_code;
		self
	}

	pub fn verification_code(&self) -> &String {
		&self.verification_code
	}
}
