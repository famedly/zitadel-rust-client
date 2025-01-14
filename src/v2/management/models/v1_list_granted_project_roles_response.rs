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
pub struct V1ListGrantedProjectRolesResponse {
  #[serde(rename = "details")]
  details: Option<models::V1ListDetails>,
  #[serde(rename = "result")]
  result: Option<Vec<models::Projectv1Role>>
}

impl V1ListGrantedProjectRolesResponse {
  pub fn new() -> V1ListGrantedProjectRolesResponse {
    V1ListGrantedProjectRolesResponse {
      details: None,
      result: None
    }
  }

  pub fn set_details(&mut self, details: models::V1ListDetails) {
    self.details = Some(details);
  }

  pub fn with_details(mut self, details: models::V1ListDetails) -> V1ListGrantedProjectRolesResponse {
    self.details = Some(details);
    self
  }

  pub fn details(&self) -> Option<&models::V1ListDetails> {
    self.details.as_ref()
  }

  pub fn reset_details(&mut self) {
    self.details = None;
  }

  pub fn set_result(&mut self, result: Vec<models::Projectv1Role>) {
    self.result = Some(result);
  }

  pub fn with_result(mut self, result: Vec<models::Projectv1Role>) -> V1ListGrantedProjectRolesResponse {
    self.result = Some(result);
    self
  }

  pub fn result(&self) -> Option<&Vec<models::Projectv1Role>> {
    self.result.as_ref()
  }

  pub fn reset_result(&mut self) {
    self.result = None;
  }

}



