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
pub struct V1AzureAdTenant {
	#[serde(rename = "tenantType")]
	tenant_type: Option<models::V1AzureAdTenantType>,
	#[serde(rename = "tenantId")]
	tenant_id: Option<String>,
}

impl V1AzureAdTenant {
	pub fn new() -> V1AzureAdTenant {
		V1AzureAdTenant { tenant_type: None, tenant_id: None }
	}

	pub fn set_tenant_type(&mut self, tenant_type: models::V1AzureAdTenantType) {
		self.tenant_type = Some(tenant_type);
	}

	pub fn with_tenant_type(mut self, tenant_type: models::V1AzureAdTenantType) -> V1AzureAdTenant {
		self.tenant_type = Some(tenant_type);
		self
	}

	pub fn tenant_type(&self) -> Option<&models::V1AzureAdTenantType> {
		self.tenant_type.as_ref()
	}

	pub fn reset_tenant_type(&mut self) {
		self.tenant_type = None;
	}

	pub fn set_tenant_id(&mut self, tenant_id: String) {
		self.tenant_id = Some(tenant_id);
	}

	pub fn with_tenant_id(mut self, tenant_id: String) -> V1AzureAdTenant {
		self.tenant_id = Some(tenant_id);
		self
	}

	pub fn tenant_id(&self) -> Option<&String> {
		self.tenant_id.as_ref()
	}

	pub fn reset_tenant_id(&mut self) {
		self.tenant_id = None;
	}
}
