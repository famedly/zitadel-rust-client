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
pub struct User {
	#[serde(rename = "userId")]
	user_id: Option<String>,
	#[serde(rename = "details")]
	details: Option<models::Details>,
	/// current state of the user
	#[serde(rename = "state")]
	state: Option<models::UserState>,
	#[serde(rename = "username")]
	username: Option<String>,
	#[serde(rename = "loginNames")]
	login_names: Option<Vec<String>>,
	#[serde(rename = "preferredLoginName")]
	preferred_login_name: Option<String>,
	/// one of type use human or machine
	#[serde(rename = "human")]
	human: Option<models::HumanUser>,
	/// one of type use human or machine
	#[serde(rename = "machine")]
	machine: Option<models::MachineUser>,
}

impl User {
	pub fn new() -> User {
		User {
			user_id: None,
			details: None,
			state: None,
			username: None,
			login_names: None,
			preferred_login_name: None,
			human: None,
			machine: None,
		}
	}

	pub fn set_user_id(&mut self, user_id: String) {
		self.user_id = Some(user_id);
	}

	pub fn with_user_id(mut self, user_id: String) -> User {
		self.user_id = Some(user_id);
		self
	}

	pub fn user_id(&self) -> Option<&String> {
		self.user_id.as_ref()
	}

	pub fn reset_user_id(&mut self) {
		self.user_id = None;
	}

	pub fn set_details(&mut self, details: models::Details) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: models::Details) -> User {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::Details> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_state(&mut self, state: models::UserState) {
		self.state = Some(state);
	}

	pub fn with_state(mut self, state: models::UserState) -> User {
		self.state = Some(state);
		self
	}

	pub fn state(&self) -> Option<&models::UserState> {
		self.state.as_ref()
	}

	pub fn reset_state(&mut self) {
		self.state = None;
	}

	pub fn set_username(&mut self, username: String) {
		self.username = Some(username);
	}

	pub fn with_username(mut self, username: String) -> User {
		self.username = Some(username);
		self
	}

	pub fn username(&self) -> Option<&String> {
		self.username.as_ref()
	}

	pub fn reset_username(&mut self) {
		self.username = None;
	}

	pub fn set_login_names(&mut self, login_names: Vec<String>) {
		self.login_names = Some(login_names);
	}

	pub fn with_login_names(mut self, login_names: Vec<String>) -> User {
		self.login_names = Some(login_names);
		self
	}

	pub fn login_names(&self) -> Option<&Vec<String>> {
		self.login_names.as_ref()
	}

	pub fn reset_login_names(&mut self) {
		self.login_names = None;
	}

	pub fn set_preferred_login_name(&mut self, preferred_login_name: String) {
		self.preferred_login_name = Some(preferred_login_name);
	}

	pub fn with_preferred_login_name(mut self, preferred_login_name: String) -> User {
		self.preferred_login_name = Some(preferred_login_name);
		self
	}

	pub fn preferred_login_name(&self) -> Option<&String> {
		self.preferred_login_name.as_ref()
	}

	pub fn reset_preferred_login_name(&mut self) {
		self.preferred_login_name = None;
	}

	pub fn set_human(&mut self, human: models::HumanUser) {
		self.human = Some(human);
	}

	pub fn with_human(mut self, human: models::HumanUser) -> User {
		self.human = Some(human);
		self
	}

	pub fn human(&self) -> Option<&models::HumanUser> {
		self.human.as_ref()
	}

	pub fn reset_human(&mut self) {
		self.human = None;
	}

	pub fn set_machine(&mut self, machine: models::MachineUser) {
		self.machine = Some(machine);
	}

	pub fn with_machine(mut self, machine: models::MachineUser) -> User {
		self.machine = Some(machine);
		self
	}

	pub fn machine(&self) -> Option<&models::MachineUser> {
		self.machine.as_ref()
	}

	pub fn reset_machine(&mut self) {
		self.machine = None;
	}
}
