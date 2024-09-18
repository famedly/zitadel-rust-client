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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserServiceRegisterPasskeyBody {
	/// \"one time code generated by ZITADEL; required to start the passkey
	/// registration without user authentication\"
	#[serde(rename = "code")]
	code: Option<models::PasskeyRegistrationCode>,
	/// \"Optionally specify the authenticator type of the passkey device
	/// (platform or cross-platform). If none is provided, both values are
	/// allowed.\"
	#[serde(rename = "authenticator")]
	authenticator: Option<models::PasskeyAuthenticator>,
	/// \"Domain on which the user is authenticated.\"
	#[serde(rename = "domain")]
	domain: Option<String>,
}

impl UserServiceRegisterPasskeyBody {
	pub fn new() -> UserServiceRegisterPasskeyBody {
		UserServiceRegisterPasskeyBody { code: None, authenticator: None, domain: None }
	}

	pub fn set_code(&mut self, code: models::PasskeyRegistrationCode) {
		self.code = Some(code);
	}

	pub fn with_code(
		mut self,
		code: models::PasskeyRegistrationCode,
	) -> UserServiceRegisterPasskeyBody {
		self.code = Some(code);
		self
	}

	pub fn code(&self) -> Option<&models::PasskeyRegistrationCode> {
		self.code.as_ref()
	}

	pub fn reset_code(&mut self) {
		self.code = None;
	}

	pub fn set_authenticator(&mut self, authenticator: models::PasskeyAuthenticator) {
		self.authenticator = Some(authenticator);
	}

	pub fn with_authenticator(
		mut self,
		authenticator: models::PasskeyAuthenticator,
	) -> UserServiceRegisterPasskeyBody {
		self.authenticator = Some(authenticator);
		self
	}

	pub fn authenticator(&self) -> Option<&models::PasskeyAuthenticator> {
		self.authenticator.as_ref()
	}

	pub fn reset_authenticator(&mut self) {
		self.authenticator = None;
	}

	pub fn set_domain(&mut self, domain: String) {
		self.domain = Some(domain);
	}

	pub fn with_domain(mut self, domain: String) -> UserServiceRegisterPasskeyBody {
		self.domain = Some(domain);
		self
	}

	pub fn domain(&self) -> Option<&String> {
		self.domain.as_ref()
	}

	pub fn reset_domain(&mut self) {
		self.domain = None;
	}
}
