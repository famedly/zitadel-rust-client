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
pub struct V1InitMfaPromptScreenText {
	#[serde(rename = "title")]
	title: Option<String>,
	#[serde(rename = "description")]
	description: Option<String>,
	#[serde(rename = "otpOption")]
	otp_option: Option<String>,
	#[serde(rename = "u2fOption")]
	u2f_option: Option<String>,
	#[serde(rename = "skipButtonText")]
	skip_button_text: Option<String>,
	#[serde(rename = "nextButtonText")]
	next_button_text: Option<String>,
}

impl V1InitMfaPromptScreenText {
	pub fn new() -> V1InitMfaPromptScreenText {
		V1InitMfaPromptScreenText {
			title: None,
			description: None,
			otp_option: None,
			u2f_option: None,
			skip_button_text: None,
			next_button_text: None,
		}
	}

	pub fn set_title(&mut self, title: String) {
		self.title = Some(title);
	}

	pub fn with_title(mut self, title: String) -> V1InitMfaPromptScreenText {
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

	pub fn with_description(mut self, description: String) -> V1InitMfaPromptScreenText {
		self.description = Some(description);
		self
	}

	pub fn description(&self) -> Option<&String> {
		self.description.as_ref()
	}

	pub fn reset_description(&mut self) {
		self.description = None;
	}

	pub fn set_otp_option(&mut self, otp_option: String) {
		self.otp_option = Some(otp_option);
	}

	pub fn with_otp_option(mut self, otp_option: String) -> V1InitMfaPromptScreenText {
		self.otp_option = Some(otp_option);
		self
	}

	pub fn otp_option(&self) -> Option<&String> {
		self.otp_option.as_ref()
	}

	pub fn reset_otp_option(&mut self) {
		self.otp_option = None;
	}

	pub fn set_u2f_option(&mut self, u2f_option: String) {
		self.u2f_option = Some(u2f_option);
	}

	pub fn with_u2f_option(mut self, u2f_option: String) -> V1InitMfaPromptScreenText {
		self.u2f_option = Some(u2f_option);
		self
	}

	pub fn u2f_option(&self) -> Option<&String> {
		self.u2f_option.as_ref()
	}

	pub fn reset_u2f_option(&mut self) {
		self.u2f_option = None;
	}

	pub fn set_skip_button_text(&mut self, skip_button_text: String) {
		self.skip_button_text = Some(skip_button_text);
	}

	pub fn with_skip_button_text(mut self, skip_button_text: String) -> V1InitMfaPromptScreenText {
		self.skip_button_text = Some(skip_button_text);
		self
	}

	pub fn skip_button_text(&self) -> Option<&String> {
		self.skip_button_text.as_ref()
	}

	pub fn reset_skip_button_text(&mut self) {
		self.skip_button_text = None;
	}

	pub fn set_next_button_text(&mut self, next_button_text: String) {
		self.next_button_text = Some(next_button_text);
	}

	pub fn with_next_button_text(mut self, next_button_text: String) -> V1InitMfaPromptScreenText {
		self.next_button_text = Some(next_button_text);
		self
	}

	pub fn next_button_text(&self) -> Option<&String> {
		self.next_button_text.as_ref()
	}

	pub fn reset_next_button_text(&mut self) {
		self.next_button_text = None;
	}
}
