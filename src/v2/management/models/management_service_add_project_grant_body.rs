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
pub struct ManagementServiceAddProjectGrantBody {
	#[serde(rename = "grantedOrgId")]
	granted_org_id: Option<String>,
	#[serde(rename = "roleKeys")]
	role_keys: Option<Vec<String>>,
}

impl ManagementServiceAddProjectGrantBody {
	pub fn new() -> ManagementServiceAddProjectGrantBody {
		ManagementServiceAddProjectGrantBody { granted_org_id: None, role_keys: None }
	}

	pub fn set_granted_org_id(&mut self, granted_org_id: String) {
		self.granted_org_id = Some(granted_org_id);
	}

	pub fn with_granted_org_id(
		mut self,
		granted_org_id: String,
	) -> ManagementServiceAddProjectGrantBody {
		self.granted_org_id = Some(granted_org_id);
		self
	}

	pub fn granted_org_id(&self) -> Option<&String> {
		self.granted_org_id.as_ref()
	}

	pub fn reset_granted_org_id(&mut self) {
		self.granted_org_id = None;
	}

	pub fn set_role_keys(&mut self, role_keys: Vec<String>) {
		self.role_keys = Some(role_keys);
	}

	pub fn with_role_keys(
		mut self,
		role_keys: Vec<String>,
	) -> ManagementServiceAddProjectGrantBody {
		self.role_keys = Some(role_keys);
		self
	}

	pub fn role_keys(&self) -> Option<&Vec<String>> {
		self.role_keys.as_ref()
	}

	pub fn reset_role_keys(&mut self) {
		self.role_keys = None;
	}
}
