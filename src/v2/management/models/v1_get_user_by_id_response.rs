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
pub struct V1GetUserByIdResponse {
  #[serde(rename = "user")]
  user: Option<models::V1User>
}

impl V1GetUserByIdResponse {
  pub fn new() -> V1GetUserByIdResponse {
    V1GetUserByIdResponse {
      user: None
    }
  }

  pub fn set_user(&mut self, user: models::V1User) {
    self.user = Some(user);
  }

  pub fn with_user(mut self, user: models::V1User) -> V1GetUserByIdResponse {
    self.user = Some(user);
    self
  }

  pub fn user(&self) -> Option<&models::V1User> {
    self.user.as_ref()
  }

  pub fn reset_user(&mut self) {
    self.user = None;
  }

}



