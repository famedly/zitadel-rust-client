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
pub struct V1UpdateCustomPasswordAgePolicyRequest {
	/// Amount of days after which a password will expire. The user will be
	/// forced to change the password on the following authentication.
	#[serde(rename = "maxAgeDays")]
	max_age_days: Option<i64>,
	/// Amount of days after which the user should be notified of the upcoming
	/// expiry. ZITADEL will not notify the user.
	#[serde(rename = "expireWarnDays")]
	expire_warn_days: Option<i64>,
}

impl V1UpdateCustomPasswordAgePolicyRequest {
	pub fn new() -> V1UpdateCustomPasswordAgePolicyRequest {
		V1UpdateCustomPasswordAgePolicyRequest { max_age_days: None, expire_warn_days: None }
	}

	pub fn set_max_age_days(&mut self, max_age_days: i64) {
		self.max_age_days = Some(max_age_days);
	}

	pub fn with_max_age_days(
		mut self,
		max_age_days: i64,
	) -> V1UpdateCustomPasswordAgePolicyRequest {
		self.max_age_days = Some(max_age_days);
		self
	}

	pub fn max_age_days(&self) -> Option<&i64> {
		self.max_age_days.as_ref()
	}

	pub fn reset_max_age_days(&mut self) {
		self.max_age_days = None;
	}

	pub fn set_expire_warn_days(&mut self, expire_warn_days: i64) {
		self.expire_warn_days = Some(expire_warn_days);
	}

	pub fn with_expire_warn_days(
		mut self,
		expire_warn_days: i64,
	) -> V1UpdateCustomPasswordAgePolicyRequest {
		self.expire_warn_days = Some(expire_warn_days);
		self
	}

	pub fn expire_warn_days(&self) -> Option<&i64> {
		self.expire_warn_days.as_ref()
	}

	pub fn reset_expire_warn_days(&mut self) {
		self.expire_warn_days = None;
	}
}
