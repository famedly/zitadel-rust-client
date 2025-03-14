/*
 * Management API
 *
 * The management API is as the name states the interface where systems can
 * mutate IAM objects like organizations, projects, clients, users and so on
 * if they have the necessary access rights.
 *
 * OpenAPI spec version: 1.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

use crate::v2::management::models;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct V1RegistrationOrgScreenText {
	#[serde(rename = "title")]
	title: Option<String>,
	#[serde(rename = "description")]
	description: Option<String>,
	#[serde(rename = "orgnameLabel")]
	orgname_label: Option<String>,
	#[serde(rename = "firstnameLabel")]
	firstname_label: Option<String>,
	#[serde(rename = "lastnameLabel")]
	lastname_label: Option<String>,
	#[serde(rename = "usernameLabel")]
	username_label: Option<String>,
	#[serde(rename = "emailLabel")]
	email_label: Option<String>,
	#[serde(rename = "passwordLabel")]
	password_label: Option<String>,
	#[serde(rename = "passwordConfirmLabel")]
	password_confirm_label: Option<String>,
	#[serde(rename = "tosAndPrivacyLabel")]
	tos_and_privacy_label: Option<String>,
	#[serde(rename = "tosConfirm")]
	tos_confirm: Option<String>,
	#[serde(rename = "tosLinkText")]
	tos_link_text: Option<String>,
	#[serde(rename = "privacyConfirm")]
	privacy_confirm: Option<String>,
	#[serde(rename = "privacyLinkText")]
	privacy_link_text: Option<String>,
	#[serde(rename = "saveButtonText")]
	save_button_text: Option<String>,
}

impl V1RegistrationOrgScreenText {
	pub fn new() -> V1RegistrationOrgScreenText {
		V1RegistrationOrgScreenText {
			title: None,
			description: None,
			orgname_label: None,
			firstname_label: None,
			lastname_label: None,
			username_label: None,
			email_label: None,
			password_label: None,
			password_confirm_label: None,
			tos_and_privacy_label: None,
			tos_confirm: None,
			tos_link_text: None,
			privacy_confirm: None,
			privacy_link_text: None,
			save_button_text: None,
		}
	}

	pub fn set_title(&mut self, title: String) {
		self.title = Some(title);
	}

	pub fn with_title(mut self, title: String) -> V1RegistrationOrgScreenText {
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

	pub fn with_description(mut self, description: String) -> V1RegistrationOrgScreenText {
		self.description = Some(description);
		self
	}

	pub fn description(&self) -> Option<&String> {
		self.description.as_ref()
	}

	pub fn reset_description(&mut self) {
		self.description = None;
	}

	pub fn set_orgname_label(&mut self, orgname_label: String) {
		self.orgname_label = Some(orgname_label);
	}

	pub fn with_orgname_label(mut self, orgname_label: String) -> V1RegistrationOrgScreenText {
		self.orgname_label = Some(orgname_label);
		self
	}

	pub fn orgname_label(&self) -> Option<&String> {
		self.orgname_label.as_ref()
	}

	pub fn reset_orgname_label(&mut self) {
		self.orgname_label = None;
	}

	pub fn set_firstname_label(&mut self, firstname_label: String) {
		self.firstname_label = Some(firstname_label);
	}

	pub fn with_firstname_label(mut self, firstname_label: String) -> V1RegistrationOrgScreenText {
		self.firstname_label = Some(firstname_label);
		self
	}

	pub fn firstname_label(&self) -> Option<&String> {
		self.firstname_label.as_ref()
	}

	pub fn reset_firstname_label(&mut self) {
		self.firstname_label = None;
	}

	pub fn set_lastname_label(&mut self, lastname_label: String) {
		self.lastname_label = Some(lastname_label);
	}

	pub fn with_lastname_label(mut self, lastname_label: String) -> V1RegistrationOrgScreenText {
		self.lastname_label = Some(lastname_label);
		self
	}

	pub fn lastname_label(&self) -> Option<&String> {
		self.lastname_label.as_ref()
	}

	pub fn reset_lastname_label(&mut self) {
		self.lastname_label = None;
	}

	pub fn set_username_label(&mut self, username_label: String) {
		self.username_label = Some(username_label);
	}

	pub fn with_username_label(mut self, username_label: String) -> V1RegistrationOrgScreenText {
		self.username_label = Some(username_label);
		self
	}

	pub fn username_label(&self) -> Option<&String> {
		self.username_label.as_ref()
	}

	pub fn reset_username_label(&mut self) {
		self.username_label = None;
	}

	pub fn set_email_label(&mut self, email_label: String) {
		self.email_label = Some(email_label);
	}

	pub fn with_email_label(mut self, email_label: String) -> V1RegistrationOrgScreenText {
		self.email_label = Some(email_label);
		self
	}

	pub fn email_label(&self) -> Option<&String> {
		self.email_label.as_ref()
	}

	pub fn reset_email_label(&mut self) {
		self.email_label = None;
	}

	pub fn set_password_label(&mut self, password_label: String) {
		self.password_label = Some(password_label);
	}

	pub fn with_password_label(mut self, password_label: String) -> V1RegistrationOrgScreenText {
		self.password_label = Some(password_label);
		self
	}

	pub fn password_label(&self) -> Option<&String> {
		self.password_label.as_ref()
	}

	pub fn reset_password_label(&mut self) {
		self.password_label = None;
	}

	pub fn set_password_confirm_label(&mut self, password_confirm_label: String) {
		self.password_confirm_label = Some(password_confirm_label);
	}

	pub fn with_password_confirm_label(
		mut self,
		password_confirm_label: String,
	) -> V1RegistrationOrgScreenText {
		self.password_confirm_label = Some(password_confirm_label);
		self
	}

	pub fn password_confirm_label(&self) -> Option<&String> {
		self.password_confirm_label.as_ref()
	}

	pub fn reset_password_confirm_label(&mut self) {
		self.password_confirm_label = None;
	}

	pub fn set_tos_and_privacy_label(&mut self, tos_and_privacy_label: String) {
		self.tos_and_privacy_label = Some(tos_and_privacy_label);
	}

	pub fn with_tos_and_privacy_label(
		mut self,
		tos_and_privacy_label: String,
	) -> V1RegistrationOrgScreenText {
		self.tos_and_privacy_label = Some(tos_and_privacy_label);
		self
	}

	pub fn tos_and_privacy_label(&self) -> Option<&String> {
		self.tos_and_privacy_label.as_ref()
	}

	pub fn reset_tos_and_privacy_label(&mut self) {
		self.tos_and_privacy_label = None;
	}

	pub fn set_tos_confirm(&mut self, tos_confirm: String) {
		self.tos_confirm = Some(tos_confirm);
	}

	pub fn with_tos_confirm(mut self, tos_confirm: String) -> V1RegistrationOrgScreenText {
		self.tos_confirm = Some(tos_confirm);
		self
	}

	pub fn tos_confirm(&self) -> Option<&String> {
		self.tos_confirm.as_ref()
	}

	pub fn reset_tos_confirm(&mut self) {
		self.tos_confirm = None;
	}

	pub fn set_tos_link_text(&mut self, tos_link_text: String) {
		self.tos_link_text = Some(tos_link_text);
	}

	pub fn with_tos_link_text(mut self, tos_link_text: String) -> V1RegistrationOrgScreenText {
		self.tos_link_text = Some(tos_link_text);
		self
	}

	pub fn tos_link_text(&self) -> Option<&String> {
		self.tos_link_text.as_ref()
	}

	pub fn reset_tos_link_text(&mut self) {
		self.tos_link_text = None;
	}

	pub fn set_privacy_confirm(&mut self, privacy_confirm: String) {
		self.privacy_confirm = Some(privacy_confirm);
	}

	pub fn with_privacy_confirm(mut self, privacy_confirm: String) -> V1RegistrationOrgScreenText {
		self.privacy_confirm = Some(privacy_confirm);
		self
	}

	pub fn privacy_confirm(&self) -> Option<&String> {
		self.privacy_confirm.as_ref()
	}

	pub fn reset_privacy_confirm(&mut self) {
		self.privacy_confirm = None;
	}

	pub fn set_privacy_link_text(&mut self, privacy_link_text: String) {
		self.privacy_link_text = Some(privacy_link_text);
	}

	pub fn with_privacy_link_text(
		mut self,
		privacy_link_text: String,
	) -> V1RegistrationOrgScreenText {
		self.privacy_link_text = Some(privacy_link_text);
		self
	}

	pub fn privacy_link_text(&self) -> Option<&String> {
		self.privacy_link_text.as_ref()
	}

	pub fn reset_privacy_link_text(&mut self) {
		self.privacy_link_text = None;
	}

	pub fn set_save_button_text(&mut self, save_button_text: String) {
		self.save_button_text = Some(save_button_text);
	}

	pub fn with_save_button_text(
		mut self,
		save_button_text: String,
	) -> V1RegistrationOrgScreenText {
		self.save_button_text = Some(save_button_text);
		self
	}

	pub fn save_button_text(&self) -> Option<&String> {
		self.save_button_text.as_ref()
	}

	pub fn reset_save_button_text(&mut self) {
		self.save_button_text = None;
	}
}
