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
pub struct UserServiceCreatePasskeyRegistrationLinkBody {
	#[serde(rename = "sendLink")]
	send_link: Option<models::SendPasskeyRegistrationLink>,
	#[serde(rename = "returnCode")]
	return_code: Option<models::ReturnPasskeyRegistrationCode>,
}

impl UserServiceCreatePasskeyRegistrationLinkBody {
	pub fn new() -> UserServiceCreatePasskeyRegistrationLinkBody {
		UserServiceCreatePasskeyRegistrationLinkBody { send_link: None, return_code: None }
	}

	pub fn set_send_link(&mut self, send_link: models::SendPasskeyRegistrationLink) {
		self.send_link = Some(send_link);
	}

	pub fn with_send_link(
		mut self,
		send_link: models::SendPasskeyRegistrationLink,
	) -> UserServiceCreatePasskeyRegistrationLinkBody {
		self.send_link = Some(send_link);
		self
	}

	pub fn send_link(&self) -> Option<&models::SendPasskeyRegistrationLink> {
		self.send_link.as_ref()
	}

	pub fn reset_send_link(&mut self) {
		self.send_link = None;
	}

	pub fn set_return_code(&mut self, return_code: models::ReturnPasskeyRegistrationCode) {
		self.return_code = Some(return_code);
	}

	pub fn with_return_code(
		mut self,
		return_code: models::ReturnPasskeyRegistrationCode,
	) -> UserServiceCreatePasskeyRegistrationLinkBody {
		self.return_code = Some(return_code);
		self
	}

	pub fn return_code(&self) -> Option<&models::ReturnPasskeyRegistrationCode> {
		self.return_code.as_ref()
	}

	pub fn reset_return_code(&mut self) {
		self.return_code = None;
	}
}
