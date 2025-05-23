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
pub struct RpcStatus {
	#[serde(rename = "code")]
	code: Option<i32>,
	#[serde(rename = "message")]
	message: Option<String>,
	#[serde(rename = "details")]
	details: Option<Vec<models::ProtobufAny>>,
}

impl RpcStatus {
	pub fn new() -> RpcStatus {
		RpcStatus { code: None, message: None, details: None }
	}

	pub fn set_code(&mut self, code: i32) {
		self.code = Some(code);
	}

	pub fn with_code(mut self, code: i32) -> RpcStatus {
		self.code = Some(code);
		self
	}

	pub fn code(&self) -> Option<&i32> {
		self.code.as_ref()
	}

	pub fn reset_code(&mut self) {
		self.code = None;
	}

	pub fn set_message(&mut self, message: String) {
		self.message = Some(message);
	}

	pub fn with_message(mut self, message: String) -> RpcStatus {
		self.message = Some(message);
		self
	}

	pub fn message(&self) -> Option<&String> {
		self.message.as_ref()
	}

	pub fn reset_message(&mut self) {
		self.message = None;
	}

	pub fn set_details(&mut self, details: Vec<models::ProtobufAny>) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: Vec<models::ProtobufAny>) -> RpcStatus {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&Vec<models::ProtobufAny>> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}
}
