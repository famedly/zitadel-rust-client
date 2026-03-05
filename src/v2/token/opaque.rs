// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

//! Verifier for opaque tokens

use std::{path::PathBuf, sync::Arc};

use anyhow_ext::{Result, bail};
use josekit::jwt::JwtPayload;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use url::Url;

use crate::v2::private_key_jwt::{KeyType, PrivateKeyJWT};

/// Verifier for opaque tokens
#[derive(Debug, Clone)]
pub struct ZitadelOpaqueTokenVerifier {
	domain: Url,
	client: ClientWithMiddleware,
	private_key_jwt: Arc<RwLock<PrivateKeyJWT>>,
}

impl ZitadelOpaqueTokenVerifier {
	/// Create a new verifier
	pub async fn new(
		domain: Url,
		client: ClientWithMiddleware,
		app_secret_file: &PathBuf,
	) -> Result<Self> {
		let private_key_jwt =
			PrivateKeyJWT::new(domain.as_str().trim_end_matches('/').to_owned(), app_secret_file)
				.await?;

		if private_key_jwt.key_type != KeyType::App {
			bail!("The provided private key jwt must be for an application");
		}

		Ok(Self { domain, client, private_key_jwt: Arc::new(RwLock::new(private_key_jwt)) })
	}

	/// Verify the opaque token
	pub async fn verify(&self, token: String) -> Result<JwtPayload, OpaqueTokenValidationError> {
		if self.private_key_jwt.read().await.is_expired() {
			self.private_key_jwt.write().await.renew()?;
		}
		let jwt = self.private_key_jwt.read().await.token.clone();
		let resp = self
			.client
			.post(self.domain.clone().join("oauth/v2/introspect")?)
			.header("Content-Type", "application/x-www-form-urlencoded")
			.form(&[
				(
					"client_assertion_type".to_owned(),
					"urn:ietf:params:oauth:client-assertion-type:jwt-bearer".to_owned(),
				),
				("client_assertion".to_owned(), jwt),
				("token".to_owned(), token),
			])
			.send()
			.await?;

		if !resp.status().is_success() {
			return Err(OpaqueTokenValidationError::BadStatusCodeError(resp.status()));
		}

		let auth_resp: AuthResponse = resp.json().await?;
		if !auth_resp.active {
			return Err(OpaqueTokenValidationError::TokenInactiveError);
		}

		Ok(JwtPayload::from_map(auth_resp.claims)?)
	}
}

#[derive(Deserialize, Serialize)]
pub(super) struct AuthResponse {
	pub active: bool,
	#[serde(flatten)]
	pub claims: serde_json::Map<String, serde_json::Value>,
}

/// Enum for errors that can happen whilst verifying the token
#[derive(Debug, thiserror::Error)]
pub enum OpaqueTokenValidationError {
	/// General error from reqwest middleware request
	#[error("Failed to do the reqwest middleware request: {0}")]
	ReqwestMiddlewareError(#[from] reqwest_middleware::Error),
	/// General error from reqwest request
	#[error("Failed to do the reqwest: {0}")]
	ReqwestError(#[from] reqwest::Error),
	/// Requested returned with a bad status code error
	#[error("The request returned with a bad status code: {0}")]
	BadStatusCodeError(reqwest::StatusCode),
	/// Token inactive error
	#[error("The token is inactive")]
	TokenInactiveError,
	/// Parsing the body as auth response error
	#[error("Failed to parse the body as auth response: {0}")]
	ParsingAuthResponseError(#[from] serde_json::Error),
	/// Building the jwt payload error
	#[error("Failed to build the jwt payload: {0}")]
	BuildJwtPayloadError(#[from] josekit::JoseError),
	/// Building the jwt payload error
	#[error("Failed to build the private key jwt: {0}")]
	BuildPrivateKeyJwtError(#[from] anyhow::Error),
	/// Url join error
	#[error("Failed to join the url: {0}")]
	UrlJoinError(#[from] url::ParseError),
}
