/*
 * Organization Service
 *
 * This API is intended to manage organizations in a ZITADEL instance.
 *
 * OpenAPI spec version: 2.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

use crate::v2::organization::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct V2SetHumanProfile {
	#[serde(rename = "givenName")]
	given_name: String,
	#[serde(rename = "familyName")]
	family_name: String,
	#[serde(rename = "nickName")]
	nick_name: Option<String>,
	#[serde(rename = "displayName")]
	display_name: Option<String>,
	#[serde(rename = "preferredLanguage")]
	preferred_language: Option<String>,
	#[serde(rename = "gender")]
	gender: Option<models::V2Gender>,
}

impl V2SetHumanProfile {
	pub fn new(given_name: String, family_name: String) -> V2SetHumanProfile {
		V2SetHumanProfile {
			given_name: given_name,
			family_name: family_name,
			nick_name: None,
			display_name: None,
			preferred_language: None,
			gender: None,
		}
	}

	pub fn set_given_name(&mut self, given_name: String) {
		self.given_name = given_name;
	}

	pub fn with_given_name(mut self, given_name: String) -> V2SetHumanProfile {
		self.given_name = given_name;
		self
	}

	pub fn given_name(&self) -> &String {
		&self.given_name
	}

	pub fn set_family_name(&mut self, family_name: String) {
		self.family_name = family_name;
	}

	pub fn with_family_name(mut self, family_name: String) -> V2SetHumanProfile {
		self.family_name = family_name;
		self
	}

	pub fn family_name(&self) -> &String {
		&self.family_name
	}

	pub fn set_nick_name(&mut self, nick_name: String) {
		self.nick_name = Some(nick_name);
	}

	pub fn with_nick_name(mut self, nick_name: String) -> V2SetHumanProfile {
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

	pub fn with_display_name(mut self, display_name: String) -> V2SetHumanProfile {
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

	pub fn with_preferred_language(mut self, preferred_language: String) -> V2SetHumanProfile {
		self.preferred_language = Some(preferred_language);
		self
	}

	pub fn preferred_language(&self) -> Option<&String> {
		self.preferred_language.as_ref()
	}

	pub fn reset_preferred_language(&mut self) {
		self.preferred_language = None;
	}

	pub fn set_gender(&mut self, gender: models::V2Gender) {
		self.gender = Some(gender);
	}

	pub fn with_gender(mut self, gender: models::V2Gender) -> V2SetHumanProfile {
		self.gender = Some(gender);
		self
	}

	pub fn gender(&self) -> Option<&models::V2Gender> {
		self.gender.as_ref()
	}

	pub fn reset_gender(&mut self) {
		self.gender = None;
	}
}
