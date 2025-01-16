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
pub struct Zitadelmanagementv1ResetCustomLoginTextsToDefaultResponse {
	#[serde(rename = "details")]
	details: Option<models::V1ObjectDetails>,
}

impl Zitadelmanagementv1ResetCustomLoginTextsToDefaultResponse {
	pub fn new() -> Zitadelmanagementv1ResetCustomLoginTextsToDefaultResponse {
		Zitadelmanagementv1ResetCustomLoginTextsToDefaultResponse { details: None }
	}

	pub fn set_details(&mut self, details: models::V1ObjectDetails) {
		self.details = Some(details);
	}

	pub fn with_details(
		mut self,
		details: models::V1ObjectDetails,
	) -> Zitadelmanagementv1ResetCustomLoginTextsToDefaultResponse {
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
