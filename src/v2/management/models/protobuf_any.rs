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
pub struct ProtobufAny {
	#[serde(rename = "@type")]
	_type: Option<String>,
}

impl ProtobufAny {
	pub fn new() -> ProtobufAny {
		ProtobufAny { _type: None }
	}

	pub fn set__type(&mut self, _type: String) {
		self._type = Some(_type);
	}

	pub fn with__type(mut self, _type: String) -> ProtobufAny {
		self._type = Some(_type);
		self
	}

	pub fn _type(&self) -> Option<&String> {
		self._type.as_ref()
	}

	pub fn reset__type(&mut self) {
		self._type = None;
	}
}
