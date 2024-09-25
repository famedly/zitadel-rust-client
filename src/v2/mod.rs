#![allow(clippy::missing_docs_in_private_items)]
//! Communication with Zitadel using http [v2 API](https://zitadel.com/docs/apis/v2)

mod authentication;
pub mod users;
use std::{path::PathBuf, sync::Arc};

use anyhow::{bail, Context, Result};
use authentication::Token;
use reqwest::{Client, Request};
use serde::de::DeserializeOwned;
use tokio::sync::RwLock;
use url::Url;

/// Default timeout value to be used in various places
const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

/// Zitadel client for using the HTTP v2 api
#[derive(Debug, Clone)]
pub struct Zitadel {
	/// Token for performing the requests as a service account
	token: Arc<RwLock<Token>>,
	/// Zitadel domain
	domain: Url,
	/// Client for performing the requests
	client: Client,
}

impl Zitadel {
	/// Builds a new Zitadel instance.
	/// - `url` should point to the Zitadel instance the client is for
	/// - `service_account_file` should be the Zitadel-generated
	///   private key file as documented at [zitadel docs](https://zitadel.com/docs/guides/integrate/service-users/private-key-jwt#2-generate-a-private-key-file)
	pub async fn new(url: Url, service_account_file: PathBuf) -> Result<Self> {
		let client = Client::new();
		let token = Token::new(&url, &service_account_file, client.clone()).await?;

		Ok(Self { token: Arc::new(RwLock::new(token)), domain: url, client })
	}

	/// Send the request to zitadel server and returns the body of the request
	/// in case of success
	async fn send_request<T: DeserializeOwned>(&mut self, mut request: Request) -> Result<T> {
		if self.token.read().await.is_expired() {
			self.token.write().await.renew().await?;
		}

		let headers = request.headers_mut();
		headers
			.append("Authorization", format!("Bearer {}", self.token.read().await.token).parse()?);
		headers.append("Content-Type", "application/json".parse()?);
		headers.append("Accept", "application/json".parse()?);

		let _ = request.timeout_mut().insert(DEFAULT_TIMEOUT);

		tracing::trace!("Request: {}", format_request_for_log(&request));

		let response = self.client.execute(request).await?;
		let status_code = response.status();
		let body = response
			.text()
			.await
			.context(format!("Error retrieving the body. Response status code: {status_code}"))?;

		if !status_code.is_success() {
			bail!("The request resulted in error. Response: {status_code}. Body: {body}")
		}

		Ok(serde_json::from_str::<T>(&body)?)
	}

	/// Crates the full url using the provided endpoint path
	fn make_url(&self, endpoint: &str) -> Result<Url> {
		self.domain.join(endpoint).context("Invalid relative path for the url")
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
