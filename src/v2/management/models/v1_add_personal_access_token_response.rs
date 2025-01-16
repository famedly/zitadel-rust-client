/*
 * Management API
 *
 * The management API is as the name states the interface where systems can
 * mutate IAM objects like organizations, projects, clients, users and so on
 * if they have the necessary access rights.
 *
 * OpenAPI spec version: 1.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

use crate::v2::management::models;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct V1AddPersonalAccessTokenResponse {
	#[serde(rename = "tokenId")]
	token_id: Option<String>,
	#[serde(rename = "token")]
	token: Option<String>,
	#[serde(rename = "details")]
	details: Option<models::V1ObjectDetails>,
}

impl V1AddPersonalAccessTokenResponse {
	pub fn new() -> V1AddPersonalAccessTokenResponse {
		V1AddPersonalAccessTokenResponse { token_id: None, token: None, details: None }
	}

	pub fn set_token_id(&mut self, token_id: String) {
		self.token_id = Some(token_id);
	}

	pub fn with_token_id(mut self, token_id: String) -> V1AddPersonalAccessTokenResponse {
		self.token_id = Some(token_id);
		self
	}

	pub fn token_id(&self) -> Option<&String> {
		self.token_id.as_ref()
	}

	pub fn reset_token_id(&mut self) {
		self.token_id = None;
	}

	pub fn set_token(&mut self, token: String) {
		self.token = Some(token);
	}

	pub fn with_token(mut self, token: String) -> V1AddPersonalAccessTokenResponse {
		self.token = Some(token);
		self
	}

	pub fn token(&self) -> Option<&String> {
		self.token.as_ref()
	}

	pub fn reset_token(&mut self) {
		self.token = None;
	}

	pub fn set_details(&mut self, details: models::V1ObjectDetails) {
		self.details = Some(details);
	}

	pub fn with_details(
		mut self,
		details: models::V1ObjectDetails,
	) -> V1AddPersonalAccessTokenResponse {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::V1ObjectDetails> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}
}
