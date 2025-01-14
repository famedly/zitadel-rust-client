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
pub struct V1Action {
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "details")]
  details: Option<models::V1ObjectDetails>,
  /// the state of the action
  #[serde(rename = "state")]
  state: Option<models::V1ActionState>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "script")]
  script: Option<String>,
  /// after which time the action will be terminated if not finished
  #[serde(rename = "timeout")]
  timeout: Option<String>,
  /// when true, the next action will be called even if this action fails
  #[serde(rename = "allowedToFail")]
  allowed_to_fail: Option<bool>
}

impl V1Action {
  pub fn new() -> V1Action {
    V1Action {
      id: None,
      details: None,
      state: None,
      name: None,
      script: None,
      timeout: None,
      allowed_to_fail: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> V1Action {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_details(&mut self, details: models::V1ObjectDetails) {
    self.details = Some(details);
  }

  pub fn with_details(mut self, details: models::V1ObjectDetails) -> V1Action {
    self.details = Some(details);
    self
  }

  pub fn details(&self) -> Option<&models::V1ObjectDetails> {
    self.details.as_ref()
  }

  pub fn reset_details(&mut self) {
    self.details = None;
  }

  pub fn set_state(&mut self, state: models::V1ActionState) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: models::V1ActionState) -> V1Action {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&models::V1ActionState> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> V1Action {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_script(&mut self, script: String) {
    self.script = Some(script);
  }

  pub fn with_script(mut self, script: String) -> V1Action {
    self.script = Some(script);
    self
  }

  pub fn script(&self) -> Option<&String> {
    self.script.as_ref()
  }

  pub fn reset_script(&mut self) {
    self.script = None;
  }

  pub fn set_timeout(&mut self, timeout: String) {
    self.timeout = Some(timeout);
  }

  pub fn with_timeout(mut self, timeout: String) -> V1Action {
    self.timeout = Some(timeout);
    self
  }

  pub fn timeout(&self) -> Option<&String> {
    self.timeout.as_ref()
  }

  pub fn reset_timeout(&mut self) {
    self.timeout = None;
  }

  pub fn set_allowed_to_fail(&mut self, allowed_to_fail: bool) {
    self.allowed_to_fail = Some(allowed_to_fail);
  }

  pub fn with_allowed_to_fail(mut self, allowed_to_fail: bool) -> V1Action {
    self.allowed_to_fail = Some(allowed_to_fail);
    self
  }

  pub fn allowed_to_fail(&self) -> Option<&bool> {
    self.allowed_to_fail.as_ref()
  }

  pub fn reset_allowed_to_fail(&mut self) {
    self.allowed_to_fail = None;
  }

}



