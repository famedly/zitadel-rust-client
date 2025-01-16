//! Implementation of the client for the zitadel organization's v2 api
mod models;

use anyhow::{Context, Result};
use delegate::delegate;
use futures::Stream;
pub use models::*;

use super::Zitadel;

impl Zitadel {
	/// Creates a new organization. [Docs](https://zitadel.com/docs/apis/resources/org_service_v2/organization-service-add-organization)
	pub async fn create_organization_with_admin(
		&self,
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

	/// Seach organizations. [Docs](https://zitadel.com/docs/apis/resources/org_service_v2/organization-service-list-organizations)
	pub async fn search_organizations(
		&self,
		body: SearchOrganizationRequest,
	) -> Result<impl Stream<Item = Zitadelorgv2Organization> + Send> {
		todo!()
	}
}

/// Pagination-supporting Organization search
#[derive(Clone, Debug)]
pub struct SearchOrganizationRequest {
	inner_request: V2ListOrganizationsRequest,
}

impl SearchOrganizationRequest {
	/// Constructor
	#[must_use]
	pub fn new(queries: Vec<Zitadelorgv2SearchQuery>) -> Self {
		Self { inner_request: V2ListOrganizationsRequest::new().with_queries(queries) }
	}

	/// Use the supplied Zitadelorgv2SearchQuery
	#[must_use]
	pub fn with_query(mut self, query: V2ListQuery) -> Self {
		self.inner_request.set_query(query);
		self
	}

	/// Use the supplied application queries
	#[must_use]
	pub fn with_queries(mut self, queries: Vec<Zitadelorgv2SearchQuery>) -> Self {
		self.inner_request.set_queries(queries);
		self
	}

	delegate! {
		to self.inner_request {
			/// Set the supplied ListQuery
			pub fn set_query(&mut self, query: V2ListQuery);
			/// The ListQuery currently used for this request
			#[must_use] pub fn query(&self) -> Option<&V2ListQuery>;
			/// Reset the ListQuery to None
			pub fn reset_query(&mut self);
			/// Set the supplied app queries
			pub fn set_queries(&mut self, queries: Vec<Zitadelorgv2SearchQuery>);
			/// The app queries currently used for this request
			#[must_use] pub fn queries(&self) -> Option<&Vec<Zitadelorgv2SearchQuery>>;
			/// Reset the app queries to None
			pub fn reset_queries(&mut self);
		}
	}
}
