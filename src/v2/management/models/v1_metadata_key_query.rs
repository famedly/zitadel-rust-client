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
pub struct V1MetadataKeyQuery {
  #[serde(rename = "key")]
  key: Option<String>,
  /// defines which text equality method is used
  #[serde(rename = "method")]
  method: Option<models::V1TextQueryMethod>
}

impl V1MetadataKeyQuery {
  pub fn new() -> V1MetadataKeyQuery {
    V1MetadataKeyQuery {
      key: None,
      method: None
    }
  }

  pub fn set_key(&mut self, key: String) {
    self.key = Some(key);
  }

  pub fn with_key(mut self, key: String) -> V1MetadataKeyQuery {
    self.key = Some(key);
    self
  }

  pub fn key(&self) -> Option<&String> {
    self.key.as_ref()
  }

  pub fn reset_key(&mut self) {
    self.key = None;
  }

  pub fn set_method(&mut self, method: models::V1TextQueryMethod) {
    self.method = Some(method);
  }

  pub fn with_method(mut self, method: models::V1TextQueryMethod) -> V1MetadataKeyQuery {
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



