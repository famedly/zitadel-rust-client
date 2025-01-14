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
pub struct V1ListProjectGrantsResponse {
  #[serde(rename = "details")]
  details: Option<models::V1ListDetails>,
  #[serde(rename = "result")]
  result: Option<Vec<models::V1GrantedProject>>
}

impl V1ListProjectGrantsResponse {
  pub fn new() -> V1ListProjectGrantsResponse {
    V1ListProjectGrantsResponse {
      details: None,
      result: None
    }
  }

  pub fn set_details(&mut self, details: models::V1ListDetails) {
    self.details = Some(details);
  }

  pub fn with_details(mut self, details: models::V1ListDetails) -> V1ListProjectGrantsResponse {
    self.details = Some(details);
    self
  }

  pub fn details(&self) -> Option<&models::V1ListDetails> {
    self.details.as_ref()
  }

  pub fn reset_details(&mut self) {
    self.details = None;
  }

  pub fn set_result(&mut self, result: Vec<models::V1GrantedProject>) {
    self.result = Some(result);
  }

  pub fn with_result(mut self, result: Vec<models::V1GrantedProject>) -> V1ListProjectGrantsResponse {
    self.result = Some(result);
    self
  }

  pub fn result(&self) -> Option<&Vec<models::V1GrantedProject>> {
    self.result.as_ref()
  }

  pub fn reset_result(&mut self) {
    self.result = None;
  }

}



