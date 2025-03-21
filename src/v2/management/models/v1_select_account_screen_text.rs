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
pub struct V1SelectAccountScreenText {
	#[serde(rename = "title")]
	title: Option<String>,
	#[serde(rename = "description")]
	description: Option<String>,
	#[serde(rename = "titleLinkingProcess")]
	title_linking_process: Option<String>,
	#[serde(rename = "descriptionLinkingProcess")]
	description_linking_process: Option<String>,
	#[serde(rename = "otherUser")]
	other_user: Option<String>,
	#[serde(rename = "sessionStateActive")]
	session_state_active: Option<String>,
	#[serde(rename = "sessionStateInactive")]
	session_state_inactive: Option<String>,
	#[serde(rename = "userMustBeMemberOfOrg")]
	user_must_be_member_of_org: Option<String>,
}

impl V1SelectAccountScreenText {
	pub fn new() -> V1SelectAccountScreenText {
		V1SelectAccountScreenText {
			title: None,
			description: None,
			title_linking_process: None,
			description_linking_process: None,
			other_user: None,
			session_state_active: None,
			session_state_inactive: None,
			user_must_be_member_of_org: None,
		}
	}

	pub fn set_title(&mut self, title: String) {
		self.title = Some(title);
	}

	pub fn with_title(mut self, title: String) -> V1SelectAccountScreenText {
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

	pub fn with_description(mut self, description: String) -> V1SelectAccountScreenText {
		self.description = Some(description);
		self
	}

	pub fn description(&self) -> Option<&String> {
		self.description.as_ref()
	}

	pub fn reset_description(&mut self) {
		self.description = None;
	}

	pub fn set_title_linking_process(&mut self, title_linking_process: String) {
		self.title_linking_process = Some(title_linking_process);
	}

	pub fn with_title_linking_process(
		mut self,
		title_linking_process: String,
	) -> V1SelectAccountScreenText {
		self.title_linking_process = Some(title_linking_process);
		self
	}

	pub fn title_linking_process(&self) -> Option<&String> {
		self.title_linking_process.as_ref()
	}

	pub fn reset_title_linking_process(&mut self) {
		self.title_linking_process = None;
	}

	pub fn set_description_linking_process(&mut self, description_linking_process: String) {
		self.description_linking_process = Some(description_linking_process);
	}

	pub fn with_description_linking_process(
		mut self,
		description_linking_process: String,
	) -> V1SelectAccountScreenText {
		self.description_linking_process = Some(description_linking_process);
		self
	}

	pub fn description_linking_process(&self) -> Option<&String> {
		self.description_linking_process.as_ref()
	}

	pub fn reset_description_linking_process(&mut self) {
		self.description_linking_process = None;
	}

	pub fn set_other_user(&mut self, other_user: String) {
		self.other_user = Some(other_user);
	}

	pub fn with_other_user(mut self, other_user: String) -> V1SelectAccountScreenText {
		self.other_user = Some(other_user);
		self
	}

	pub fn other_user(&self) -> Option<&String> {
		self.other_user.as_ref()
	}

	pub fn reset_other_user(&mut self) {
		self.other_user = None;
	}

	pub fn set_session_state_active(&mut self, session_state_active: String) {
		self.session_state_active = Some(session_state_active);
	}

	pub fn with_session_state_active(
		mut self,
		session_state_active: String,
	) -> V1SelectAccountScreenText {
		self.session_state_active = Some(session_state_active);
		self
	}

	pub fn session_state_active(&self) -> Option<&String> {
		self.session_state_active.as_ref()
	}

	pub fn reset_session_state_active(&mut self) {
		self.session_state_active = None;
	}

	pub fn set_session_state_inactive(&mut self, session_state_inactive: String) {
		self.session_state_inactive = Some(session_state_inactive);
	}

	pub fn with_session_state_inactive(
		mut self,
		session_state_inactive: String,
	) -> V1SelectAccountScreenText {
		self.session_state_inactive = Some(session_state_inactive);
		self
	}

	pub fn session_state_inactive(&self) -> Option<&String> {
		self.session_state_inactive.as_ref()
	}

	pub fn reset_session_state_inactive(&mut self) {
		self.session_state_inactive = None;
	}

	pub fn set_user_must_be_member_of_org(&mut self, user_must_be_member_of_org: String) {
		self.user_must_be_member_of_org = Some(user_must_be_member_of_org);
	}

	pub fn with_user_must_be_member_of_org(
		mut self,
		user_must_be_member_of_org: String,
	) -> V1SelectAccountScreenText {
		self.user_must_be_member_of_org = Some(user_must_be_member_of_org);
		self
	}

	pub fn user_must_be_member_of_org(&self) -> Option<&String> {
		self.user_must_be_member_of_org.as_ref()
	}

	pub fn reset_user_must_be_member_of_org(&mut self) {
		self.user_must_be_member_of_org = None;
	}
}
