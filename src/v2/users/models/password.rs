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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Password {
	#[serde(rename = "password")]
	password: String,
	#[serde(rename = "changeRequired")]
	change_required: Option<bool>,
}

impl Password {
	pub fn new(password: String) -> Password {
		Password { password, change_required: None }
	}

	pub fn set_password(&mut self, password: String) {
		self.password = password;
	}

	pub fn with_password(mut self, password: String) -> Password {
		self.password = password;
		self
	}

	pub fn password(&self) -> &String {
		&self.password
	}

	pub fn set_change_required(&mut self, change_required: bool) {
		self.change_required = Some(change_required);
	}

	pub fn with_change_required(mut self, change_required: bool) -> Password {
		self.change_required = Some(change_required);
		self
	}

	pub fn change_required(&self) -> Option<&bool> {
		self.change_required.as_ref()
	}

	pub fn reset_change_required(&mut self) {
		self.change_required = None;
	}
}
