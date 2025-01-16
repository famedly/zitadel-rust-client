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
pub struct V1MembershipOrgQuery {
	#[serde(rename = "orgId")]
	org_id: Option<String>,
}

impl V1MembershipOrgQuery {
	pub fn new() -> V1MembershipOrgQuery {
		V1MembershipOrgQuery { org_id: None }
	}

	pub fn set_org_id(&mut self, org_id: String) {
		self.org_id = Some(org_id);
	}

	pub fn with_org_id(mut self, org_id: String) -> V1MembershipOrgQuery {
		self.org_id = Some(org_id);
		self
	}

	pub fn org_id(&self) -> Option<&String> {
		self.org_id.as_ref()
	}

	pub fn reset_org_id(&mut self) {
		self.org_id = None;
	}
}
