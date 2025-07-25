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
pub struct V2betaGetTargetResponse {
	#[serde(rename = "target")]
	target: Option<super::V2betaTarget>,
}

impl V2betaGetTargetResponse {
	pub fn new() -> V2betaGetTargetResponse {
		V2betaGetTargetResponse { target: None }
	}

	pub fn set_target(&mut self, target: super::V2betaTarget) {
		self.target = Some(target);
	}

	pub fn with_target(mut self, target: super::V2betaTarget) -> V2betaGetTargetResponse {
		self.target = Some(target);
		self
	}

	pub fn target(&self) -> Option<&super::V2betaTarget> {
		self.target.as_ref()
	}

	pub fn reset_target(&mut self) {
		self.target = None;
	}
}
