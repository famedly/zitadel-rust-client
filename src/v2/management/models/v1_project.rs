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
pub struct V1Project {
	#[serde(rename = "id")]
	id: Option<String>,
	#[serde(rename = "details")]
	details: Option<models::V1ObjectDetails>,
	#[serde(rename = "name")]
	name: Option<String>,
	/// current state of the project
	#[serde(rename = "state")]
	state: Option<models::V1ProjectState>,
	#[serde(rename = "projectRoleAssertion")]
	project_role_assertion: Option<bool>,
	#[serde(rename = "projectRoleCheck")]
	project_role_check: Option<bool>,
	#[serde(rename = "hasProjectCheck")]
	has_project_check: Option<bool>,
	#[serde(rename = "privateLabelingSetting")]
	private_labeling_setting: Option<models::V1PrivateLabelingSetting>,
}

impl V1Project {
	pub fn new() -> V1Project {
		V1Project {
			id: None,
			details: None,
			name: None,
			state: None,
			project_role_assertion: None,
			project_role_check: None,
			has_project_check: None,
			private_labeling_setting: None,
		}
	}

	pub fn set_id(&mut self, id: String) {
		self.id = Some(id);
	}

	pub fn with_id(mut self, id: String) -> V1Project {
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

	pub fn with_details(mut self, details: models::V1ObjectDetails) -> V1Project {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::V1ObjectDetails> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_name(&mut self, name: String) {
		self.name = Some(name);
	}

	pub fn with_name(mut self, name: String) -> V1Project {
		self.name = Some(name);
		self
	}

	pub fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	pub fn reset_name(&mut self) {
		self.name = None;
	}

	pub fn set_state(&mut self, state: models::V1ProjectState) {
		self.state = Some(state);
	}

	pub fn with_state(mut self, state: models::V1ProjectState) -> V1Project {
		self.state = Some(state);
		self
	}

	pub fn state(&self) -> Option<&models::V1ProjectState> {
		self.state.as_ref()
	}

	pub fn reset_state(&mut self) {
		self.state = None;
	}

	pub fn set_project_role_assertion(&mut self, project_role_assertion: bool) {
		self.project_role_assertion = Some(project_role_assertion);
	}

	pub fn with_project_role_assertion(mut self, project_role_assertion: bool) -> V1Project {
		self.project_role_assertion = Some(project_role_assertion);
		self
	}

	pub fn project_role_assertion(&self) -> Option<&bool> {
		self.project_role_assertion.as_ref()
	}

	pub fn reset_project_role_assertion(&mut self) {
		self.project_role_assertion = None;
	}

	pub fn set_project_role_check(&mut self, project_role_check: bool) {
		self.project_role_check = Some(project_role_check);
	}

	pub fn with_project_role_check(mut self, project_role_check: bool) -> V1Project {
		self.project_role_check = Some(project_role_check);
		self
	}

	pub fn project_role_check(&self) -> Option<&bool> {
		self.project_role_check.as_ref()
	}

	pub fn reset_project_role_check(&mut self) {
		self.project_role_check = None;
	}

	pub fn set_has_project_check(&mut self, has_project_check: bool) {
		self.has_project_check = Some(has_project_check);
	}

	pub fn with_has_project_check(mut self, has_project_check: bool) -> V1Project {
		self.has_project_check = Some(has_project_check);
		self
	}

	pub fn has_project_check(&self) -> Option<&bool> {
		self.has_project_check.as_ref()
	}

	pub fn reset_has_project_check(&mut self) {
		self.has_project_check = None;
	}

	pub fn set_private_labeling_setting(
		&mut self,
		private_labeling_setting: models::V1PrivateLabelingSetting,
	) {
		self.private_labeling_setting = Some(private_labeling_setting);
	}

	pub fn with_private_labeling_setting(
		mut self,
		private_labeling_setting: models::V1PrivateLabelingSetting,
	) -> V1Project {
		self.private_labeling_setting = Some(private_labeling_setting);
		self
	}

	pub fn private_labeling_setting(&self) -> Option<&models::V1PrivateLabelingSetting> {
		self.private_labeling_setting.as_ref()
	}

	pub fn reset_private_labeling_setting(&mut self) {
		self.private_labeling_setting = None;
	}
}
