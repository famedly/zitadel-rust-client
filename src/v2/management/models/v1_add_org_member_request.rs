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
pub struct V1AddOrgMemberRequest {
	#[serde(rename = "userId")]
	user_id: Option<String>,
	/// If no roles are provided the user won't have any rights
	#[serde(rename = "roles")]
	roles: Option<Vec<String>>,
}

impl V1AddOrgMemberRequest {
	pub fn new() -> V1AddOrgMemberRequest {
		V1AddOrgMemberRequest { user_id: None, roles: None }
	}

	pub fn set_user_id(&mut self, user_id: String) {
		self.user_id = Some(user_id);
	}

	pub fn with_user_id(mut self, user_id: String) -> V1AddOrgMemberRequest {
		self.user_id = Some(user_id);
		self
	}

	pub fn user_id(&self) -> Option<&String> {
		self.user_id.as_ref()
	}

	pub fn reset_user_id(&mut self) {
		self.user_id = None;
	}

	pub fn set_roles(&mut self, roles: Vec<String>) {
		self.roles = Some(roles);
	}

	pub fn with_roles(mut self, roles: Vec<String>) -> V1AddOrgMemberRequest {
		self.roles = Some(roles);
		self
	}

	pub fn roles(&self) -> Option<&Vec<String>> {
		self.roles.as_ref()
	}

	pub fn reset_roles(&mut self) {
		self.roles = None;
	}
}
