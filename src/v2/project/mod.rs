// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

//! Implementation of the client for the zitadel project api
mod models;

use anyhow_ext::Result;
use anyhow_trace::anyhow_trace;
pub use models::*;

use super::Zitadel;

#[anyhow_trace]
impl Zitadel {
	/// Get granted project by id [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-get-granted-project-by-id)
	pub async fn get_granted_project_by_id(
		&self,
		project_id: &str,
		grant_id: &str,
	) -> Result<V1ProjectGrantResponse> {
		let request = self
			.client
			.get(self.make_url(&format!(
				"/management/v1/granted_projects/{project_id}/grants/{grant_id}"
			))?)
			.build()?;

		Ok(self.send_request(request).await?)
	}
}
