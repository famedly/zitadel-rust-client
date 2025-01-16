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
pub struct Projectv1Role {
	#[serde(rename = "key")]
	key: Option<String>,
	#[serde(rename = "details")]
	details: Option<models::V1ObjectDetails>,
	#[serde(rename = "displayName")]
	display_name: Option<String>,
	#[serde(rename = "group")]
	group: Option<String>,
}

impl Projectv1Role {
	pub fn new() -> Projectv1Role {
		Projectv1Role { key: None, details: None, display_name: None, group: None }
	}

	pub fn set_key(&mut self, key: String) {
		self.key = Some(key);
	}

	pub fn with_key(mut self, key: String) -> Projectv1Role {
		self.key = Some(key);
		self
	}

	pub fn key(&self) -> Option<&String> {
		self.key.as_ref()
	}

	pub fn reset_key(&mut self) {
		self.key = None;
	}

	pub fn set_details(&mut self, details: models::V1ObjectDetails) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: models::V1ObjectDetails) -> Projectv1Role {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::V1ObjectDetails> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_display_name(&mut self, display_name: String) {
		self.display_name = Some(display_name);
	}

	pub fn with_display_name(mut self, display_name: String) -> Projectv1Role {
		self.display_name = Some(display_name);
		self
	}

	pub fn display_name(&self) -> Option<&String> {
		self.display_name.as_ref()
	}

	pub fn reset_display_name(&mut self) {
		self.display_name = None;
	}

	pub fn set_group(&mut self, group: String) {
		self.group = Some(group);
	}

	pub fn with_group(mut self, group: String) -> Projectv1Role {
		self.group = Some(group);
		self
	}

	pub fn group(&self) -> Option<&String> {
		self.group.as_ref()
	}

	pub fn reset_group(&mut self) {
		self.group = None;
	}
}
