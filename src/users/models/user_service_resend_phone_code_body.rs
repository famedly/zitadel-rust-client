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
#[allow(unused_imports)]
use serde_json::Value;

use crate::users::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserServiceResendPhoneCodeBody {
	#[serde(rename = "sendCode")]
	send_code: Option<models::SendPhoneVerificationCode>,
	#[serde(rename = "returnCode")]
	return_code: Option<models::ReturnPhoneVerificationCode>,
}

impl UserServiceResendPhoneCodeBody {
	pub fn new() -> UserServiceResendPhoneCodeBody {
		UserServiceResendPhoneCodeBody { send_code: None, return_code: None }
	}

	pub fn set_send_code(&mut self, send_code: models::SendPhoneVerificationCode) {
		self.send_code = Some(send_code);
	}

	pub fn with_send_code(
		mut self,
		send_code: models::SendPhoneVerificationCode,
	) -> UserServiceResendPhoneCodeBody {
		self.send_code = Some(send_code);
		self
	}

	pub fn send_code(&self) -> Option<&models::SendPhoneVerificationCode> {
		self.send_code.as_ref()
	}

	pub fn reset_send_code(&mut self) {
		self.send_code = None;
	}

	pub fn set_return_code(&mut self, return_code: models::ReturnPhoneVerificationCode) {
		self.return_code = Some(return_code);
	}

	pub fn with_return_code(
		mut self,
		return_code: models::ReturnPhoneVerificationCode,
	) -> UserServiceResendPhoneCodeBody {
		self.return_code = Some(return_code);
		self
	}

	pub fn return_code(&self) -> Option<&models::ReturnPhoneVerificationCode> {
		self.return_code.as_ref()
	}

	pub fn reset_return_code(&mut self) {
		self.return_code = None;
	}
}
