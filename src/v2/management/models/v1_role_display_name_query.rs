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
pub struct V1RoleDisplayNameQuery {
  #[serde(rename = "displayName")]
  display_name: Option<String>,
  /// defines which text equality method is used
  #[serde(rename = "method")]
  method: Option<models::V1TextQueryMethod>
}

impl V1RoleDisplayNameQuery {
  pub fn new() -> V1RoleDisplayNameQuery {
    V1RoleDisplayNameQuery {
      display_name: None,
      method: None
    }
  }

  pub fn set_display_name(&mut self, display_name: String) {
    self.display_name = Some(display_name);
  }

  pub fn with_display_name(mut self, display_name: String) -> V1RoleDisplayNameQuery {
    self.display_name = Some(display_name);
    self
  }

  pub fn display_name(&self) -> Option<&String> {
    self.display_name.as_ref()
  }

  pub fn reset_display_name(&mut self) {
    self.display_name = None;
  }

  pub fn set_method(&mut self, method: models::V1TextQueryMethod) {
    self.method = Some(method);
  }

  pub fn with_method(mut self, method: models::V1TextQueryMethod) -> V1RoleDisplayNameQuery {
    self.method = Some(method);
    self
  }

  pub fn method(&self) -> Option<&models::V1TextQueryMethod> {
    self.method.as_ref()
  }

  pub fn reset_method(&mut self) {
    self.method = None;
  }

}



