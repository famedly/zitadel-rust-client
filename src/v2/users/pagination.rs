use std::{future::Future, pin::Pin, task::Poll};

use anyhow::{Context, Result};
use futures::{FutureExt, Stream};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use crate::v2::Zitadel;

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
	fn to_paginated_request(&self, page: usize, page_size: usize) -> Self::Item;
}

type DataFuture<T> = dyn Future<Output = Result<Vec<T>>> + Send;

pub(crate) struct PaginationHandler<Q, T>
where
	Q: Serialize + Send,
	T: DeserializeOwned + 'static,
{
	zitadel: Zitadel,
	query: Box<dyn PaginationRequest<Item = Q> + Send>,
	endpoint: Url,
	page: usize,
	page_size: usize,
	buffer: Vec<T>,
	done: bool,
	data_fut: Pin<Box<DataFuture<T>>>,
}

impl<Q, T> PaginationHandler<Q, T>
where
	Q: Serialize + Send + 'static,
	T: DeserializeOwned + 'static,
{
	pub(crate) fn new(
		zitadel: Zitadel,
		page_size: usize,
		query: impl PaginationRequest<Item = Q> + Send + 'static,
		endpoint: Url,
	) -> Self {
		let page = 0;
		let req_query = query.to_paginated_request(page, page_size);
		Self {
			zitadel: zitadel.clone(),
			query: Box::new(query),
			endpoint: endpoint.clone(),
			page,
			page_size,
			buffer: Vec::new(),
			done: false,
			data_fut: Box::pin(get_more_data::<T>(zitadel, req_query, endpoint)),
		}
	}
}

async fn get_more_data<T: DeserializeOwned + 'static>(
	mut zitadel: Zitadel,
	query: impl Serialize + Send,
	endpoint: Url,
) -> Result<Vec<T>> {
	let request = zitadel
		.client
		.post(endpoint)
		.json(&query)
		.build()
		.context("Error building request for pagination")?;
	let mut resp = zitadel.send_request::<PaginatedResponse<T>>(request).await?;

	Ok(resp.take_result().unwrap_or(Vec::new()))
}

impl<Q, T> Unpin for PaginationHandler<Q, T>
where
	Q: Serialize + Send + 'static,
	T: DeserializeOwned + 'static,
{
}

impl<Q, T> Stream for PaginationHandler<Q, T>
where
	Q: Serialize + Send + 'static,
	T: DeserializeOwned + 'static,
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
						self.done = self.buffer.len() < self.page_size;
						self.data_fut = Box::pin(get_more_data::<T>(
							self.zitadel.clone(),
							self.query.to_paginated_request(self.page, self.page_size),
							self.endpoint.clone(),
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
