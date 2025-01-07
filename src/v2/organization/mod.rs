//! Implementation of the client for the zitadel organization's v2 api
mod models;

use anyhow::{Context, Result};
pub use models::*;

use super::Zitadel;

impl Zitadel {
	/// Creates a new organization. [Docs](https://zitadel.com/docs/apis/resources/org_service_v2/organization-service-add-organization)
	pub async fn create_organization_with_admin(
		&mut self,
		body: V2AddOrganizationRequest,
	) -> Result<V2AddOrganizationResponse> {
		let request = self
			.client
			.post(self.make_url("v2/organizations")?)
			.json(&body)
			.build()
			.context("Error building create_organization_with_admin request")?;

		self.send_request(request).await
	}
}
