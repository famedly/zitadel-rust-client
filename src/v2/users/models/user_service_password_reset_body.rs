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
pub struct UserServicePasswordResetBody {
	#[serde(rename = "sendLink")]
	send_link: Option<models::SendPasswordResetLink>,
	#[serde(rename = "returnCode")]
	return_code: Option<models::ReturnPasswordResetCode>,
}

impl UserServicePasswordResetBody {
	pub fn new() -> UserServicePasswordResetBody {
		UserServicePasswordResetBody { send_link: None, return_code: None }
	}

	pub fn set_send_link(&mut self, send_link: models::SendPasswordResetLink) {
		self.send_link = Some(send_link);
	}

	pub fn with_send_link(
		mut self,
		send_link: models::SendPasswordResetLink,
	) -> UserServicePasswordResetBody {
		self.send_link = Some(send_link);
		self
	}

	pub fn send_link(&self) -> Option<&models::SendPasswordResetLink> {
		self.send_link.as_ref()
	}

	pub fn reset_send_link(&mut self) {
		self.send_link = None;
	}

	pub fn set_return_code(&mut self, return_code: models::ReturnPasswordResetCode) {
		self.return_code = Some(return_code);
	}

	pub fn with_return_code(
		mut self,
		return_code: models::ReturnPasswordResetCode,
	) -> UserServicePasswordResetBody {
		self.return_code = Some(return_code);
		self
	}

	pub fn return_code(&self) -> Option<&models::ReturnPasswordResetCode> {
		self.return_code.as_ref()
	}

	pub fn reset_return_code(&mut self) {
		self.return_code = None;
	}
}