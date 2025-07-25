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
pub struct V2betaTarget {
	/// The unique identifier of the target.
	#[serde(rename = "id")]
	id: Option<String>,
	/// The timestamp of the target creation.
	#[serde(rename = "creationDate")]
	creation_date: Option<String>,
	/// The timestamp of the last change to the target (e.g. creation,
	/// activation, deactivation).
	#[serde(rename = "changeDate")]
	change_date: Option<String>,
	#[serde(rename = "name")]
	name: Option<String>,
	#[serde(rename = "restWebhook")]
	rest_webhook: Option<super::V2betaRestWebhook>,
	#[serde(rename = "restCall")]
	rest_call: Option<super::V2betaRestCall>,
	#[serde(rename = "restAsync")]
	rest_async: Option<super::V2betaRestAsync>,
	/// Timeout defines the duration until ZITADEL cancels the execution. If the
	/// target doesn't respond before this timeout expires, the the connection
	/// is closed and the action fails. Depending on the target type and
	/// possible setting on `interrupt_on_error` following targets will not be
	/// called. In case of a `rest_async` target only this specific target will
	/// fail, without any influence on other targets of the same execution.
	#[serde(rename = "timeout")]
	timeout: Option<String>,
	#[serde(rename = "endpoint")]
	endpoint: Option<String>,
	#[serde(rename = "signingKey")]
	signing_key: Option<String>,
}

impl V2betaTarget {
	pub fn new() -> V2betaTarget {
		V2betaTarget {
			id: None,
			creation_date: None,
			change_date: None,
			name: None,
			rest_webhook: None,
			rest_call: None,
			rest_async: None,
			timeout: None,
			endpoint: None,
			signing_key: None,
		}
	}

	pub fn set_id(&mut self, id: String) {
		self.id = Some(id);
	}

	pub fn with_id(mut self, id: String) -> V2betaTarget {
		self.id = Some(id);
		self
	}

	pub fn id(&self) -> Option<&String> {
		self.id.as_ref()
	}

	pub fn reset_id(&mut self) {
		self.id = None;
	}

	pub fn set_creation_date(&mut self, creation_date: String) {
		self.creation_date = Some(creation_date);
	}

	pub fn with_creation_date(mut self, creation_date: String) -> V2betaTarget {
		self.creation_date = Some(creation_date);
		self
	}

	pub fn creation_date(&self) -> Option<&String> {
		self.creation_date.as_ref()
	}

	pub fn reset_creation_date(&mut self) {
		self.creation_date = None;
	}

	pub fn set_change_date(&mut self, change_date: String) {
		self.change_date = Some(change_date);
	}

	pub fn with_change_date(mut self, change_date: String) -> V2betaTarget {
		self.change_date = Some(change_date);
		self
	}

	pub fn change_date(&self) -> Option<&String> {
		self.change_date.as_ref()
	}

	pub fn reset_change_date(&mut self) {
		self.change_date = None;
	}

	pub fn set_name(&mut self, name: String) {
		self.name = Some(name);
	}

	pub fn with_name(mut self, name: String) -> V2betaTarget {
		self.name = Some(name);
		self
	}

	pub fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	pub fn reset_name(&mut self) {
		self.name = None;
	}

	pub fn set_rest_webhook(&mut self, rest_webhook: super::V2betaRestWebhook) {
		self.rest_webhook = Some(rest_webhook);
	}

	pub fn with_rest_webhook(mut self, rest_webhook: super::V2betaRestWebhook) -> V2betaTarget {
		self.rest_webhook = Some(rest_webhook);
		self
	}

	pub fn rest_webhook(&self) -> Option<&super::V2betaRestWebhook> {
		self.rest_webhook.as_ref()
	}

	pub fn reset_rest_webhook(&mut self) {
		self.rest_webhook = None;
	}

	pub fn set_rest_call(&mut self, rest_call: super::V2betaRestCall) {
		self.rest_call = Some(rest_call);
	}

	pub fn with_rest_call(mut self, rest_call: super::V2betaRestCall) -> V2betaTarget {
		self.rest_call = Some(rest_call);
		self
	}

	pub fn rest_call(&self) -> Option<&super::V2betaRestCall> {
		self.rest_call.as_ref()
	}

	pub fn reset_rest_call(&mut self) {
		self.rest_call = None;
	}

	pub fn set_rest_async(&mut self, rest_async: super::V2betaRestAsync) {
		self.rest_async = Some(rest_async);
	}

	pub fn with_rest_async(mut self, rest_async: super::V2betaRestAsync) -> V2betaTarget {
		self.rest_async = Some(rest_async);
		self
	}

	pub fn rest_async(&self) -> Option<&super::V2betaRestAsync> {
		self.rest_async.as_ref()
	}

	pub fn reset_rest_async(&mut self) {
		self.rest_async = None;
	}

	pub fn set_timeout(&mut self, timeout: String) {
		self.timeout = Some(timeout);
	}

	pub fn with_timeout(mut self, timeout: String) -> V2betaTarget {
		self.timeout = Some(timeout);
		self
	}

	pub fn timeout(&self) -> Option<&String> {
		self.timeout.as_ref()
	}

	pub fn reset_timeout(&mut self) {
		self.timeout = None;
	}

	pub fn set_endpoint(&mut self, endpoint: String) {
		self.endpoint = Some(endpoint);
	}

	pub fn with_endpoint(mut self, endpoint: String) -> V2betaTarget {
		self.endpoint = Some(endpoint);
		self
	}

	pub fn endpoint(&self) -> Option<&String> {
		self.endpoint.as_ref()
	}

	pub fn reset_endpoint(&mut self) {
		self.endpoint = None;
	}

	pub fn set_signing_key(&mut self, signing_key: String) {
		self.signing_key = Some(signing_key);
	}

	pub fn with_signing_key(mut self, signing_key: String) -> V2betaTarget {
		self.signing_key = Some(signing_key);
		self
	}

	pub fn signing_key(&self) -> Option<&String> {
		self.signing_key.as_ref()
	}

	pub fn reset_signing_key(&mut self) {
		self.signing_key = None;
	}
}
