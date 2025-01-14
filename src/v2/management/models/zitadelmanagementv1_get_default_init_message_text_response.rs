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
pub struct Zitadelmanagementv1GetDefaultInitMessageTextResponse {
  #[serde(rename = "customText")]
  custom_text: Option<models::V1MessageCustomText>
}

impl Zitadelmanagementv1GetDefaultInitMessageTextResponse {
  pub fn new() -> Zitadelmanagementv1GetDefaultInitMessageTextResponse {
    Zitadelmanagementv1GetDefaultInitMessageTextResponse {
      custom_text: None
    }
  }

  pub fn set_custom_text(&mut self, custom_text: models::V1MessageCustomText) {
    self.custom_text = Some(custom_text);
  }

  pub fn with_custom_text(mut self, custom_text: models::V1MessageCustomText) -> Zitadelmanagementv1GetDefaultInitMessageTextResponse {
    self.custom_text = Some(custom_text);
    self
  }

  pub fn custom_text(&self) -> Option<&models::V1MessageCustomText> {
    self.custom_text.as_ref()
  }

  pub fn reset_custom_text(&mut self) {
    self.custom_text = None;
  }

}



