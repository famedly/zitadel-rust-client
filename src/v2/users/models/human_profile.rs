/*
 * User Service
 *
 * This API is intended to manage users in a ZITADEL instance.
 *
 * OpenAPI spec version: 2.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{Deserialize, Serialize};

use crate::v2::users::models;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct HumanProfile {
	#[serde(rename = "givenName")]
	given_name: Option<String>,
	#[serde(rename = "familyName")]
	family_name: Option<String>,
	#[serde(rename = "nickName")]
	nick_name: Option<String>,
	#[serde(rename = "displayName")]
	display_name: Option<String>,
	#[serde(rename = "preferredLanguage")]
	preferred_language: Option<String>,
	#[serde(rename = "gender")]
	gender: Option<models::Gender>,
	/// avatar URL of the user
	#[serde(rename = "avatarUrl")]
	avatar_url: Option<String>,
}

impl HumanProfile {
	pub fn new() -> HumanProfile {
		HumanProfile {
			given_name: None,
			family_name: None,
			nick_name: None,
			display_name: None,
			preferred_language: None,
			gender: None,
			avatar_url: None,
		}
	}

	pub fn set_given_name(&mut self, given_name: String) {
		self.given_name = Some(given_name);
	}

	pub fn with_given_name(mut self, given_name: String) -> HumanProfile {
		self.given_name = Some(given_name);
		self
	}

	pub fn given_name(&self) -> Option<&String> {
		self.given_name.as_ref()
	}

	pub fn reset_given_name(&mut self) {
		self.given_name = None;
	}

	pub fn set_family_name(&mut self, family_name: String) {
		self.family_name = Some(family_name);
	}

	pub fn with_family_name(mut self, family_name: String) -> HumanProfile {
		self.family_name = Some(family_name);
		self
	}

	pub fn family_name(&self) -> Option<&String> {
		self.family_name.as_ref()
	}

	pub fn reset_family_name(&mut self) {
		self.family_name = None;
	}

	pub fn set_nick_name(&mut self, nick_name: String) {
		self.nick_name = Some(nick_name);
	}

	pub fn with_nick_name(mut self, nick_name: String) -> HumanProfile {
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

	pub fn with_display_name(mut self, display_name: String) -> HumanProfile {
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

	pub fn with_preferred_language(mut self, preferred_language: String) -> HumanProfile {
		self.preferred_language = Some(preferred_language);
		self
	}

	pub fn preferred_language(&self) -> Option<&String> {
		self.preferred_language.as_ref()
	}

	pub fn reset_preferred_language(&mut self) {
		self.preferred_language = None;
	}

	pub fn set_gender(&mut self, gender: models::Gender) {
		self.gender = Some(gender);
	}

	pub fn with_gender(mut self, gender: models::Gender) -> HumanProfile {
		self.gender = Some(gender);
		self
	}

	pub fn gender(&self) -> Option<&models::Gender> {
		self.gender.as_ref()
	}

	pub fn reset_gender(&mut self) {
		self.gender = None;
	}

	pub fn set_avatar_url(&mut self, avatar_url: String) {
		self.avatar_url = Some(avatar_url);
	}

	pub fn with_avatar_url(mut self, avatar_url: String) -> HumanProfile {
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
