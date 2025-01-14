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
pub struct V1UpdateCustomPasswordComplexityPolicyRequest {
  #[serde(rename = "minLength")]
  min_length: Option<String>,
  /// Defines if the password MUST contain an upper case letter
  #[serde(rename = "hasUppercase")]
  has_uppercase: Option<bool>,
  /// Defines if the password MUST contain a lowercase letter
  #[serde(rename = "hasLowercase")]
  has_lowercase: Option<bool>,
  /// Defines if the password MUST contain a number
  #[serde(rename = "hasNumber")]
  has_number: Option<bool>,
  /// defines if the password MUST contain a symbol. E.g. \"$\"
  #[serde(rename = "hasSymbol")]
  has_symbol: Option<bool>
}

impl V1UpdateCustomPasswordComplexityPolicyRequest {
  pub fn new() -> V1UpdateCustomPasswordComplexityPolicyRequest {
    V1UpdateCustomPasswordComplexityPolicyRequest {
      min_length: None,
      has_uppercase: None,
      has_lowercase: None,
      has_number: None,
      has_symbol: None
    }
  }

  pub fn set_min_length(&mut self, min_length: String) {
    self.min_length = Some(min_length);
  }

  pub fn with_min_length(mut self, min_length: String) -> V1UpdateCustomPasswordComplexityPolicyRequest {
    self.min_length = Some(min_length);
    self
  }

  pub fn min_length(&self) -> Option<&String> {
    self.min_length.as_ref()
  }

  pub fn reset_min_length(&mut self) {
    self.min_length = None;
  }

  pub fn set_has_uppercase(&mut self, has_uppercase: bool) {
    self.has_uppercase = Some(has_uppercase);
  }

  pub fn with_has_uppercase(mut self, has_uppercase: bool) -> V1UpdateCustomPasswordComplexityPolicyRequest {
    self.has_uppercase = Some(has_uppercase);
    self
  }

  pub fn has_uppercase(&self) -> Option<&bool> {
    self.has_uppercase.as_ref()
  }

  pub fn reset_has_uppercase(&mut self) {
    self.has_uppercase = None;
  }

  pub fn set_has_lowercase(&mut self, has_lowercase: bool) {
    self.has_lowercase = Some(has_lowercase);
  }

  pub fn with_has_lowercase(mut self, has_lowercase: bool) -> V1UpdateCustomPasswordComplexityPolicyRequest {
    self.has_lowercase = Some(has_lowercase);
    self
  }

  pub fn has_lowercase(&self) -> Option<&bool> {
    self.has_lowercase.as_ref()
  }

  pub fn reset_has_lowercase(&mut self) {
    self.has_lowercase = None;
  }

  pub fn set_has_number(&mut self, has_number: bool) {
    self.has_number = Some(has_number);
  }

  pub fn with_has_number(mut self, has_number: bool) -> V1UpdateCustomPasswordComplexityPolicyRequest {
    self.has_number = Some(has_number);
    self
  }

  pub fn has_number(&self) -> Option<&bool> {
    self.has_number.as_ref()
  }

  pub fn reset_has_number(&mut self) {
    self.has_number = None;
  }

  pub fn set_has_symbol(&mut self, has_symbol: bool) {
    self.has_symbol = Some(has_symbol);
  }

  pub fn with_has_symbol(mut self, has_symbol: bool) -> V1UpdateCustomPasswordComplexityPolicyRequest {
    self.has_symbol = Some(has_symbol);
    self
  }

  pub fn has_symbol(&self) -> Option<&bool> {
    self.has_symbol.as_ref()
  }

  pub fn reset_has_symbol(&mut self) {
    self.has_symbol = None;
  }

}



