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
pub struct V1GitLabSelfHostedConfig {
	#[serde(rename = "issuer")]
	issuer: Option<String>,
	/// client id of the GitLab application
	#[serde(rename = "clientId")]
	client_id: Option<String>,
	/// the scopes requested by ZITADEL during the request to GitLab
	#[serde(rename = "scopes")]
	scopes: Option<Vec<String>>,
}

impl V1GitLabSelfHostedConfig {
	pub fn new() -> V1GitLabSelfHostedConfig {
		V1GitLabSelfHostedConfig { issuer: None, client_id: None, scopes: None }
	}

	pub fn set_issuer(&mut self, issuer: String) {
		self.issuer = Some(issuer);
	}

	pub fn with_issuer(mut self, issuer: String) -> V1GitLabSelfHostedConfig {
		self.issuer = Some(issuer);
		self
	}

	pub fn issuer(&self) -> Option<&String> {
		self.issuer.as_ref()
	}

	pub fn reset_issuer(&mut self) {
		self.issuer = None;
	}

	pub fn set_client_id(&mut self, client_id: String) {
		self.client_id = Some(client_id);
	}

	pub fn with_client_id(mut self, client_id: String) -> V1GitLabSelfHostedConfig {
		self.client_id = Some(client_id);
		self
	}

	pub fn client_id(&self) -> Option<&String> {
		self.client_id.as_ref()
	}

	pub fn reset_client_id(&mut self) {
		self.client_id = None;
	}

	pub fn set_scopes(&mut self, scopes: Vec<String>) {
		self.scopes = Some(scopes);
	}

	pub fn with_scopes(mut self, scopes: Vec<String>) -> V1GitLabSelfHostedConfig {
		self.scopes = Some(scopes);
		self
	}

	pub fn scopes(&self) -> Option<&Vec<String>> {
		self.scopes.as_ref()
	}

	pub fn reset_scopes(&mut self) {
		self.scopes = None;
	}
}
