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
pub struct Passkey {
	#[serde(rename = "id")]
	id: Option<String>,
	/// current state of the passkey
	#[serde(rename = "state")]
	state: Option<models::AuthFactorState>,
	#[serde(rename = "name")]
	name: Option<String>,
}

impl Passkey {
	pub fn new() -> Passkey {
		Passkey { id: None, state: None, name: None }
	}

	pub fn set_id(&mut self, id: String) {
		self.id = Some(id);
	}

	pub fn with_id(mut self, id: String) -> Passkey {
		self.id = Some(id);
		self
	}

	pub fn id(&self) -> Option<&String> {
		self.id.as_ref()
	}

	pub fn reset_id(&mut self) {
		self.id = None;
	}

	pub fn set_state(&mut self, state: models::AuthFactorState) {
		self.state = Some(state);
	}

	pub fn with_state(mut self, state: models::AuthFactorState) -> Passkey {
		self.state = Some(state);
		self
	}

	pub fn state(&self) -> Option<&models::AuthFactorState> {
		self.state.as_ref()
	}

	pub fn reset_state(&mut self) {
		self.state = None;
	}

	pub fn set_name(&mut self, name: String) {
		self.name = Some(name);
	}

	pub fn with_name(mut self, name: String) -> Passkey {
		self.name = Some(name);
		self
	}

	pub fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	pub fn reset_name(&mut self) {
		self.name = None;
	}
}
