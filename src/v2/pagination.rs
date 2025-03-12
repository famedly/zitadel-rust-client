//! Module to transparently handle response pagination
use std::{collections::VecDeque, future::Future, pin::Pin, task::Poll};

use anyhow_ext::{Context, Result};
use anyhow_trace::anyhow_trace;
use famedly_rust_utils::GenericCombinators;
use futures::{FutureExt, Stream};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use super::HEADER_ZITADEL_ORGANIZATION_ID;
use crate::v2::{users::ListQuery, Zitadel};

/// A paginated response
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PaginatedResponse<T> {
	#[serde(rename = "result")]
	result: Option<Vec<T>>,
}

impl<T> PaginatedResponse<T>
where
	T: DeserializeOwned + 'static,
{
	fn take_result(&mut self) -> Option<Vec<T>> {
		self.result.take()
	}
}

pub(crate) trait PaginationRequest {
	type Item: Serialize;
	fn to_paginated_request(&self, page: usize) -> Self::Item;
	fn page_size(&self) -> usize;
}

type DataFuture<T> = dyn Future<Output = Result<Vec<T>>> + Send + Sync;

/// A handler for paginated requests.
///
/// Fetches the data from the Zitadel API and
/// returns a stream of the data.
///
/// **Some Zitadel endpoints allow to set an `org_id`. If not provided, the
/// organization of the caller is used.**
pub(crate) struct PaginationHandler<Q, T, R>
where
	Q: Serialize + Send + Sync,
	T: DeserializeOwned + 'static,
	R: PaginationRequest<Item = Q> + Send + Sync,
{
	zitadel: Zitadel,
	query: R,
	endpoint: Url,
	page: usize,
	buffer: VecDeque<T>,
	done: bool,
	data_fut: Pin<Box<DataFuture<T>>>,
	org_id: Option<String>,
}

impl<Q, T, R> PaginationHandler<Q, T, R>
where
	Q: Serialize + Send + Sync + 'static,
	T: DeserializeOwned + 'static,
	R: PaginationRequest<Item = Q> + Send + Sync,
{
	pub(crate) fn new(zitadel: Zitadel, query: R, endpoint: Url, org_id: Option<String>) -> Self {
		let page = 0;
		let req_query = query.to_paginated_request(page);
		Self {
			zitadel: zitadel.clone(),
			query,
			endpoint: endpoint.clone(),
			page,
			buffer: VecDeque::new(),
			done: false,
			data_fut: Box::pin(get_more_data::<T>(zitadel, req_query, endpoint, org_id.clone())),
			org_id,
		}
	}
}

#[anyhow_trace]
async fn get_more_data<T: DeserializeOwned + 'static>(
	zitadel: Zitadel,
	query: impl Serialize + Send + Sync,
	endpoint: Url,
	org_id: Option<String>,
) -> Result<Vec<T>> {
	let request = zitadel
		.client
		.post(endpoint)
		.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
		.json(&query)
		.build()
		.context("Error building request for pagination")?;
	let mut resp = zitadel.send_request::<PaginatedResponse<T>>(request).await?;

	Ok(resp.take_result().unwrap_or(Vec::new()))
}

impl<Q, T, R> Unpin for PaginationHandler<Q, T, R>
where
	Q: Serialize + Send + Sync + 'static,
	T: DeserializeOwned + 'static,
	R: PaginationRequest<Item = Q> + Send + Sync,
{
}

impl<Q, T, R> Stream for PaginationHandler<Q, T, R>
where
	Q: Serialize + Send + Sync + 'static,
	T: DeserializeOwned + 'static,
	R: PaginationRequest<Item = Q> + Send + Sync,
{
	type Item = Result<T>;
	fn poll_next(
		mut self: Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
	) -> Poll<Option<Self::Item>> {
		if self.buffer.is_empty() {
			if self.done {
				return Poll::Ready(None);
			}

			match self.data_fut.poll_unpin(cx) {
				Poll::Pending => return Poll::Pending,
				Poll::Ready(result) => match result {
					Ok(data) => {
						self.buffer = data.into();

						self.page += 1;
						self.done = self.buffer.len() < self.query.page_size();
						self.data_fut = Box::pin(get_more_data::<T>(
							self.zitadel.clone(),
							self.query.to_paginated_request(self.page),
							self.endpoint.clone(),
							self.org_id.clone(),
						));
					}
					Err(e) => {
						self.done = true;
						return Poll::Ready(Some(Err(e)));
					}
				},
			}
		}
		Poll::Ready(self.buffer.pop_front().map(Ok))
	}
}

/// Generic parameters for paginated requests
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct PaginationParams {
	pub page_size: usize,
	pub asc: bool,
}

#[allow(missing_docs)]
impl PaginationParams {
	#[must_use]
	pub fn with_page_size(mut self, x: usize) -> Self {
		self.page_size = x;
		self
	}

	#[must_use]
	pub fn with_asc(mut self, x: bool) -> Self {
		self.asc = x;
		self
	}
}

impl Default for PaginationParams {
	fn default() -> Self {
		Self::DEFAULT
	}
}

impl PaginationParams {
	#[allow(missing_docs)]
	pub const DEFAULT: Self = Self { page_size: 1000, asc: true };
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub(crate) struct NoSorting;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub(crate) struct NoQueries;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename = "camel_case")]
pub(crate) struct GenericListRequestBody<Query, Sorting> {
	query: Option<ListQuery>,
	#[serde(skip_serializing_if = "Option::is_none")]
	sorting_column: Option<Sorting>,
	#[serde(skip_serializing_if = "Option::is_none")]
	queries: Option<Vec<Query>>,
}

fn mk_list_query(page: usize, params: &PaginationParams) -> ListQuery {
	ListQuery::new()
		.with_limit(params.page_size)
		.with_offset((page * params.page_size).to_string())
		.with_asc(params.asc)
}

impl PaginationRequest for Option<PaginationParams> {
	type Item = GenericListRequestBody<NoQueries, NoSorting>;
	fn to_paginated_request(&self, page: usize) -> Self::Item {
		let params = self.as_ref().unwrap_or(&PaginationParams::DEFAULT);
		let page = mk_list_query(page, params);
		Self::Item { query: Some(page), sorting_column: None, queries: None }
	}

	fn page_size(&self) -> usize {
		self.as_ref().unwrap_or(&PaginationParams::DEFAULT).page_size
	}
}

impl<Q: Clone + Serialize> PaginationRequest for (Option<PaginationParams>, Option<Vec<Q>>) {
	type Item = GenericListRequestBody<Q, NoSorting>;
	fn to_paginated_request(&self, page: usize) -> Self::Item {
		let params = self.0.as_ref().unwrap_or(&PaginationParams::DEFAULT);
		let page = mk_list_query(page, params);
		Self::Item { query: Some(page), sorting_column: None, queries: self.1.clone() }
	}

	fn page_size(&self) -> usize {
		self.0.as_ref().unwrap_or(&PaginationParams::DEFAULT).page_size
	}
}

impl<Q: Clone + Serialize, S: Clone + Serialize> PaginationRequest
	for (Option<PaginationParams>, Option<S>, Option<Vec<Q>>)
{
	type Item = GenericListRequestBody<Q, S>;
	fn to_paginated_request(&self, page: usize) -> Self::Item {
		let params = self.0.as_ref().unwrap_or(&PaginationParams::DEFAULT);
		let page = mk_list_query(page, params);
		Self::Item { query: Some(page), sorting_column: self.1.clone(), queries: self.2.clone() }
	}

	fn page_size(&self) -> usize {
		self.0.as_ref().unwrap_or(&PaginationParams::DEFAULT).page_size
	}
}
