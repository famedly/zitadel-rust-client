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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Details {
	/// on read: the sequence of the last event reduced by the projection  on
	/// manipulation: the timestamp of the event(s) added by the manipulation
	#[serde(rename = "sequence")]
	sequence: Option<String>,
	/// on read: the timestamp of the first event of the object
	/// on create: the timestamp of the event(s) added by the manipulation
	/// Only available on v1 api responses!
	#[serde(rename = "creationDate")]
	creation_date: Option<String>,
	/// on read: the timestamp of the last event reduced by the projection  on
	/// manipulation: the timestamp of the event(s) added by the manipulation
	#[serde(rename = "changeDate")]
	change_date: Option<String>,
	#[serde(rename = "resourceOwner")]
	resource_owner: Option<String>,
}

impl Details {
	pub fn new() -> Details {
		Details { sequence: None, creation_date: None, change_date: None, resource_owner: None }
	}

	pub fn set_sequence(&mut self, sequence: String) {
		self.sequence = Some(sequence);
	}

	pub fn with_sequence(mut self, sequence: String) -> Details {
		self.sequence = Some(sequence);
		self
	}

	pub fn sequence(&self) -> Option<&String> {
		self.sequence.as_ref()
	}

	pub fn reset_sequence(&mut self) {
		self.sequence = None;
	}

	pub fn set_creation_date(&mut self, creation_date: String) {
		self.creation_date = Some(creation_date);
	}

	pub fn with_creation_date(mut self, creation_date: String) -> Details {
		self.creation_date = Some(creation_date);
		self
	}

	pub fn creation_date(&self) -> Option<&String> {
		self.creation_date.as_ref()
	}

	pub fn reset_creation_date(&mut self) {
		self.creation_date = None;
	}

	pub fn set_change_date(&mut self, change_date: String) {
		self.change_date = Some(change_date);
	}

	pub fn with_change_date(mut self, change_date: String) -> Details {
		self.change_date = Some(change_date);
		self
	}

	pub fn change_date(&self) -> Option<&String> {
		self.change_date.as_ref()
	}

	pub fn reset_change_date(&mut self) {
		self.change_date = None;
	}

	pub fn set_resource_owner(&mut self, resource_owner: String) {
		self.resource_owner = Some(resource_owner);
	}

	pub fn with_resource_owner(mut self, resource_owner: String) -> Details {
		self.resource_owner = Some(resource_owner);
		self
	}

	pub fn resource_owner(&self) -> Option<&String> {
		self.resource_owner.as_ref()
	}

	pub fn reset_resource_owner(&mut self) {
		self.resource_owner = None;
	}
}
