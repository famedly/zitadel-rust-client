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

use crate::v2::users::models;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SendPasswordResetLink {
	#[serde(rename = "notificationType")]
	notification_type: Option<models::NotificationType>,
	/// \"Optionally set a url_template, which will be used in the password
	/// reset mail sent by ZITADEL to guide the user to your password change
	/// page. If no template is set, the default ZITADEL url will be used.\"
	#[serde(rename = "urlTemplate")]
	url_template: Option<String>,
}

impl SendPasswordResetLink {
	pub fn new() -> SendPasswordResetLink {
		SendPasswordResetLink { notification_type: None, url_template: None }
	}

	pub fn set_notification_type(&mut self, notification_type: models::NotificationType) {
		self.notification_type = Some(notification_type);
	}

	pub fn with_notification_type(
		mut self,
		notification_type: models::NotificationType,
	) -> SendPasswordResetLink {
		self.notification_type = Some(notification_type);
		self
	}

	pub fn notification_type(&self) -> Option<&models::NotificationType> {
		self.notification_type.as_ref()
	}

	pub fn reset_notification_type(&mut self) {
		self.notification_type = None;
	}

	pub fn set_url_template(&mut self, url_template: String) {
		self.url_template = Some(url_template);
	}

	pub fn with_url_template(mut self, url_template: String) -> SendPasswordResetLink {
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
