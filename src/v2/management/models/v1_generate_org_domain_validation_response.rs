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
pub struct V1GenerateOrgDomainValidationResponse {
	#[serde(rename = "token")]
	token: Option<String>,
	#[serde(rename = "url")]
	url: Option<String>,
}

impl V1GenerateOrgDomainValidationResponse {
	pub fn new() -> V1GenerateOrgDomainValidationResponse {
		V1GenerateOrgDomainValidationResponse { token: None, url: None }
	}

	pub fn set_token(&mut self, token: String) {
		self.token = Some(token);
	}

	pub fn with_token(mut self, token: String) -> V1GenerateOrgDomainValidationResponse {
		self.token = Some(token);
		self
	}

	pub fn token(&self) -> Option<&String> {
		self.token.as_ref()
	}

	pub fn reset_token(&mut self) {
		self.token = None;
	}

	pub fn set_url(&mut self, url: String) {
		self.url = Some(url);
	}

	pub fn with_url(mut self, url: String) -> V1GenerateOrgDomainValidationResponse {
		self.url = Some(url);
		self
	}

	pub fn url(&self) -> Option<&String> {
		self.url.as_ref()
	}

	pub fn reset_url(&mut self) {
		self.url = None;
	}
}
