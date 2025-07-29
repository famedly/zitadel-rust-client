// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

use crate::v2::project::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1ProjectGrant {
	/// The unique identifier of the organization which granted the project to
	/// the granted_organization_id.
	#[serde(rename = "projectOwnerId")]
	organization_id: Option<String>,
	/// The project owner name.
	#[serde(rename = "projectOwnerName")]
	project_owner_name: Option<String>,
	/// The ID of the organization the project is granted to.
	#[serde(rename = "grantedOrgId")]
	granted_organization_id: Option<String>,
	/// The name of the organization the project is granted to.
	#[serde(rename = "grantedOrgName")]
	granted_organization_name: Option<String>,
	/// The roles of the granted project.
	#[serde(rename = "grantedRoleKeys")]
	granted_role_keys: Option<Vec<String>>,
	/// The ID of the granted project.
	#[serde(rename = "projectId")]
	project_id: Option<String>,
	/// The name of the granted project.
	#[serde(rename = "projectName")]
	project_name: Option<String>,
	/// current state of the project
	#[serde(rename = "state")]
	state: Option<String>,
}

impl V1ProjectGrant {
	pub fn new() -> V1ProjectGrant {
		V1ProjectGrant {
			organization_id: None,
			project_owner_name: None,
			granted_organization_id: None,
			granted_organization_name: None,
			granted_role_keys: None,
			project_id: None,
			project_name: None,
			state: None,
		}
	}

	pub fn set_organization_id(&mut self, organization_id: String) {
		self.organization_id = Some(organization_id);
	}

	pub fn with_organization_id(mut self, organization_id: String) -> V1ProjectGrant {
		self.organization_id = Some(organization_id);
		self
	}

	pub fn organization_id(&self) -> Option<&String> {
		self.organization_id.as_ref()
	}

	pub fn reset_organization_id(&mut self) {
		self.organization_id = None;
	}

	pub fn set_creation_date(&mut self, creation_date: String) {
		self.project_owner_name = Some(creation_date);
	}

	pub fn with_creation_date(mut self, creation_date: String) -> V1ProjectGrant {
		self.project_owner_name = Some(creation_date);
		self
	}

	pub fn creation_date(&self) -> Option<&String> {
		self.project_owner_name.as_ref()
	}

	pub fn reset_creation_date(&mut self) {
		self.project_owner_name = None;
	}

	pub fn set_granted_organization_id(&mut self, granted_organization_id: String) {
		self.granted_organization_id = Some(granted_organization_id);
	}

	pub fn with_granted_organization_id(
		mut self,
		granted_organization_id: String,
	) -> V1ProjectGrant {
		self.granted_organization_id = Some(granted_organization_id);
		self
	}

	pub fn granted_organization_id(&self) -> Option<&String> {
		self.granted_organization_id.as_ref()
	}

	pub fn reset_granted_organization_id(&mut self) {
		self.granted_organization_id = None;
	}

	pub fn set_granted_organization_name(&mut self, granted_organization_name: String) {
		self.granted_organization_name = Some(granted_organization_name);
	}

	pub fn with_granted_organization_name(
		mut self,
		granted_organization_name: String,
	) -> V1ProjectGrant {
		self.granted_organization_name = Some(granted_organization_name);
		self
	}

	pub fn granted_organization_name(&self) -> Option<&String> {
		self.granted_organization_name.as_ref()
	}

	pub fn reset_granted_organization_name(&mut self) {
		self.granted_organization_name = None;
	}

	pub fn set_granted_role_keys(&mut self, granted_role_keys: Vec<String>) {
		self.granted_role_keys = Some(granted_role_keys);
	}

	pub fn with_granted_role_keys(mut self, granted_role_keys: Vec<String>) -> V1ProjectGrant {
		self.granted_role_keys = Some(granted_role_keys);
		self
	}

	pub fn granted_role_keys(&self) -> Option<&Vec<String>> {
		self.granted_role_keys.as_ref()
	}

	pub fn reset_granted_role_keys(&mut self) {
		self.granted_role_keys = None;
	}

	pub fn set_project_id(&mut self, project_id: String) {
		self.project_id = Some(project_id);
	}

	pub fn with_project_id(mut self, project_id: String) -> V1ProjectGrant {
		self.project_id = Some(project_id);
		self
	}

	pub fn project_id(&self) -> Option<&String> {
		self.project_id.as_ref()
	}

	pub fn reset_project_id(&mut self) {
		self.project_id = None;
	}

	pub fn set_project_name(&mut self, project_name: String) {
		self.project_name = Some(project_name);
	}

	pub fn with_project_name(mut self, project_name: String) -> V1ProjectGrant {
		self.project_name = Some(project_name);
		self
	}

	pub fn project_name(&self) -> Option<&String> {
		self.project_name.as_ref()
	}

	pub fn reset_project_name(&mut self) {
		self.project_name = None;
	}

	pub fn set_state(&mut self, state: String) {
		self.state = Some(state);
	}

	pub fn with_state(mut self, state: String) -> V1ProjectGrant {
		self.state = Some(state);
		self
	}

	pub fn state(&self) -> Option<&String> {
		self.state.as_ref()
	}

	pub fn reset_state(&mut self) {
		self.state = None;
	}
}
