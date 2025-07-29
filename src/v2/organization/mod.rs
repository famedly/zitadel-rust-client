// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

//! Implementation of the client for the zitadel organization's v2 api
mod models;

use anyhow_ext::Result;
use anyhow_trace::anyhow_trace;
pub use models::*;

use super::Zitadel;

#[anyhow_trace]
impl Zitadel {
	/// Creates a new organization. [Docs](https://zitadel.com/docs/apis/resources/org_service_v2/organization-service-add-organization)
	pub async fn create_organization_with_admin(
		&self,
		body: V2AddOrganizationRequest,
	) -> Result<V2AddOrganizationResponse> {
		let request = self.client.post(self.make_url("v2/organizations")?).json(&body).build()?;

		Ok(self.send_request(request).await?)
	}
}
