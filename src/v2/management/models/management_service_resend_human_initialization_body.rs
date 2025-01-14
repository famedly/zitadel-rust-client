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
pub struct ManagementServiceResendHumanInitializationBody {
  /// Send a new email address if the one set on the user is wrong or has a typo.
  #[serde(rename = "email")]
  email: Option<String>
}

impl ManagementServiceResendHumanInitializationBody {
  pub fn new() -> ManagementServiceResendHumanInitializationBody {
    ManagementServiceResendHumanInitializationBody {
      email: None
    }
  }

  pub fn set_email(&mut self, email: String) {
    self.email = Some(email);
  }

  pub fn with_email(mut self, email: String) -> ManagementServiceResendHumanInitializationBody {
    self.email = Some(email);
    self
  }

  pub fn email(&self) -> Option<&String> {
    self.email.as_ref()
  }

  pub fn reset_email(&mut self) {
    self.email = None;
  }

}



