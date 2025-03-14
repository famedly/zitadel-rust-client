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
pub struct Zitadelmanagementv1GetPasswordComplexityPolicyResponse {
	#[serde(rename = "policy")]
	policy: Option<models::V1PasswordComplexityPolicy>,
	/// Deprecated: Is defined in the policy itself
	#[serde(rename = "isDefault")]
	is_default: Option<bool>,
}

impl Zitadelmanagementv1GetPasswordComplexityPolicyResponse {
	pub fn new() -> Zitadelmanagementv1GetPasswordComplexityPolicyResponse {
		Zitadelmanagementv1GetPasswordComplexityPolicyResponse { policy: None, is_default: None }
	}

	pub fn set_policy(&mut self, policy: models::V1PasswordComplexityPolicy) {
		self.policy = Some(policy);
	}

	pub fn with_policy(
		mut self,
		policy: models::V1PasswordComplexityPolicy,
	) -> Zitadelmanagementv1GetPasswordComplexityPolicyResponse {
		self.policy = Some(policy);
		self
	}

	pub fn policy(&self) -> Option<&models::V1PasswordComplexityPolicy> {
		self.policy.as_ref()
	}

	pub fn reset_policy(&mut self) {
		self.policy = None;
	}

	pub fn set_is_default(&mut self, is_default: bool) {
		self.is_default = Some(is_default);
	}

	pub fn with_is_default(
		mut self,
		is_default: bool,
	) -> Zitadelmanagementv1GetPasswordComplexityPolicyResponse {
		self.is_default = Some(is_default);
		self
	}

	pub fn is_default(&self) -> Option<&bool> {
		self.is_default.as_ref()
	}

	pub fn reset_is_default(&mut self) {
		self.is_default = None;
	}
}
