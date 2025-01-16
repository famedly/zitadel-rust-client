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
pub struct V1MfaProvidersText {
	#[serde(rename = "chooseOther")]
	choose_other: Option<String>,
	#[serde(rename = "otp")]
	otp: Option<String>,
	#[serde(rename = "u2f")]
	u2f: Option<String>,
}

impl V1MfaProvidersText {
	pub fn new() -> V1MfaProvidersText {
		V1MfaProvidersText { choose_other: None, otp: None, u2f: None }
	}

	pub fn set_choose_other(&mut self, choose_other: String) {
		self.choose_other = Some(choose_other);
	}

	pub fn with_choose_other(mut self, choose_other: String) -> V1MfaProvidersText {
		self.choose_other = Some(choose_other);
		self
	}

	pub fn choose_other(&self) -> Option<&String> {
		self.choose_other.as_ref()
	}

	pub fn reset_choose_other(&mut self) {
		self.choose_other = None;
	}

	pub fn set_otp(&mut self, otp: String) {
		self.otp = Some(otp);
	}

	pub fn with_otp(mut self, otp: String) -> V1MfaProvidersText {
		self.otp = Some(otp);
		self
	}

	pub fn otp(&self) -> Option<&String> {
		self.otp.as_ref()
	}

	pub fn reset_otp(&mut self) {
		self.otp = None;
	}

	pub fn set_u2f(&mut self, u2f: String) {
		self.u2f = Some(u2f);
	}

	pub fn with_u2f(mut self, u2f: String) -> V1MfaProvidersText {
		self.u2f = Some(u2f);
		self
	}

	pub fn u2f(&self) -> Option<&String> {
		self.u2f.as_ref()
	}

	pub fn reset_u2f(&mut self) {
		self.u2f = None;
	}
}
