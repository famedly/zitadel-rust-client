//! Handles the authentication of the service account on zitadel and the renew
//! of the token
use std::{fmt::Debug, path::PathBuf};

use anyhow_ext::{bail, Context, Result};
use anyhow_trace::anyhow_trace;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio::sync::RwLock;
use url::Url;

const MIN_SCOPE: [&str; 2] = ["openid", "urn:zitadel:iam:org:project:id:zitadel:aud"];

#[derive(Deserialize, Serialize)]
struct ServiceAccount {
	#[allow(dead_code)]
	r#type: String,
	#[serde(rename = "keyId")]
	key_id: String,
	key: String,
	#[serde(rename = "userId")]
	user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
	iss: String,
	sub: String,
	aud: String,
	iat: i64,
	exp: i64,
}

#[derive(Deserialize, Serialize)]
pub(super) struct AuthResponse {
	pub access_token: String,
	#[allow(dead_code)]
	pub token_type: String,
	#[allow(dead_code)]
	pub expires_in: i64,
}

/// Token for the provided service user account.
/// This already takes care of renewing the token
#[derive(Debug)]
pub struct Token {
	inner: RwLock<InnerToken>,
}

/// Token and information to renew it
struct InnerToken {
	/// Token for authenticating the requests
	pub token: String,
	/// Time when the token will expiry
	pub expiry: OffsetDateTime,

	header: Header,
	claims: Claims,
	key: EncodingKey,
	client: reqwest::Client,
	scope: Option<Vec<String>>,
	url: Url,
}

#[anyhow_trace]
impl Token {
	/// Create a new token
	///
	/// Creates a new token for the provided service account.
	/// The minimal scope of this token is 'openid' and
	/// 'urn:zitadel:iam:org:project:id:zitadel:aud'.
	/// For more scoped check the [zitadel doc](https://zitadel.com/docs/apis/openidoauth/scopes)
	///
	/// # Arguments
	///
	/// * `url` - Url for Zitadel server
	/// * `service_account_file` - Path to the service account json file
	/// * `client` - Reqwest client to be used
	/// * `scope` - Additional scopes for the requested token
	/// * `aud` - Cursom aud claim (`url` is used if unset)
	pub async fn new(
		url: Url,
		service_account_file: &PathBuf,
		client: reqwest::Client,
		scope: Option<Vec<String>>,
		aud: Option<String>,
	) -> Result<Self> {
		let service_account: ServiceAccount =
			serde_json::from_str(tokio::fs::read_to_string(service_account_file).await?.as_ref())?;

		let mut header = Header::new(Algorithm::RS256);
		header.kid = Some(service_account.key_id.clone());

		// The renew will fix it
		let claims = Claims {
			iss: service_account.user_id.clone(),
			sub: service_account.user_id.clone(),
			aud: aud.unwrap_or_else(|| url.as_str().trim_end_matches('/').to_owned()),
			iat: 0,
			exp: 0,
		};

		let key = EncodingKey::from_rsa_pem(service_account.key.as_bytes())?;

		let mut inner = InnerToken {
			token: "".to_owned(),
			expiry: OffsetDateTime::now_utc(),
			header,
			claims,
			key,
			client,
			scope,
			url,
		};

		inner.renew().await.context("Error getting the token for the first time")?;

		Ok(Self { inner: RwLock::new(inner) })
	}

	/// Get a valid token. If the token is expired a new token will be generated
	/// and returned
	pub async fn token(&self) -> Result<String> {
		if self.inner.read().await.is_expired() {
			self.inner.write().await.renew().await.dot()?;
		}

		Ok(self.inner.read().await.token.clone())
	}
}

impl InnerToken {
	/// Renew the token
	pub async fn renew(&mut self) -> Result<()> {
		self.claims.iat = OffsetDateTime::now_utc().unix_timestamp();
		self.expiry = OffsetDateTime::now_utc() + time::Duration::minutes(59);
		self.claims.exp = self.expiry.unix_timestamp();

		let jwt = encode(&self.header, &self.claims, &self.key)?;
		let mut scope = self.scope.clone().unwrap_or_default();
		scope.append(&mut MIN_SCOPE.iter().copied().map(String::from).collect());

		let resp = self
			.client
			.post(self.url.clone().join("oauth/v2/token")?)
			.header("Content-Type", "application/x-www-form-urlencoded")
			.form(&[
				("grant_type".to_owned(), "urn:ietf:params:oauth:grant-type:jwt-bearer".to_owned()),
				("scope".to_owned(), scope.join(" ")),
				("assertion".to_owned(), jwt),
			])
			.send()
			.await
			.context("Error renewing the token")?;

		let status = resp.status();
		let body = resp.text().await?;
		if !status.is_success() {
			bail!(
				"Request for renewing the token was not successful. Status code: {}. Body: {body}",
				status.as_str(),
			)
		}

		let auth_resp: AuthResponse = serde_json::from_str(&body)?;

		self.token = auth_resp.access_token;

		Ok(())
	}

	#[must_use]
	/// Check if the token is expired
	pub fn is_expired(&self) -> bool {
		OffsetDateTime::now_utc() > self.expiry
	}
}

impl Debug for InnerToken {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{{expiry:{:?}, header:{:?}, claims:{:?}}}",
			self.expiry, self.header, self.claims
		)
	}
}

#[cfg(test)]
mod tests {

	use anyhow::Result;
	use tempfile::tempdir;
	use time::OffsetDateTime;
	use url::Url;
	use wiremock::{
		matchers::{method, path},
		Mock, MockServer, ResponseTemplate,
	};

	use crate::v2::authentication::{self, ServiceAccount, Token};

	#[tokio::test]
	async fn test_token() -> Result<()> {
		let mock_server = MockServer::start().await;
		let temp_dir = tempdir()?;
		let service_account_file = temp_dir.path().join("service-user.json");
		let key = josekit::jwk::alg::rsa::RsaKeyPair::generate(2048)?;
		tokio::fs::write(
			&service_account_file,
			serde_json::to_string(&ServiceAccount {
				r#type: "".to_owned(),
				key_id: "".to_owned(),
				key: String::from_utf8(key.to_pem_private_key())?,
				user_id: "".to_owned(),
			})?,
		)
		.await?;

		let zitadel_token = "my_token".to_owned();

		let auth_resp = authentication::AuthResponse {
			access_token: zitadel_token.clone(),
			token_type: "token_type".to_owned(),
			expires_in: -1,
		};

		Mock::given(method("POST"))
			.and(path("/oauth/v2/token"))
			.respond_with(ResponseTemplate::new(200).set_body_json(auth_resp))
			.expect(2)
			.mount(&mock_server)
			.await;

		let url = Url::parse(&mock_server.uri())?;
		let token =
			Token::new(url, &service_account_file, reqwest::Client::new(), None, None).await?;

		assert_eq!(token.token().await?, zitadel_token);
		// A second call shouldn't trigger a renew
		assert_eq!(token.token().await?, zitadel_token);

		token.inner.write().await.expiry = OffsetDateTime::now_utc() - time::Duration::minutes(10);

		// Now we should have a call to renew
		assert_eq!(token.token().await?, zitadel_token);

		assert!(token.inner.read().await.expiry > OffsetDateTime::now_utc());

		Ok(())
	}
}
