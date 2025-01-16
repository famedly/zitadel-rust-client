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
pub struct V1AzureAdConfig {
	/// client id of the Azure AD application
	#[serde(rename = "clientId")]
	client_id: Option<String>,
	/// Defines what user accounts should be able to login (Personal,
	/// Organizational, All)
	#[serde(rename = "tenant")]
	tenant: Option<models::V1AzureAdTenant>,
	/// Azure AD doesn't send if the email has been verified. Enable this if the
	/// user email should always be added verified in ZITADEL (no verification
	/// emails will be sent)
	#[serde(rename = "emailVerified")]
	email_verified: Option<bool>,
	/// the scopes requested by ZITADEL during the request to Azure AD
	#[serde(rename = "scopes")]
	scopes: Option<Vec<String>>,
}

impl V1AzureAdConfig {
	pub fn new() -> V1AzureAdConfig {
		V1AzureAdConfig { client_id: None, tenant: None, email_verified: None, scopes: None }
	}

	pub fn set_client_id(&mut self, client_id: String) {
		self.client_id = Some(client_id);
	}

	pub fn with_client_id(mut self, client_id: String) -> V1AzureAdConfig {
		self.client_id = Some(client_id);
		self
	}

	pub fn client_id(&self) -> Option<&String> {
		self.client_id.as_ref()
	}

	pub fn reset_client_id(&mut self) {
		self.client_id = None;
	}

	pub fn set_tenant(&mut self, tenant: models::V1AzureAdTenant) {
		self.tenant = Some(tenant);
	}

	pub fn with_tenant(mut self, tenant: models::V1AzureAdTenant) -> V1AzureAdConfig {
		self.tenant = Some(tenant);
		self
	}

	pub fn tenant(&self) -> Option<&models::V1AzureAdTenant> {
		self.tenant.as_ref()
	}

	pub fn reset_tenant(&mut self) {
		self.tenant = None;
	}

	pub fn set_email_verified(&mut self, email_verified: bool) {
		self.email_verified = Some(email_verified);
	}

	pub fn with_email_verified(mut self, email_verified: bool) -> V1AzureAdConfig {
		self.email_verified = Some(email_verified);
		self
	}

	pub fn email_verified(&self) -> Option<&bool> {
		self.email_verified.as_ref()
	}

	pub fn reset_email_verified(&mut self) {
		self.email_verified = None;
	}

	pub fn set_scopes(&mut self, scopes: Vec<String>) {
		self.scopes = Some(scopes);
	}

	pub fn with_scopes(mut self, scopes: Vec<String>) -> V1AzureAdConfig {
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
