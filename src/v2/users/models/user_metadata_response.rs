// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

use base64::prelude::{BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};

use crate::v2::users::models;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserMetadataResponse {
	#[serde(rename = "details")]
	details: Option<models::Details>,
	#[serde(rename = "key")]
	key: Option<String>,
	#[serde(rename = "value")]
	value: Option<String>,
}

impl UserMetadataResponse {
	pub fn new() -> UserMetadataResponse {
		UserMetadataResponse { details: None, key: None, value: None }
	}
	pub fn set_details(&mut self, details: models::Details) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: models::Details) -> UserMetadataResponse {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::Details> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}
	pub fn set_key(&mut self, key: String) {
		self.key = Some(key);
	}

	pub fn with_key(mut self, key: String) -> UserMetadataResponse {
		self.key = Some(key);
		self
	}

	pub fn key(&self) -> Option<&String> {
		self.key.as_ref()
	}

	pub fn set_value(&mut self, value: String) {
		self.value = Some(BASE64_STANDARD.encode(value));
	}

	pub fn with_value(mut self, value: String) -> UserMetadataResponse {
		self.value = Some(BASE64_STANDARD.encode(value));
		self
	}

	pub fn value(&self) -> Option<String> {
		self.value
			.as_ref()
			.and_then(|value| BASE64_STANDARD.decode(value).ok())
			.and_then(|value| String::from_utf8(value).ok())
	}
}
