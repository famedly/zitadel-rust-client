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
pub struct Userv1PersonalAccessToken {
	#[serde(rename = "id")]
	id: Option<String>,
	#[serde(rename = "details")]
	details: Option<models::V1ObjectDetails>,
	/// the date the token will expire
	#[serde(rename = "expirationDate")]
	expiration_date: Option<String>,
	/// scopes granted to the token
	#[serde(rename = "scopes")]
	scopes: Option<Vec<String>>,
}

impl Userv1PersonalAccessToken {
	pub fn new() -> Userv1PersonalAccessToken {
		Userv1PersonalAccessToken { id: None, details: None, expiration_date: None, scopes: None }
	}

	pub fn set_id(&mut self, id: String) {
		self.id = Some(id);
	}

	pub fn with_id(mut self, id: String) -> Userv1PersonalAccessToken {
		self.id = Some(id);
		self
	}

	pub fn id(&self) -> Option<&String> {
		self.id.as_ref()
	}

	pub fn reset_id(&mut self) {
		self.id = None;
	}

	pub fn set_details(&mut self, details: models::V1ObjectDetails) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: models::V1ObjectDetails) -> Userv1PersonalAccessToken {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::V1ObjectDetails> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_expiration_date(&mut self, expiration_date: String) {
		self.expiration_date = Some(expiration_date);
	}

	pub fn with_expiration_date(mut self, expiration_date: String) -> Userv1PersonalAccessToken {
		self.expiration_date = Some(expiration_date);
		self
	}

	pub fn expiration_date(&self) -> Option<&String> {
		self.expiration_date.as_ref()
	}

	pub fn reset_expiration_date(&mut self) {
		self.expiration_date = None;
	}

	pub fn set_scopes(&mut self, scopes: Vec<String>) {
		self.scopes = Some(scopes);
	}

	pub fn with_scopes(mut self, scopes: Vec<String>) -> Userv1PersonalAccessToken {
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
