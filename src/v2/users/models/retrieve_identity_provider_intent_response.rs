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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RetrieveIdentityProviderIntentResponse {
	#[serde(rename = "details")]
	details: Option<models::Details>,
	#[serde(rename = "idpInformation")]
	idp_information: Option<models::IdpInformation>,
	/// ID of the user in ZITADEL if external user is linked
	#[serde(rename = "userId")]
	user_id: Option<String>,
}

impl RetrieveIdentityProviderIntentResponse {
	pub fn new() -> RetrieveIdentityProviderIntentResponse {
		RetrieveIdentityProviderIntentResponse {
			details: None,
			idp_information: None,
			user_id: None,
		}
	}

	pub fn set_details(&mut self, details: models::Details) {
		self.details = Some(details);
	}

	pub fn with_details(
		mut self,
		details: models::Details,
	) -> RetrieveIdentityProviderIntentResponse {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::Details> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_idp_information(&mut self, idp_information: models::IdpInformation) {
		self.idp_information = Some(idp_information);
	}

	pub fn with_idp_information(
		mut self,
		idp_information: models::IdpInformation,
	) -> RetrieveIdentityProviderIntentResponse {
		self.idp_information = Some(idp_information);
		self
	}

	pub fn idp_information(&self) -> Option<&models::IdpInformation> {
		self.idp_information.as_ref()
	}

	pub fn reset_idp_information(&mut self) {
		self.idp_information = None;
	}

	pub fn set_user_id(&mut self, user_id: String) {
		self.user_id = Some(user_id);
	}

	pub fn with_user_id(mut self, user_id: String) -> RetrieveIdentityProviderIntentResponse {
		self.user_id = Some(user_id);
		self
	}

	pub fn user_id(&self) -> Option<&String> {
		self.user_id.as_ref()
	}

	pub fn reset_user_id(&mut self) {
		self.user_id = None;
	}
}