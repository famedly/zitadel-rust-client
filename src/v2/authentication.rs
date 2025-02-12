//! Handles the authentication of the service account on zitadel and the renew
//! of the token
use std::{fmt::Debug, path::PathBuf};

use anyhow::{bail, Context, Result};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use url::Url;

const MIN_SCOPE: [&str; 2] = ["openid", "urn:zitadel:iam:org:project:id:zitadel:aud"];

#[derive(Deserialize)]
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

/// Token and information to renew it
pub struct Token {
	/// Token for authenticating the requests
	pub token: String,
	/// Time when the token will expiry
	pub expiry: OffsetDateTime,

	header: Header,
	claims: Claims,
	key: EncodingKey,
	client: reqwest::Client,
	scope: Option<Vec<String>>,
}

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
	pub async fn new(
		url: &Url,
		service_account_file: &PathBuf,
		client: reqwest::Client,
		scope: Option<Vec<String>>,
	) -> Result<Self> {
		let service_account: ServiceAccount =
			serde_json::from_str(std::fs::read_to_string(service_account_file)?.as_ref())?;

		let mut header = Header::new(Algorithm::RS256);
		header.kid = Some(service_account.key_id.clone());

		// The renew will fix it
		let claims = Claims {
			iss: service_account.user_id.clone(),
			sub: service_account.user_id.clone(),
			aud: url.as_str().trim_end_matches('/').to_owned(),
			iat: 0,
			exp: 0,
		};

		let key = EncodingKey::from_rsa_pem(service_account.key.as_bytes())?;

		let mut token = Self {
			token: "".to_owned(),
			expiry: OffsetDateTime::now_utc(),
			header,
			claims,
			key,
			client,
			scope,
		};

		token.renew().await.context("Error getting the token for the first time")?;

		Ok(token)
	}

	/// Renew the token
	pub async fn renew(&mut self) -> Result<()> {
		tracing::info!("Renewing the token");
		tracing::info!("Current expiry: {:?}", self.expiry);
		tracing::info!("Current iat: {:?}", self.claims.iat);
		tracing::info!("Current exp: {:?}", self.claims.exp);

		self.claims.iat = OffsetDateTime::now_utc().unix_timestamp();
		self.expiry = OffsetDateTime::now_utc() + time::Duration::minutes(1);
		self.claims.exp = self.expiry.unix_timestamp();

		tracing::info!("New expiry: {:?}", self.expiry);
		tracing::info!("New iat: {:?}", self.claims.iat);
		tracing::info!("New exp: {:?}", self.claims.exp);

		let jwt = encode(&self.header, &self.claims, &self.key)?;
		let mut scope = self.scope.clone().unwrap_or_default();
		scope.append(&mut MIN_SCOPE.iter().copied().map(String::from).collect());

		tracing::info!("Scope: {:?}", scope);
		tracing::info!("JWT: {:?}", jwt);
		tracing::info!("Header: {:?}", self.header);
		tracing::info!("Claims: {:?}", self.claims);
		tracing::info!("Client: {:?}", self.client);

		tracing::info!("Sending request to {}", format!("{}/oauth/v2/token", &self.claims.aud));

		let resp = self
			.client
			.post(format!("{}/oauth/v2/token", &self.claims.aud))
			.header("Content-Type", "application/x-www-form-urlencoded")
			.form(&[
				("grant_type".to_owned(), "urn:ietf:params:oauth:grant-type:jwt-bearer".to_owned()),
				("scope".to_owned(), scope.join(" ")),
				("assertion".to_owned(), jwt),
			])
			.send()
			.await
			.context("Error renewing the token")?;

		tracing::info!("Response from renewing the token: {:?}", resp);

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

	/// Check if the token is expired
	pub(crate) fn is_expired(&self) -> bool {
		OffsetDateTime::now_utc() > self.expiry
	}
}

impl Debug for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{{expiry:{:?}, header:{:?}, claims:{:?}}}",
			self.expiry, self.header, self.claims
		)
	}
}
