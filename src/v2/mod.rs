#![allow(clippy::missing_docs_in_private_items)]
//! Communication with Zitadel using http [v2 API](https://zitadel.com/docs/apis/v2)

/// Service user authentication
pub mod authentication;
pub mod events;
pub mod management;
pub mod organization;
pub mod pagination;
/// Helper client to authenticate tokens
pub mod token;
pub mod users;
use std::{path::PathBuf, sync::Arc};

use anyhow_ext::{bail, Context, Result};
use anyhow_trace::anyhow_trace;
use authentication::Token;
use headers::{Authorization, HeaderMapExt};
use reqwest::{Client, Request};
use serde::de::DeserializeOwned;
use tokio::sync::RwLock;
use url::Url;

/// Header for Zitadel organization ID
pub const HEADER_ZITADEL_ORGANIZATION_ID: &str = "x-zitadel-orgid";

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
	#[anyhow_trace]
	pub async fn new(url: Url, service_account_file: PathBuf) -> Result<Self> {
		let client = Client::new();
		let token = Token::new(&url, &service_account_file, client.clone(), None).await?;

		Ok(Self { token: Arc::new(RwLock::new(token)), domain: url, client })
	}

	/// Send the request to zitadel server and returns the body of the request
	/// in case of success
	#[anyhow_trace]
	#[allow(clippy::needless_question_mark)] // anyhow_trace triggers this lint
	async fn send_request<T: DeserializeOwned>(&self, mut request: Request) -> Result<T> {
		if self.token.read().await.is_expired() {
			self.token.write().await.renew().await.dot()?;
		}

		let headers = request.headers_mut();
		HeaderMapExt::typed_insert(
			headers,
			Authorization::bearer(&self.token.read().await.token).dot()?,
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
	#[anyhow_trace]
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

#[cfg(test)]
mod tests {

	use std::path::Path;

	use anyhow_ext::Result;
	use time::OffsetDateTime;
	use wiremock::{
		matchers::{self, method, path},
		Mock, MockServer, ResponseTemplate,
	};

	use super::*;

	const SERVICE_USER_KEY_PATH: &str = "tests/environment/zitadel/service-user.json";

	#[tokio::test]
	async fn test_renew_token() -> Result<()> {
		let mock_server = MockServer::start().await;

		let auth_resp = authentication::AuthResponse {
			access_token: "my_token".to_owned(),
			token_type: "token_type".to_owned(),
			expires_in: -1,
		};

		Mock::given(method("POST"))
			.and(path("/oauth/v2/token"))
			.respond_with(ResponseTemplate::new(200).set_body_json(auth_resp))
			.expect(2)
			.mount(&mock_server)
			.await;

		Mock::given(method("POST"))
			.and(matchers::path_regex(r"/v2/users/.*"))
			.respond_with(ResponseTemplate::new(200))
			.mount(&mock_server)
			.await;

		let service_account_file = Path::new(SERVICE_USER_KEY_PATH);
		let url = Url::parse(&mock_server.uri())?;
		let zitadel = Zitadel::new(url, service_account_file.to_path_buf()).await?;

		zitadel.token.write().await.expiry =
			OffsetDateTime::now_utc() - time::Duration::minutes(10);

		let _ = zitadel.get_user_by_id("user_id").await;

		assert!(zitadel.token.read().await.expiry > OffsetDateTime::now_utc());

		Ok(())
	}
}
