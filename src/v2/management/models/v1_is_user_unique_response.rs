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
pub struct V1IsUserUniqueResponse {
  #[serde(rename = "isUnique")]
  is_unique: Option<bool>
}

impl V1IsUserUniqueResponse {
  pub fn new() -> V1IsUserUniqueResponse {
    V1IsUserUniqueResponse {
      is_unique: None
    }
  }

  pub fn set_is_unique(&mut self, is_unique: bool) {
    self.is_unique = Some(is_unique);
  }

  pub fn with_is_unique(mut self, is_unique: bool) -> V1IsUserUniqueResponse {
    self.is_unique = Some(is_unique);
    self
  }

  pub fn is_unique(&self) -> Option<&bool> {
    self.is_unique.as_ref()
  }

  pub fn reset_is_unique(&mut self) {
    self.is_unique = None;
  }

}



