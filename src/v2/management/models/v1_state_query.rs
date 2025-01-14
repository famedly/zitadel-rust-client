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
pub struct V1StateQuery {
  /// current state of the user
  #[serde(rename = "state")]
  state: Option<models::V1UserState>
}

impl V1StateQuery {
  pub fn new() -> V1StateQuery {
    V1StateQuery {
      state: None
    }
  }

  pub fn set_state(&mut self, state: models::V1UserState) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: models::V1UserState) -> V1StateQuery {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&models::V1UserState> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

}



