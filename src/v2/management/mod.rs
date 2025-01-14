//! Implementation of the client for the Zitadel Management api
mod models;

use anyhow::{Context, Result};
pub use models::*;

use super::Zitadel;

impl Zitadel {
	/// Create actions. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-create-action)
	pub async fn create_action(
		&self,
		body: V1CreateActionRequest,
	) -> Result<V1CreateActionResponse> {
		let request = self
			.client
			.post(self.make_url("management/v1/actions")?)
			.json(&body)
			.build()
			.context("Error building create_action request")?;

		self.send_request(request).await
	}

	/// Update action. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-update-action)
	pub async fn update_action(
		&self,
		action_id: String,
		body: ManagementServiceUpdateActionBody,
	) -> Result<V1UpdateActionResponse> {
		let request = self
			.client
			.put(self.make_url(&format!("management/v1/actions/{action_id}"))?)
			.json(&body)
			.build()
			.context("Error building update_action request")?;

		self.send_request(request).await
	}

	/// Delete action. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-delete-action)
	pub async fn delete_action(&self, action_id: String) -> Result<V1DeleteActionResponse> {
		let request = self
			.client
			.delete(self.make_url(&format!("management/v1/actions/{action_id}"))?)
			.json(&ManagementServiceDeleteActionBody::new())
			.build()
			.context("Error building delete_action request")?;

		self.send_request(request).await
	}

	/// Get a flow. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-get-flow)
	pub async fn get_flow(&self, flow_type: u32) -> Result<V1GetFlowResponse> {
		let request = self
			.client
			.get(self.make_url(&format!("management/v1/flows/{flow_type}"))?)
			.build()
			.context("Error building get_flow request")?;

		self.send_request(request).await
	}

	/// Set trigger actions. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-set-trigger-actions)
	pub async fn set_trigger_actions(
		&self,
		// TODO: Should we provide enums for these?
		flow_type: u32,
		trigger_type: u32,
		body: ManagementServiceSetTriggerActionsBody,
	) -> Result<V1SetTriggerActionsResponse> {
		let request = self
			.client
			.post(
				self.make_url(&format!("management/v1/flows/{flow_type}/trigger/{trigger_type}"))?,
			)
			.json(&body)
			.build()
			.context("Error building set_trigger_actions request")?;

		self.send_request(request).await
	}

	/// Create application. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-api-app)
	pub async fn create_application(
		&self,
		project_id: String,
		body: ManagementServiceAddApiAppBody,
	) -> Result<V1AddApiAppResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("management/v1/projects/{project_id}/apps/api"))?)
			.json(&body)
			.build()
			.context("Error building create_application request")?;

		self.send_request(request).await
	}
}
