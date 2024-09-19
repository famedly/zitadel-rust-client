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
pub struct ListDetails {
	#[serde(rename = "totalResult")]
	total_result: Option<String>,
	#[serde(rename = "processedSequence")]
	processed_sequence: Option<String>,
	/// the last time the projection got updated
	#[serde(rename = "timestamp")]
	timestamp: Option<String>,
}

impl ListDetails {
	pub fn new() -> ListDetails {
		ListDetails { total_result: None, processed_sequence: None, timestamp: None }
	}

	pub fn set_total_result(&mut self, total_result: String) {
		self.total_result = Some(total_result);
	}

	pub fn with_total_result(mut self, total_result: String) -> ListDetails {
		self.total_result = Some(total_result);
		self
	}

	pub fn total_result(&self) -> Option<&String> {
		self.total_result.as_ref()
	}

	pub fn reset_total_result(&mut self) {
		self.total_result = None;
	}

	pub fn set_processed_sequence(&mut self, processed_sequence: String) {
		self.processed_sequence = Some(processed_sequence);
	}

	pub fn with_processed_sequence(mut self, processed_sequence: String) -> ListDetails {
		self.processed_sequence = Some(processed_sequence);
		self
	}

	pub fn processed_sequence(&self) -> Option<&String> {
		self.processed_sequence.as_ref()
	}

	pub fn reset_processed_sequence(&mut self) {
		self.processed_sequence = None;
	}

	pub fn set_timestamp(&mut self, timestamp: String) {
		self.timestamp = Some(timestamp);
	}

	pub fn with_timestamp(mut self, timestamp: String) -> ListDetails {
		self.timestamp = Some(timestamp);
		self
	}

	pub fn timestamp(&self) -> Option<&String> {
		self.timestamp.as_ref()
	}

	pub fn reset_timestamp(&mut self) {
		self.timestamp = None;
	}
}