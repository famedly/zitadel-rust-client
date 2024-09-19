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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotificationType {
	#[serde(rename = "NOTIFICATION_TYPE_Unspecified")]
	Unspecified,
	#[serde(rename = "NOTIFICATION_TYPE_Email")]
	Email,
	#[serde(rename = "NOTIFICATION_TYPE_SMS")]
	Sms,
}

impl std::fmt::Display for NotificationType {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Unspecified => write!(f, "NOTIFICATION_TYPE_Unspecified"),
			Self::Email => write!(f, "NOTIFICATION_TYPE_Email"),
			Self::Sms => write!(f, "NOTIFICATION_TYPE_SMS"),
		}
	}
}

impl Default for NotificationType {
	fn default() -> NotificationType {
		Self::Unspecified
	}
}