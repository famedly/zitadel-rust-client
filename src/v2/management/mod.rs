//! Implementation of the client for the Zitadel Management api
mod models;

use anyhow::{Context, Result};
use delegate::delegate;
use famedly_rust_utils::GenericCombinators;
use futures::Stream;
pub use models::*;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

use super::{
	pagination::{PaginationHandler, PaginationRequest},
	Zitadel,
};

/// Metadata/Header for Zitadel organization ID, used to set/get metadata for
/// organizations.
pub const HEADER_ZITADEL_ORGANIZATION_ID: &str = "x-zitadel-orgid";

impl Zitadel {
	/// Create actions. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-create-action)
	pub async fn create_action(
		&self,
		organization_id: Option<String>,
		body: V1CreateActionRequest,
	) -> Result<V1CreateActionResponse> {
		let request = self
			.client
			.post(self.make_url("management/v1/actions")?)
			.header(
				HEADER_ZITADEL_ORGANIZATION_ID,
				HeaderValue::from_str(&organization_id.unwrap_or_default())?,
			)
			.json(&body)
			.build()
			.context("Error building create_action request")?;

		self.send_request(request).await
	}

	/// Update action. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-update-action)
	pub async fn update_action(
		&self,
		organization_id: Option<String>,
		action_id: String,
		body: ManagementServiceUpdateActionBody,
	) -> Result<V1UpdateActionResponse> {
		let request = self
			.client
			.put(self.make_url(&format!("management/v1/actions/{action_id}"))?)
			.header(
				HEADER_ZITADEL_ORGANIZATION_ID,
				HeaderValue::from_str(&organization_id.unwrap_or_default())?,
			)
			.json(&body)
			.build()
			.context("Error building update_action request")?;

		self.send_request(request).await
	}

	/// Delete action. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-delete-action)
	pub async fn delete_action(
		&self,
		organization_id: Option<String>,
		action_id: String,
	) -> Result<V1DeleteActionResponse> {
		let request = self
			.client
			.delete(self.make_url(&format!("management/v1/actions/{action_id}"))?)
			.header(
				HEADER_ZITADEL_ORGANIZATION_ID,
				HeaderValue::from_str(&organization_id.unwrap_or_default())?,
			)
			.json(&ManagementServiceDeleteActionBody::new())
			.build()
			.context("Error building delete_action request")?;

		self.send_request(request).await
	}

	/// Search for actions. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-actions)
	pub fn search_actions(
		&self,
		body: ListActionsRequest,
	) -> Result<impl Stream<Item = V1Action> + Send + Sync> {
		// TODO: Make it possible to use HEADER_ZITADEL_ORGANIZATION_ID
		Ok(PaginationHandler::<_, V1Action>::new(
			self.clone(),
			body,
			self.make_url("management/v1/actions/_search")?,
		))
	}

	/// Search for actions. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-actions)
	pub async fn list_actions_without_pagination(
		&self,
		org_id: Option<String>,
		body: ListActionsRequest,
	) -> Result<SearchWithoutPaginationResponse<V1Action>> {
		let request = self
			.client
			.post(self.make_url("management/v1/actions/_search")?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(&body.inner_request)
			.build()
			.context("Error building list_actions_without_pagination request")?;

		self.send_request(request).await
	}

	/// Get a flow. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-get-flow)
	pub async fn get_flow(
		&self,
		organization_id: Option<String>,
		flow_type: u32,
	) -> Result<V1GetFlowResponse> {
		let request = self
			.client
			.get(self.make_url(&format!("management/v1/flows/{flow_type}"))?)
			.header(
				HEADER_ZITADEL_ORGANIZATION_ID,
				HeaderValue::from_str(&organization_id.unwrap_or_default())?,
			)
			.build()
			.context("Error building get_flow request")?;

		self.send_request(request).await
	}

	/// Set trigger actions. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-set-trigger-actions)
	pub async fn set_trigger_actions(
		&self,
		organization_id: Option<String>,
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
			.header(
				HEADER_ZITADEL_ORGANIZATION_ID,
				HeaderValue::from_str(&organization_id.unwrap_or_default())?,
			)
			.json(&body)
			.build()
			.context("Error building set_trigger_actions request")?;

		self.send_request(request).await
	}

	/// Create application. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-api-app)
	pub async fn create_application(
		&self,
		organization_id: Option<String>,
		project_id: String,
		body: ManagementServiceAddApiAppBody,
	) -> Result<V1AddApiAppResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("management/v1/projects/{project_id}/apps/api"))?)
			.header(
				HEADER_ZITADEL_ORGANIZATION_ID,
				HeaderValue::from_str(&organization_id.unwrap_or_default())?,
			)
			.json(&body)
			.build()
			.context("Error building create_application request")?;

		self.send_request(request).await
	}

	/// Remove application. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-remove-app)
	pub async fn remove_application(
		&self,
		organization_id: Option<String>,
		project_id: String,
		application_id: String,
	) -> Result<V1RemoveAppResponse> {
		let request =
			self.client
				.delete(self.make_url(&format!(
					"management/v1/projects/{project_id}/apps/{application_id}"
				))?)
				.header(
					HEADER_ZITADEL_ORGANIZATION_ID,
					HeaderValue::from_str(&organization_id.unwrap_or_default())?,
				)
				.build()
				.context("Error building remove_application request")?;

		self.send_request(request).await
	}

	/// Search for applications [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-apps)
	pub fn list_applications(
		&self,
		project_id: String,
		body: ListApplicationsRequest,
	) -> Result<impl Stream<Item = V1App>> {
		// TODO: Make it possible to use HEADER_ZITADEL_ORGANIZATION_ID
		Ok(PaginationHandler::<_, V1App>::new(
			self.clone(),
			body,
			self.make_url(&format!("management/v1/projects/{project_id}/apps/_search"))?,
		))
	}

	/// Search for applications [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-apps)
	pub async fn list_applications_without_pagination(
		&self,
		org_id: Option<String>,
		project_id: String,
		body: ListApplicationsRequest,
	) -> Result<SearchWithoutPaginationResponse<V1App>> {
		let request = self
			.client
			.post(self.make_url(&format!("management/v1/projects/{project_id}/apps/_search"))?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(&body.inner_request)
			.build()
			.context("Error building list_applications_without_pagination request")?;

		self.send_request(request).await
	}

	/// Create project. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-project)
	pub async fn create_project(
		&self,
		organization_id: Option<String>,
		body: V1AddProjectRequest,
	) -> Result<V1AddProjectResponse> {
		let request = self
			.client
			.post(self.make_url("management/v1/projects")?)
			.header(
				HEADER_ZITADEL_ORGANIZATION_ID,
				HeaderValue::from_str(&organization_id.unwrap_or_default())?,
			)
			.json(&body)
			.build()
			.context("Error building create_project request")?;

		self.send_request(request).await
	}

	/// Remove project. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-remove-project)
	pub async fn remove_project(
		&self,
		organization_id: Option<String>,
		project_id: String,
	) -> Result<V1RemoveProjectResponse> {
		let request = self
			.client
			.delete(self.make_url(&format!("management/v1/projects/{project_id}"))?)
			.header(
				HEADER_ZITADEL_ORGANIZATION_ID,
				HeaderValue::from_str(&organization_id.unwrap_or_default())?,
			)
			.build()
			.context("Error building delete_project request")?;

		self.send_request(request).await
	}

	/// Search for projects [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-projects)
	pub fn list_projects(
		&self,
		body: ListProjectsRequest,
	) -> Result<impl Stream<Item = V1Project>> {
		// TODO: Make it possible to use HEADER_ZITADEL_ORGANIZATION_ID
		Ok(PaginationHandler::<_, V1Project>::new(
			self.clone(),
			body,
			self.make_url("management/v1/projects/_search")?,
		))
	}

	/// Search for projects [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-projects)
	pub async fn list_projects_without_pagination(
		&self,
		org_id: Option<String>,
		body: ListProjectsRequest,
	) -> Result<SearchWithoutPaginationResponse<V1Project>> {
		let request = self
			.client
			.post(self.make_url("management/v1/projects/_search")?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(&body.inner_request)
			.build()
			.context("Error building list_projects_without_pagination request")?;

		self.send_request(request).await
	}
}

/// Pagination-supporting project search
#[derive(Clone, Debug, Serialize)]
pub struct ListProjectsRequest {
	inner_request: V1ListProjectsRequest,
}

impl ListProjectsRequest {
	/// Constructor
	#[must_use]
	pub fn new(queries: Vec<V1ProjectQuery>) -> Self {
		Self { inner_request: V1ListProjectsRequest::new().with_queries(queries) }
	}

	/// Use the supplied ListQuery
	#[must_use]
	pub fn with_query(mut self, query: V1ListQuery) -> Self {
		self.inner_request.set_query(query);
		self
	}

	/// Use the supplied application queries
	#[must_use]
	pub fn with_queries(mut self, queries: Vec<V1ProjectQuery>) -> Self {
		self.inner_request.set_queries(queries);
		self
	}

	delegate! {
		to self.inner_request {
			/// Set the supplied ListQuery
			pub fn set_query(&mut self, query: V1ListQuery);
			/// The ListQuery currently used for this request
			#[must_use] pub fn query(&self) -> Option<&V1ListQuery>;
			/// Reset the ListQuery to None
			pub fn reset_query(&mut self);
			/// Set the supplied app queries
			pub fn set_queries(&mut self, queries: Vec<V1ProjectQuery>);
			/// The app queries currently used for this request
			#[must_use] pub fn queries(&self) -> Option<&Vec<V1ProjectQuery>>;
			/// Reset the app queries to None
			pub fn reset_queries(&mut self);
		}
	}
}

/// Response for search without pagination
#[derive(Clone, Debug, Deserialize)]
pub struct SearchWithoutPaginationResponse<T> {
	/// The result of the search
	pub result: Option<Vec<T>>,
	/// The details of the search
	pub details: Option<V1ListDetails>,
}

impl PaginationRequest for ListProjectsRequest {
	type Item = V1ListProjectsRequest;

	fn to_paginated_request(&self, page: usize) -> Self::Item {
		self.inner_request.clone().with_query(
			self.inner_request
				.query()
				.unwrap_or(&V1ListQuery::new())
				.clone()
				.with_offset((page * self.page_size()).to_string()),
		)
	}

	fn page_size(&self) -> usize {
		(*self.inner_request.query().and_then(|query| query.limit()).unwrap_or(&1000))
			.try_into()
			.unwrap_or(1000)
	}
}

/// Pagination-supporting application search
#[derive(Clone, Debug, Serialize)]
pub struct ListApplicationsRequest {
	inner_request: ManagementServiceListAppsBody,
}

impl ListApplicationsRequest {
	/// Constructor
	#[must_use]
	pub fn new(queries: Vec<V1AppQuery>) -> Self {
		Self { inner_request: ManagementServiceListAppsBody::new().with_queries(queries) }
	}

	/// Use the supplied ListQuery
	#[must_use]
	pub fn with_query(mut self, query: V1ListQuery) -> Self {
		self.inner_request.set_query(query);
		self
	}

	/// Use the supplied application queries
	#[must_use]
	pub fn with_queries(mut self, queries: Vec<V1AppQuery>) -> Self {
		self.inner_request.set_queries(queries);
		self
	}

	delegate! {
		to self.inner_request {
			/// Set the supplied ListQuery
			pub fn set_query(&mut self, query: V1ListQuery);
			/// The ListQuery currently used for this request
			#[must_use] pub fn query(&self) -> Option<&V1ListQuery>;
			/// Reset the ListQuery to None
			pub fn reset_query(&mut self);
			/// Set the supplied app queries
			pub fn set_queries(&mut self, queries: Vec<V1AppQuery>);
			/// The app queries currently used for this request
			#[must_use] pub fn queries(&self) -> Option<&Vec<V1AppQuery>>;
			/// Reset the app queries to None
			pub fn reset_queries(&mut self);
		}
	}
}

impl PaginationRequest for ListApplicationsRequest {
	type Item = ManagementServiceListAppsBody;

	fn to_paginated_request(&self, page: usize) -> Self::Item {
		self.inner_request.clone().with_query(
			self.inner_request
				.query()
				.unwrap_or(&V1ListQuery::new())
				.clone()
				.with_offset((page * self.page_size()).to_string()),
		)
	}

	fn page_size(&self) -> usize {
		(*self.inner_request.query().and_then(|query| query.limit()).unwrap_or(&1000))
			.try_into()
			.unwrap_or(1000)
	}
}

/// Pagination-supporting action search
#[derive(Clone, Debug, Serialize)]
pub struct ListActionsRequest {
	inner_request: V1ListActionsRequest,
}

impl ListActionsRequest {
	/// Constructor
	#[must_use]
	pub fn new(queries: Vec<V1ActionQuery>) -> Self {
		ListActionsRequest { inner_request: V1ListActionsRequest::new().with_queries(queries) }
	}

	/// Use the supplied ListQuery
	#[must_use]
	pub fn with_query(mut self, query: V1ListQuery) -> Self {
		self.inner_request.set_query(query);
		self
	}

	/// Use the supplied action queries
	#[must_use]
	pub fn with_queries(mut self, queries: Vec<V1ActionQuery>) -> Self {
		self.inner_request.set_queries(queries);
		self
	}

	delegate! {
		to self.inner_request {
			/// Set the supplied ListQuery
			pub fn set_query(&mut self, query: V1ListQuery);
			/// The ListQuery currently used for this request
			#[must_use] pub fn query(&self) -> Option<&V1ListQuery>;
			/// Reset the ListQuery to None
			pub fn reset_query(&mut self);
			/// Set the supplied action queries
			pub fn set_queries(&mut self, queries: Vec<V1ActionQuery>);
			/// The action queries currently used for this request
			#[must_use] pub fn queries(&self) -> Option<&Vec<V1ActionQuery>>;
			/// Reset the action queries to None
			pub fn reset_queries(&mut self);
		}
	}
}

impl PaginationRequest for ListActionsRequest {
	type Item = V1ListActionsRequest;

	fn to_paginated_request(&self, page: usize) -> Self::Item {
		self.inner_request.clone().with_query(
			self.inner_request
				.query()
				.unwrap_or(&V1ListQuery::new())
				.clone()
				.with_offset((page * self.page_size()).to_string()),
		)
	}

	fn page_size(&self) -> usize {
		(*self.inner_request.query().and_then(|query| query.limit()).unwrap_or(&1000))
			.try_into()
			// Realistically, page sizes should never be large enough
			// to hit the platform MAX_INT, but hey, I guess we can still avoid crashing.
			.unwrap_or(1000)
	}
}
