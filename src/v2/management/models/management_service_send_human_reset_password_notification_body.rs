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
pub struct ManagementServiceSendHumanResetPasswordNotificationBody {
  #[serde(rename = "type")]
  _type: Option<models::V1SendHumanResetPasswordNotificationRequestType>
}

impl ManagementServiceSendHumanResetPasswordNotificationBody {
  pub fn new() -> ManagementServiceSendHumanResetPasswordNotificationBody {
    ManagementServiceSendHumanResetPasswordNotificationBody {
      _type: None
    }
  }

  pub fn set__type(&mut self, _type: models::V1SendHumanResetPasswordNotificationRequestType) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: models::V1SendHumanResetPasswordNotificationRequestType) -> ManagementServiceSendHumanResetPasswordNotificationBody {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&models::V1SendHumanResetPasswordNotificationRequestType> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

}



