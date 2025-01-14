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
pub struct V1InitPasswordScreenText {
  #[serde(rename = "title")]
  title: Option<String>,
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "codeLabel")]
  code_label: Option<String>,
  #[serde(rename = "newPasswordLabel")]
  new_password_label: Option<String>,
  #[serde(rename = "newPasswordConfirmLabel")]
  new_password_confirm_label: Option<String>,
  #[serde(rename = "nextButtonText")]
  next_button_text: Option<String>,
  #[serde(rename = "resendButtonText")]
  resend_button_text: Option<String>
}

impl V1InitPasswordScreenText {
  pub fn new() -> V1InitPasswordScreenText {
    V1InitPasswordScreenText {
      title: None,
      description: None,
      code_label: None,
      new_password_label: None,
      new_password_confirm_label: None,
      next_button_text: None,
      resend_button_text: None
    }
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> V1InitPasswordScreenText {
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

  pub fn with_description(mut self, description: String) -> V1InitPasswordScreenText {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_code_label(&mut self, code_label: String) {
    self.code_label = Some(code_label);
  }

  pub fn with_code_label(mut self, code_label: String) -> V1InitPasswordScreenText {
    self.code_label = Some(code_label);
    self
  }

  pub fn code_label(&self) -> Option<&String> {
    self.code_label.as_ref()
  }

  pub fn reset_code_label(&mut self) {
    self.code_label = None;
  }

  pub fn set_new_password_label(&mut self, new_password_label: String) {
    self.new_password_label = Some(new_password_label);
  }

  pub fn with_new_password_label(mut self, new_password_label: String) -> V1InitPasswordScreenText {
    self.new_password_label = Some(new_password_label);
    self
  }

  pub fn new_password_label(&self) -> Option<&String> {
    self.new_password_label.as_ref()
  }

  pub fn reset_new_password_label(&mut self) {
    self.new_password_label = None;
  }

  pub fn set_new_password_confirm_label(&mut self, new_password_confirm_label: String) {
    self.new_password_confirm_label = Some(new_password_confirm_label);
  }

  pub fn with_new_password_confirm_label(mut self, new_password_confirm_label: String) -> V1InitPasswordScreenText {
    self.new_password_confirm_label = Some(new_password_confirm_label);
    self
  }

  pub fn new_password_confirm_label(&self) -> Option<&String> {
    self.new_password_confirm_label.as_ref()
  }

  pub fn reset_new_password_confirm_label(&mut self) {
    self.new_password_confirm_label = None;
  }

  pub fn set_next_button_text(&mut self, next_button_text: String) {
    self.next_button_text = Some(next_button_text);
  }

  pub fn with_next_button_text(mut self, next_button_text: String) -> V1InitPasswordScreenText {
    self.next_button_text = Some(next_button_text);
    self
  }

  pub fn next_button_text(&self) -> Option<&String> {
    self.next_button_text.as_ref()
  }

  pub fn reset_next_button_text(&mut self) {
    self.next_button_text = None;
  }

  pub fn set_resend_button_text(&mut self, resend_button_text: String) {
    self.resend_button_text = Some(resend_button_text);
  }

  pub fn with_resend_button_text(mut self, resend_button_text: String) -> V1InitPasswordScreenText {
    self.resend_button_text = Some(resend_button_text);
    self
  }

  pub fn resend_button_text(&self) -> Option<&String> {
    self.resend_button_text.as_ref()
  }

  pub fn reset_resend_button_text(&mut self) {
    self.resend_button_text = None;
  }

}



