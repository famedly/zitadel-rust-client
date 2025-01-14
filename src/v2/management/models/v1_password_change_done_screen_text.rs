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
pub struct V1PasswordChangeDoneScreenText {
  #[serde(rename = "title")]
  title: Option<String>,
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "nextButtonText")]
  next_button_text: Option<String>
}

impl V1PasswordChangeDoneScreenText {
  pub fn new() -> V1PasswordChangeDoneScreenText {
    V1PasswordChangeDoneScreenText {
      title: None,
      description: None,
      next_button_text: None
    }
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> V1PasswordChangeDoneScreenText {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> V1PasswordChangeDoneScreenText {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_next_button_text(&mut self, next_button_text: String) {
    self.next_button_text = Some(next_button_text);
  }

  pub fn with_next_button_text(mut self, next_button_text: String) -> V1PasswordChangeDoneScreenText {
    self.next_button_text = Some(next_button_text);
    self
  }

  pub fn next_button_text(&self) -> Option<&String> {
    self.next_button_text.as_ref()
  }

  pub fn reset_next_button_text(&mut self) {
    self.next_button_text = None;
  }

}



