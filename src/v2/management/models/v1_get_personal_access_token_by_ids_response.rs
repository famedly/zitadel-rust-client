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
pub struct V1GetPersonalAccessTokenByIdsResponse {
	#[serde(rename = "token")]
	token: Option<models::Userv1PersonalAccessToken>,
}

impl V1GetPersonalAccessTokenByIdsResponse {
	pub fn new() -> V1GetPersonalAccessTokenByIdsResponse {
		V1GetPersonalAccessTokenByIdsResponse { token: None }
	}

	pub fn set_token(&mut self, token: models::Userv1PersonalAccessToken) {
		self.token = Some(token);
	}

	pub fn with_token(
		mut self,
		token: models::Userv1PersonalAccessToken,
	) -> V1GetPersonalAccessTokenByIdsResponse {
		self.token = Some(token);
		self
	}

	pub fn token(&self) -> Option<&models::Userv1PersonalAccessToken> {
		self.token.as_ref()
	}

	pub fn reset_token(&mut self) {
		self.token = None;
	}
}
