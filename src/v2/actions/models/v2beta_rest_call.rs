/*
 * Action Service
 *
 * This API is intended to manage custom executions (previously known as
 * actions) in a ZITADEL instance. This service is in beta state. It can AND
 * will continue breaking until a stable version is released.
 *
 * OpenAPI spec version: 2.0-beta
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V2betaRestCall {
	/// Define if any error stops the whole execution. By default the process
	/// continues as normal.
	#[serde(rename = "interruptOnError")]
	interrupt_on_error: Option<bool>,
}

impl V2betaRestCall {
	pub fn new() -> V2betaRestCall {
		V2betaRestCall { interrupt_on_error: None }
	}

	pub fn set_interrupt_on_error(&mut self, interrupt_on_error: bool) {
		self.interrupt_on_error = Some(interrupt_on_error);
	}

	pub fn with_interrupt_on_error(mut self, interrupt_on_error: bool) -> V2betaRestCall {
		self.interrupt_on_error = Some(interrupt_on_error);
		self
	}

	pub fn interrupt_on_error(&self) -> Option<&bool> {
		self.interrupt_on_error.as_ref()
	}

	pub fn reset_interrupt_on_error(&mut self) {
		self.interrupt_on_error = None;
	}
}
