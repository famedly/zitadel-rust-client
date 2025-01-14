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
pub struct Userv1Profile {
  #[serde(rename = "firstName")]
  first_name: Option<String>,
  #[serde(rename = "lastName")]
  last_name: Option<String>,
  #[serde(rename = "nickName")]
  nick_name: Option<String>,
  /// a user can set the display name, if nothing is set ZITADEL computes \"first_name last_name\"
  #[serde(rename = "displayName")]
  display_name: Option<String>,
  /// language tag analog https://tools.ietf.org/html/rfc3066
  #[serde(rename = "preferredLanguage")]
  preferred_language: Option<String>,
  /// the gender of the human
  #[serde(rename = "gender")]
  gender: Option<models::V1Gender>,
  /// avatar URL of the user
  #[serde(rename = "avatarUrl")]
  avatar_url: Option<String>
}

impl Userv1Profile {
  pub fn new() -> Userv1Profile {
    Userv1Profile {
      first_name: None,
      last_name: None,
      nick_name: None,
      display_name: None,
      preferred_language: None,
      gender: None,
      avatar_url: None
    }
  }

  pub fn set_first_name(&mut self, first_name: String) {
    self.first_name = Some(first_name);
  }

  pub fn with_first_name(mut self, first_name: String) -> Userv1Profile {
    self.first_name = Some(first_name);
    self
  }

  pub fn first_name(&self) -> Option<&String> {
    self.first_name.as_ref()
  }

  pub fn reset_first_name(&mut self) {
    self.first_name = None;
  }

  pub fn set_last_name(&mut self, last_name: String) {
    self.last_name = Some(last_name);
  }

  pub fn with_last_name(mut self, last_name: String) -> Userv1Profile {
    self.last_name = Some(last_name);
    self
  }

  pub fn last_name(&self) -> Option<&String> {
    self.last_name.as_ref()
  }

  pub fn reset_last_name(&mut self) {
    self.last_name = None;
  }

  pub fn set_nick_name(&mut self, nick_name: String) {
    self.nick_name = Some(nick_name);
  }

  pub fn with_nick_name(mut self, nick_name: String) -> Userv1Profile {
    self.nick_name = Some(nick_name);
    self
  }

  pub fn nick_name(&self) -> Option<&String> {
    self.nick_name.as_ref()
  }

  pub fn reset_nick_name(&mut self) {
    self.nick_name = None;
  }

  pub fn set_display_name(&mut self, display_name: String) {
    self.display_name = Some(display_name);
  }

  pub fn with_display_name(mut self, display_name: String) -> Userv1Profile {
    self.display_name = Some(display_name);
    self
  }

  pub fn display_name(&self) -> Option<&String> {
    self.display_name.as_ref()
  }

  pub fn reset_display_name(&mut self) {
    self.display_name = None;
  }

  pub fn set_preferred_language(&mut self, preferred_language: String) {
    self.preferred_language = Some(preferred_language);
  }

  pub fn with_preferred_language(mut self, preferred_language: String) -> Userv1Profile {
    self.preferred_language = Some(preferred_language);
    self
  }

  pub fn preferred_language(&self) -> Option<&String> {
    self.preferred_language.as_ref()
  }

  pub fn reset_preferred_language(&mut self) {
    self.preferred_language = None;
  }

  pub fn set_gender(&mut self, gender: models::V1Gender) {
    self.gender = Some(gender);
  }

  pub fn with_gender(mut self, gender: models::V1Gender) -> Userv1Profile {
    self.gender = Some(gender);
    self
  }

  pub fn gender(&self) -> Option<&models::V1Gender> {
    self.gender.as_ref()
  }

  pub fn reset_gender(&mut self) {
    self.gender = None;
  }

  pub fn set_avatar_url(&mut self, avatar_url: String) {
    self.avatar_url = Some(avatar_url);
  }

  pub fn with_avatar_url(mut self, avatar_url: String) -> Userv1Profile {
    self.avatar_url = Some(avatar_url);
    self
  }

  pub fn avatar_url(&self) -> Option<&String> {
    self.avatar_url.as_ref()
  }

  pub fn reset_avatar_url(&mut self) {
    self.avatar_url = None;
  }

}



