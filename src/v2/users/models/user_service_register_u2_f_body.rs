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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserServiceRegisterU2FBody {
	/// \"Domain on which the user is authenticated.\"
	#[serde(rename = "domain")]
	domain: Option<String>,
}

impl UserServiceRegisterU2FBody {
	pub fn new() -> UserServiceRegisterU2FBody {
		UserServiceRegisterU2FBody { domain: None }
	}

	pub fn set_domain(&mut self, domain: String) {
		self.domain = Some(domain);
	}

	pub fn with_domain(mut self, domain: String) -> UserServiceRegisterU2FBody {
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
