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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserServiceSetPasswordBody {
	#[serde(rename = "newPassword")]
	new_password: Option<models::Password>,
	#[serde(rename = "currentPassword")]
	current_password: String,
	/// \"the verification code generated during password reset request\"
	#[serde(rename = "verificationCode")]
	verification_code: String,
}

impl UserServiceSetPasswordBody {
	pub fn new(current_password: String, verification_code: String) -> UserServiceSetPasswordBody {
		UserServiceSetPasswordBody { new_password: None, current_password, verification_code }
	}

	pub fn set_new_password(&mut self, new_password: models::Password) {
		self.new_password = Some(new_password);
	}

	pub fn with_new_password(
		mut self,
		new_password: models::Password,
	) -> UserServiceSetPasswordBody {
		self.new_password = Some(new_password);
		self
	}

	pub fn new_password(&self) -> Option<&models::Password> {
		self.new_password.as_ref()
	}

	pub fn reset_new_password(&mut self) {
		self.new_password = None;
	}

	pub fn set_current_password(&mut self, current_password: String) {
		self.current_password = current_password;
	}

	pub fn with_current_password(mut self, current_password: String) -> UserServiceSetPasswordBody {
		self.current_password = current_password;
		self
	}

	pub fn current_password(&self) -> &String {
		&self.current_password
	}

	pub fn set_verification_code(&mut self, verification_code: String) {
		self.verification_code = verification_code;
	}

	pub fn with_verification_code(
		mut self,
		verification_code: String,
	) -> UserServiceSetPasswordBody {
		self.verification_code = verification_code;
		self
	}

	pub fn verification_code(&self) -> &String {
		&self.verification_code
	}
}
