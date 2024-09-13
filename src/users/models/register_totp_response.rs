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
pub struct RegisterTotpResponse {
	#[serde(rename = "details")]
	details: Option<models::Details>,
	#[serde(rename = "uri")]
	uri: Option<String>,
	#[serde(rename = "secret")]
	secret: Option<String>,
}

impl RegisterTotpResponse {
	pub fn new() -> RegisterTotpResponse {
		RegisterTotpResponse { details: None, uri: None, secret: None }
	}

	pub fn set_details(&mut self, details: models::Details) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: models::Details) -> RegisterTotpResponse {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::Details> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_uri(&mut self, uri: String) {
		self.uri = Some(uri);
	}

	pub fn with_uri(mut self, uri: String) -> RegisterTotpResponse {
		self.uri = Some(uri);
		self
	}

	pub fn uri(&self) -> Option<&String> {
		self.uri.as_ref()
	}

	pub fn reset_uri(&mut self) {
		self.uri = None;
	}

	pub fn set_secret(&mut self, secret: String) {
		self.secret = Some(secret);
	}

	pub fn with_secret(mut self, secret: String) -> RegisterTotpResponse {
		self.secret = Some(secret);
		self
	}

	pub fn secret(&self) -> Option<&String> {
		self.secret.as_ref()
	}

	pub fn reset_secret(&mut self) {
		self.secret = None;
	}
}
