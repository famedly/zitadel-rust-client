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
pub struct V1AddOrgDomainRequest {
	#[serde(rename = "domain")]
	domain: String,
}

impl V1AddOrgDomainRequest {
	pub fn new(domain: String) -> V1AddOrgDomainRequest {
		V1AddOrgDomainRequest { domain: domain }
	}

	pub fn set_domain(&mut self, domain: String) {
		self.domain = domain;
	}

	pub fn with_domain(mut self, domain: String) -> V1AddOrgDomainRequest {
		self.domain = domain;
		self
	}

	pub fn domain(&self) -> &String {
		&self.domain
	}
}
