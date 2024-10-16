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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StateQuery {
	/// current state of the user
	#[serde(rename = "state")]
	state: models::UserState,
}

impl StateQuery {
	/// Query for users with a specific state.
	pub fn new(state: models::UserState) -> StateQuery {
		StateQuery { state }
	}

	pub fn set_state(&mut self, state: models::UserState) {
		self.state = state;
	}

	pub fn with_state(mut self, state: models::UserState) -> StateQuery {
		self.state = state;
		self
	}

	pub fn state(&self) -> &models::UserState {
		&self.state
	}
}
