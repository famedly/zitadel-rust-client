#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::needless_question_mark)]
// need it for anyhow_trace messages on a last expression in functions

// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

//! Communication with Zitadel using http [v2 API](https://zitadel.com/docs/apis/v2)

pub mod actions;
/// Service user authentication
pub mod authentication;
pub mod events;
pub mod management;
pub mod organization;
pub mod pagination;
pub mod project;
/// Helper client to authenticate tokens
pub mod token;
pub mod users;
use std::{path::PathBuf, sync::Arc};

use anyhow_ext::{Context, Result, bail};
use anyhow_trace::anyhow_trace;
use authentication::Token;
use headers::{Authorization, HeaderMapExt};
use reqwest::{Client, Request};
use serde::de::DeserializeOwned;
use url::Url;

/// Header for Zitadel organization ID
pub const HEADER_ZITADEL_ORGANIZATION_ID: &str = "x-zitadel-orgid";

/// Default timeout value to be used in various places
const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

/// Zitadel client for using the HTTP v2 api
#[derive(Debug, Clone)]
pub struct Zitadel {
	/// Token for performing the requests as a service account
	token: Arc<Token>,
	/// Zitadel domain
	domain: Url,
	/// Client for performing the requests
	client: Client,
}

#[anyhow_trace]
impl Zitadel {
	/// Builds a new Zitadel instance.
	/// - `url` should point to the Zitadel instance the client is for
	/// - `service_account_file` should be the Zitadel-generated
	///   private key file as documented at [zitadel docs](https://zitadel.com/docs/guides/integrate/service-users/private-key-jwt#2-generate-a-private-key-file)
	/// - `aud` - Custom `aud` claim to use for auth (`url` is used if unset)
	pub async fn new(url: Url, service_account_file: PathBuf, aud: Option<String>) -> Result<Self> {
		let client = Client::new();
		let token =
			Token::new(url.clone(), &service_account_file, client.clone(), None, aud).await?;

		Ok(Self { token: Arc::new(token), domain: url, client })
	}

	/// Send the request to zitadel server and returns the body of the request
	/// in case of success
	async fn send_request<T: DeserializeOwned>(&self, mut request: Request) -> Result<T> {
		let headers = request.headers_mut();
		HeaderMapExt::typed_insert(
			headers,
			Authorization::bearer(&self.token.token().await?).dot()?,
		);
		headers.append("Content-Type", "application/json".parse()?);
		headers.append("Accept", "application/json".parse()?);

		let _ = request.timeout_mut().insert(DEFAULT_TIMEOUT);

		tracing::trace!("Request: {}", format_request_for_log(&request));

		let response = self.client.execute(request).await?;
		let status_code = response.status();
		let body = response.text().await.with_context(|| {
			format!("Error retrieving the body. Response status code: {status_code}")
		})?;

		if !status_code.is_success() {
			bail!("The request resulted in error. Response: {status_code}. Body: {body}")
		}

		tracing::trace!("Response: {}", body);

		Ok(serde_json::from_str::<T>(&body)?)
	}

	/// Crates the full url using the provided endpoint path
	fn make_url(&self, endpoint: &str) -> Result<Url> {
		self.domain.join(endpoint).context("Invalid relative path for the url")
	}

	/// Gets the Zitadel domain used by the client
	#[must_use]
	pub fn domain(&self) -> &Url {
		&self.domain
	}
}

/// Format a request for logging purposes; this *must* omit any
/// secrets or otherwise sensitive information
fn format_request_for_log(request: &Request) -> String {
	format!(
		"{} {}, headers: {:?}",
		request.method(),
		request.url(),
		request
			.headers()
			.into_iter()
			.filter(|(name, _)| name.as_str().to_lowercase() != "authorization")
			.collect::<Vec<_>>()
	)
}
