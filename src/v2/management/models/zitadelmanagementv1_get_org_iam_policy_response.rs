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
pub struct Zitadelmanagementv1GetOrgIamPolicyResponse {
	#[serde(rename = "policy")]
	policy: Option<models::V1OrgIamPolicy>,
}

impl Zitadelmanagementv1GetOrgIamPolicyResponse {
	pub fn new() -> Zitadelmanagementv1GetOrgIamPolicyResponse {
		Zitadelmanagementv1GetOrgIamPolicyResponse { policy: None }
	}

	pub fn set_policy(&mut self, policy: models::V1OrgIamPolicy) {
		self.policy = Some(policy);
	}

	pub fn with_policy(
		mut self,
		policy: models::V1OrgIamPolicy,
	) -> Zitadelmanagementv1GetOrgIamPolicyResponse {
		self.policy = Some(policy);
		self
	}

	pub fn policy(&self) -> Option<&models::V1OrgIamPolicy> {
		self.policy.as_ref()
	}

	pub fn reset_policy(&mut self) {
		self.policy = None;
	}
}
