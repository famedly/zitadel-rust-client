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
pub struct ListAuthenticationMethodTypesResponse {
	#[serde(rename = "details")]
	details: Option<models::ListDetails>,
	#[serde(rename = "authMethodTypes")]
	auth_method_types: Option<Vec<models::AuthenticationMethodType>>,
}

impl ListAuthenticationMethodTypesResponse {
	pub fn new() -> ListAuthenticationMethodTypesResponse {
		ListAuthenticationMethodTypesResponse { details: None, auth_method_types: None }
	}

	pub fn set_details(&mut self, details: models::ListDetails) {
		self.details = Some(details);
	}

	pub fn with_details(
		mut self,
		details: models::ListDetails,
	) -> ListAuthenticationMethodTypesResponse {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::ListDetails> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_auth_method_types(
		&mut self,
		auth_method_types: Vec<models::AuthenticationMethodType>,
	) {
		self.auth_method_types = Some(auth_method_types);
	}

	pub fn with_auth_method_types(
		mut self,
		auth_method_types: Vec<models::AuthenticationMethodType>,
	) -> ListAuthenticationMethodTypesResponse {
		self.auth_method_types = Some(auth_method_types);
		self
	}

	pub fn auth_method_types(&self) -> Option<&Vec<models::AuthenticationMethodType>> {
		self.auth_method_types.as_ref()
	}

	pub fn reset_auth_method_types(&mut self) {
		self.auth_method_types = None;
	}
}
