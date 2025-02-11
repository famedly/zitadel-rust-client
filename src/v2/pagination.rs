//! Module to transparently handle response pagination
use std::{future::Future, pin::Pin, task::Poll};

use anyhow::{Context, Result};
use famedly_rust_utils::GenericCombinators;
use futures::{FutureExt, Stream};
use reqwest::header::HeaderValue;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use super::HEADER_ZITADEL_ORGANIZATION_ID;
use crate::v2::Zitadel;

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
	type Item: Serialize + 'static;
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
	buffer: Vec<T>,
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
			buffer: Vec::new(),
			done: false,
			data_fut: Box::pin(get_more_data::<T>(zitadel, req_query, endpoint, org_id.clone())),
			org_id,
		}
	}
}

async fn get_more_data<T: DeserializeOwned + 'static>(
	zitadel: Zitadel,
	query: impl Serialize + Send + Sync,
	endpoint: Url,
	org_id: Option<String>,
) -> Result<Vec<T>> {
	let org_id = org_id.as_deref().map(HeaderValue::from_str).transpose()?;
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
	type Item = T;
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
						self.buffer = data;

						// We need to reverse to get te right order since we pop
						self.buffer.reverse();

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
						tracing::error!("Error getting a new page. Err: {e}");
						return Poll::Ready(None);
					}
				},
			}
		}
		Poll::Ready(self.buffer.pop())
	}
}
