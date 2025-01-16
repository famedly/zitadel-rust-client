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
pub struct V1AddOrgOidcidpResponse {
	#[serde(rename = "details")]
	details: Option<models::V1ObjectDetails>,
	#[serde(rename = "idpId")]
	idp_id: Option<String>,
}

impl V1AddOrgOidcidpResponse {
	pub fn new() -> V1AddOrgOidcidpResponse {
		V1AddOrgOidcidpResponse { details: None, idp_id: None }
	}

	pub fn set_details(&mut self, details: models::V1ObjectDetails) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: models::V1ObjectDetails) -> V1AddOrgOidcidpResponse {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::V1ObjectDetails> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_idp_id(&mut self, idp_id: String) {
		self.idp_id = Some(idp_id);
	}

	pub fn with_idp_id(mut self, idp_id: String) -> V1AddOrgOidcidpResponse {
		self.idp_id = Some(idp_id);
		self
	}

	pub fn idp_id(&self) -> Option<&String> {
		self.idp_id.as_ref()
	}

	pub fn reset_idp_id(&mut self) {
		self.idp_id = None;
	}
}
