//! V2 actions API.

mod models;

use std::collections::VecDeque;

use anyhow_ext::Result;
use futures::Stream;
pub use models::*;

use super::{
	pagination::{PaginationHandler, PaginationParams},
	Zitadel,
};

#[anyhow_trace::anyhow_trace]
impl Zitadel {
	/// [Create Target](https://zitadel.com/docs/apis/resources/action_service_v2/action-service-create-target)
	pub async fn create_target(
		&self,
		req: &V2betaCreateTargetRequest,
	) -> Result<V2betaCreateTargetResponse> {
		let request =
			self.client.post(self.make_url("v2beta/actions/targets")?).json(req).build()?;
		Ok(self.send_request(request).await?)
	}

	/// [Update Target](https://zitadel.com/docs/apis/resources/action_service_v2/action-service-update-target)
	pub async fn update_target(
		&self,
		id: &str,
		req: &ActionServiceUpdateTargetBody,
	) -> Result<V2betaUpdateTargetResponse> {
		let request = self
			.client
			.post(self.make_url("v2beta/actions/targets/")?.join(id)?)
			.json(req)
			.build()?;
		Ok(self.send_request(request).await?)
	}

	/// [Delete Target](https://zitadel.com/docs/apis/resources/action_service_v2/action-service-delete-target)
	pub async fn delete_target(&self, id: &str) -> Result<V2betaDeleteTargetResponse> {
		let request =
			self.client.delete(self.make_url("v2beta/actions/targets/")?.join(id)?).build()?;
		Ok(self.send_request(request).await?)
	}

	/// [List targets](https://zitadel.com/docs/apis/resources/action_service_v2/action-service-list-targets)
	pub fn list_targets(
		&self,
		params: Option<PaginationParams>,
		sort_by: Option<V2betaTargetFieldName>,
		filters: Option<Vec<V2betaTargetSearchFilter>>,
	) -> Result<impl Stream<Item = Result<V2betaTarget>> + Send + Sync> {
		Ok(PaginationHandler::<V2betaListTargetsRequest, _, _>::new(
			self.clone(),
			(params, sort_by, filters),
			self.make_url("v2beta/actions/targets/_search")?,
			None,
		))
	}

	/// [Set Execution | ZITADEL Docs](https://zitadel.com/docs/apis/resources/action_service_v2/action-service-set-execution)
	pub async fn set_execution(
		&self,
		req: &V2betaSetExecutionRequest,
	) -> Result<V2betaSetExecutionResponse> {
		let request =
			self.client.put(self.make_url("v2beta/actions/executions")?).json(req).build()?;
		Ok(self.send_request(request).await?)
	}

	/// [List Executions | ZITADEL Docs](https://zitadel.com/docs/apis/resources/action_service_v2/action-service-list-executions)
	pub fn list_executions<'a>(
		&'a self,
		params: &'a Option<PaginationParams>,
		sort_by: &'a Option<ListExecutionsSorting>,
	) -> impl Stream<Item = Result<V2betaExecution>> + Send + Sync + use<'a> {
		futures::stream::try_unfold((0, VecDeque::new()), async |(mut page, mut buffered)| {
			if let Some(x) = buffered.pop_front() {
				return Ok(Some((x, (page, buffered))));
			}

			let url = self.make_url("v2beta/actions/executions/_search")?;
			let params = params.as_ref().unwrap_or(&PaginationParams::DEFAULT);
			let sort_by = format!(
				"{:?}",
				sort_by
					.as_ref()
					.unwrap_or(&ListExecutionsSorting::EXECUTION_FIELD_NAME_UNSPECIFIED)
			);
			let req = self
				.client
				.post(url.clone())
				.query(&[
					("pagination.limit", params.page_size.to_string()),
					("pagination.offset", (page * params.page_size).to_string()),
					("pagination.asc", params.asc.to_string()),
					("sortingColumn", sort_by),
				])
				.build()?;
			buffered = self
				.send_request::<V2betaListExecutionsResponse>(req)
				.await?
				.result
				.unwrap_or_default()
				.into();
			page += 1;

			let Some(x) = buffered.pop_front() else {
				return Ok(None);
			};
			Ok(Some((x, (page, buffered))))
		})
	}
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, missing_docs)]
pub enum ListExecutionsSorting {
	EXECUTION_FIELD_NAME_UNSPECIFIED,
	EXECUTION_FIELD_NAME_ID,
	EXECUTION_FIELD_NAME_CREATED_DATE,
	EXECUTION_FIELD_NAME_CHANGED_DATE,
}
