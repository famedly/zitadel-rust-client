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
pub struct V1GetHumanPhoneResponse {
  #[serde(rename = "details")]
  details: Option<models::V1ObjectDetails>,
  #[serde(rename = "phone")]
  phone: Option<models::Userv1Phone>
}

impl V1GetHumanPhoneResponse {
  pub fn new() -> V1GetHumanPhoneResponse {
    V1GetHumanPhoneResponse {
      details: None,
      phone: None
    }
  }

  pub fn set_details(&mut self, details: models::V1ObjectDetails) {
    self.details = Some(details);
  }

  pub fn with_details(mut self, details: models::V1ObjectDetails) -> V1GetHumanPhoneResponse {
    self.details = Some(details);
    self
  }

  pub fn details(&self) -> Option<&models::V1ObjectDetails> {
    self.details.as_ref()
  }

  pub fn reset_details(&mut self) {
    self.details = None;
  }

  pub fn set_phone(&mut self, phone: models::Userv1Phone) {
    self.phone = Some(phone);
  }

  pub fn with_phone(mut self, phone: models::Userv1Phone) -> V1GetHumanPhoneResponse {
    self.phone = Some(phone);
    self
  }

  pub fn phone(&self) -> Option<&models::Userv1Phone> {
    self.phone.as_ref()
  }

  pub fn reset_phone(&mut self) {
    self.phone = None;
  }

}



