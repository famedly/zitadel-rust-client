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
pub struct ListQuery {
	#[serde(rename = "offset")]
	offset: Option<String>,
	/// Maximum amount of events returned. The default is set to 1000 in https://github.com/zitadel/zitadel/blob/new-eventstore/cmd/zitadel/startup.yaml. If the limit exceeds the maximum configured ZITADEL will throw an error. If no limit is present the default is taken.
	#[serde(rename = "limit")]
	limit: Option<i64>,
	/// default is descending
	#[serde(rename = "asc")]
	asc: Option<bool>,
}

impl ListQuery {
	/// Object unspecific list filters like offset, limit and asc/desc.
	pub fn new() -> ListQuery {
		ListQuery { offset: None, limit: None, asc: None }
	}

	pub fn set_offset(&mut self, offset: String) {
		self.offset = Some(offset);
	}

	pub fn with_offset(mut self, offset: String) -> ListQuery {
		self.offset = Some(offset);
		self
	}

	pub fn offset(&self) -> Option<&String> {
		self.offset.as_ref()
	}

	pub fn reset_offset(&mut self) {
		self.offset = None;
	}

	pub fn set_limit(&mut self, limit: i64) {
		self.limit = Some(limit);
	}

	pub fn with_limit(mut self, limit: i64) -> ListQuery {
		self.limit = Some(limit);
		self
	}

	pub fn limit(&self) -> Option<&i64> {
		self.limit.as_ref()
	}

	pub fn reset_limit(&mut self) {
		self.limit = None;
	}

	pub fn set_asc(&mut self, asc: bool) {
		self.asc = Some(asc);
	}

	pub fn with_asc(mut self, asc: bool) -> ListQuery {
		self.asc = Some(asc);
		self
	}

	pub fn asc(&self) -> Option<&bool> {
		self.asc.as_ref()
	}

	pub fn reset_asc(&mut self) {
		self.asc = None;
	}
}