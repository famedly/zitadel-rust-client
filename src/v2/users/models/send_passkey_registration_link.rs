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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SendPasskeyRegistrationLink {
	/// \"Optionally set a url_template, which will be used in the mail sent by
	/// ZITADEL to guide the user to your passkey registration page. If no
	/// template is set, the default ZITADEL url will be used.\"
	#[serde(rename = "urlTemplate")]
	url_template: Option<String>,
}

impl SendPasskeyRegistrationLink {
	pub fn new() -> SendPasskeyRegistrationLink {
		SendPasskeyRegistrationLink { url_template: None }
	}

	pub fn set_url_template(&mut self, url_template: String) {
		self.url_template = Some(url_template);
	}

	pub fn with_url_template(mut self, url_template: String) -> SendPasskeyRegistrationLink {
		self.url_template = Some(url_template);
		self
	}

	pub fn url_template(&self) -> Option<&String> {
		self.url_template.as_ref()
	}

	pub fn reset_url_template(&mut self) {
		self.url_template = None;
	}
}