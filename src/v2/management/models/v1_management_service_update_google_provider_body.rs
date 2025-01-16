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
pub struct V1ManagementServiceUpdateGoogleProviderBody {
	#[serde(rename = "name")]
	name: Option<String>,
	/// Client id generated by Google
	#[serde(rename = "clientId")]
	client_id: Option<String>,
	/// Client secret will only be updated if provided
	#[serde(rename = "clientSecret")]
	client_secret: Option<String>,
	/// The scopes requested by ZITADEL during the request to Google
	#[serde(rename = "scopes")]
	scopes: Option<Vec<String>>,
	#[serde(rename = "providerOptions")]
	provider_options: Option<models::V1Options>,
}

impl V1ManagementServiceUpdateGoogleProviderBody {
	pub fn new() -> V1ManagementServiceUpdateGoogleProviderBody {
		V1ManagementServiceUpdateGoogleProviderBody {
			name: None,
			client_id: None,
			client_secret: None,
			scopes: None,
			provider_options: None,
		}
	}

	pub fn set_name(&mut self, name: String) {
		self.name = Some(name);
	}

	pub fn with_name(mut self, name: String) -> V1ManagementServiceUpdateGoogleProviderBody {
		self.name = Some(name);
		self
	}

	pub fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	pub fn reset_name(&mut self) {
		self.name = None;
	}

	pub fn set_client_id(&mut self, client_id: String) {
		self.client_id = Some(client_id);
	}

	pub fn with_client_id(
		mut self,
		client_id: String,
	) -> V1ManagementServiceUpdateGoogleProviderBody {
		self.client_id = Some(client_id);
		self
	}

	pub fn client_id(&self) -> Option<&String> {
		self.client_id.as_ref()
	}

	pub fn reset_client_id(&mut self) {
		self.client_id = None;
	}

	pub fn set_client_secret(&mut self, client_secret: String) {
		self.client_secret = Some(client_secret);
	}

	pub fn with_client_secret(
		mut self,
		client_secret: String,
	) -> V1ManagementServiceUpdateGoogleProviderBody {
		self.client_secret = Some(client_secret);
		self
	}

	pub fn client_secret(&self) -> Option<&String> {
		self.client_secret.as_ref()
	}

	pub fn reset_client_secret(&mut self) {
		self.client_secret = None;
	}

	pub fn set_scopes(&mut self, scopes: Vec<String>) {
		self.scopes = Some(scopes);
	}

	pub fn with_scopes(
		mut self,
		scopes: Vec<String>,
	) -> V1ManagementServiceUpdateGoogleProviderBody {
		self.scopes = Some(scopes);
		self
	}

	pub fn scopes(&self) -> Option<&Vec<String>> {
		self.scopes.as_ref()
	}

	pub fn reset_scopes(&mut self) {
		self.scopes = None;
	}

	pub fn set_provider_options(&mut self, provider_options: models::V1Options) {
		self.provider_options = Some(provider_options);
	}

	pub fn with_provider_options(
		mut self,
		provider_options: models::V1Options,
	) -> V1ManagementServiceUpdateGoogleProviderBody {
		self.provider_options = Some(provider_options);
		self
	}

	pub fn provider_options(&self) -> Option<&models::V1Options> {
		self.provider_options.as_ref()
	}

	pub fn reset_provider_options(&mut self) {
		self.provider_options = None;
	}
}
