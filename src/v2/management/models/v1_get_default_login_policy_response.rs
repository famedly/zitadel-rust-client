/* 
 * Management API
 *
 * The management API is as the name states the interface where systems can mutate IAM objects like organizations, projects, clients, users and so on if they have the necessary access rights.
 *
 * OpenAPI spec version: 1.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;
use crate::v2::management::models;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct V1GetDefaultLoginPolicyResponse {
  #[serde(rename = "policy")]
  policy: Option<models::V1LoginPolicy>
}

impl V1GetDefaultLoginPolicyResponse {
  pub fn new() -> V1GetDefaultLoginPolicyResponse {
    V1GetDefaultLoginPolicyResponse {
      policy: None
    }
  }

  pub fn set_policy(&mut self, policy: models::V1LoginPolicy) {
    self.policy = Some(policy);
  }

  pub fn with_policy(mut self, policy: models::V1LoginPolicy) -> V1GetDefaultLoginPolicyResponse {
    self.policy = Some(policy);
    self
  }

  pub fn policy(&self) -> Option<&models::V1LoginPolicy> {
    self.policy.as_ref()
  }

  pub fn reset_policy(&mut self) {
    self.policy = None;
  }

}



