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
pub struct UserServiceResendEmailCodeBody {
	#[serde(rename = "sendCode")]
	send_code: Option<models::SendEmailVerificationCode>,
	#[serde(rename = "returnCode")]
	return_code: Option<models::ReturnEmailVerificationCode>,
}

impl UserServiceResendEmailCodeBody {
	pub fn new() -> UserServiceResendEmailCodeBody {
		UserServiceResendEmailCodeBody { send_code: None, return_code: None }
	}

	pub fn set_send_code(&mut self, send_code: models::SendEmailVerificationCode) {
		self.send_code = Some(send_code);
	}

	pub fn with_send_code(
		mut self,
		send_code: models::SendEmailVerificationCode,
	) -> UserServiceResendEmailCodeBody {
		self.send_code = Some(send_code);
		self
	}

	pub fn send_code(&self) -> Option<&models::SendEmailVerificationCode> {
		self.send_code.as_ref()
	}

	pub fn reset_send_code(&mut self) {
		self.send_code = None;
	}

	pub fn set_return_code(&mut self, return_code: models::ReturnEmailVerificationCode) {
		self.return_code = Some(return_code);
	}

	pub fn with_return_code(
		mut self,
		return_code: models::ReturnEmailVerificationCode,
	) -> UserServiceResendEmailCodeBody {
		self.return_code = Some(return_code);
		self
	}

	pub fn return_code(&self) -> Option<&models::ReturnEmailVerificationCode> {
		self.return_code.as_ref()
	}

	pub fn reset_return_code(&mut self) {
		self.return_code = None;
	}
}
