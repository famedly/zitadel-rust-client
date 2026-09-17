// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

//! Verifier for JWT tokens

use std::sync::Arc;

use anyhow_ext::Result;
use cache_control::CacheControl;
use jsonwebtoken::{
	Algorithm, DecodingKey, Validation, decode, decode_header,
	errors::{Error as JwtError, ErrorKind},
	jwk::JwkSet,
};
use reqwest::{Client, Response, header};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
#[cfg(feature = "telemetry")]
use rust_telemetry::reqwest_middleware::OtelMiddleware;
use serde::de::DeserializeOwned;
use time::OffsetDateTime;
use tokio::sync::RwLock;
use url::Url;

use super::payload::JwtPayload;
use crate::v2::DEFAULT_TIMEOUT;

/// Zitadel client to verify a token's validity
#[derive(Debug, Clone)]
pub struct ZitadelJWTVerifier {
	/// Zitadel domain
	domain: Url,
	/// Client for performing the requests
	client: ClientWithMiddleware,
	/// Key set cache from Zitadel
	jwks_cache: Arc<RwLock<JwkSetCache>>,
}

/// Cache of Zitadel jwks
#[derive(Debug, Clone)]
struct JwkSetCache {
	/// Key set from Zitadel
	jwks: JwkSet,
	/// Time when the jwks is no longer valid
	expires_at: OffsetDateTime,
}

impl ZitadelJWTVerifier {
	/// Creates a new verifier to verify with a specific server
	#[must_use]
	pub fn new(url: Url) -> Self {
		// We expect the client to be built successfully because we are only
		// adding a timeout to it
		#[allow(clippy::expect_used)]
		let client_builder = ClientBuilder::new(
			Client::builder()
				.timeout(DEFAULT_TIMEOUT)
				.build()
				.expect("Failed to build reqwest client"),
		);
		#[cfg(feature = "telemetry")]
		let client_builder = client_builder.with(OtelMiddleware);
		let client = client_builder.build();
		Self {
			domain: url,
			client,
			jwks_cache: Arc::new(RwLock::new(JwkSetCache {
				jwks: JwkSet { keys: Vec::new() },
				expires_at: OffsetDateTime::now_utc(),
			})),
		}
	}

	/// Verifies the token and deserializes registered plus extra claims
	/// The performed verifications are:
	///     - Token signature
	///     - Token not expired
	///     - Token not issued in the future
	///     - Token issuer is the expected server
	///
	/// Extra claims are `T`. `JwtPayload` with no type argument keeps every
	/// remaining claim in a [`std::collections::HashMap`]. Do not include
	/// `iss`, `exp`, or `iat` on `T`.
	pub async fn verify<T: DeserializeOwned>(
		&self,
		token: String,
	) -> Result<JwtPayload<T>, TokenValidationError> {
		use TokenValidationError::*;

		let header = decode_header(&token)?;
		let kid = header.kid.ok_or(BadToken("No kid"))?;

		let (mut jwk, expires_at) = {
			let jwks_cache = self.jwks_cache.read().await;
			(jwks_cache.jwks.find(&kid).cloned(), jwks_cache.expires_at)
		};
		if expires_at < OffsetDateTime::now_utc() || jwk.is_none() {
			let mut jwks_cache = self.jwks_cache.write().await;
			*jwks_cache = self.get_jwks().await?;
			jwk = jwks_cache.jwks.find(&kid).cloned();
			tracing::debug!("Updated JWKs");
		}

		let jwk = jwk.ok_or(KidNotFoundError(kid))?;
		let decoding_key = DecodingKey::from_jwk(&jwk)?;
		let payload =
			decode::<JwtPayload<T>>(&token, &decoding_key, &signature_validation(&self.domain))?
				.claims;

		payload
			.issued_at()
			.filter(|iat| *iat < OffsetDateTime::now_utc())
			.ok_or(TokenIssuedInFutureError)?;

		Ok(payload)
	}

	/// Gets the jwks and the expiration date for it
	async fn get_jwks(&self) -> Result<JwkSetCache, RenewJwksError> {
		let mut url = self.domain.clone();
		url.set_path("oauth/v2/keys");
		let response = self.client.get(url).send().await?;

		let status_code = response.status();
		if !status_code.is_success() {
			return Err(RenewJwksError::BadStatusCodeError(status_code));
		}

		let expires_at = Self::get_cache_control(&response);

		let body = response.bytes().await?;
		let jwks = serde_json::from_slice(&body)?;

		Ok(JwkSetCache { jwks, expires_at })
	}

	/// Retrieves the cache-control information from the header
	fn get_cache_control(response: &Response) -> OffsetDateTime {
		let cache_control = response
			.headers()
			.get(header::CACHE_CONTROL)
			.map(|c| c.to_str().unwrap_or_default())
			.unwrap_or_default();
		let Some(cache_control) = CacheControl::from_value(cache_control) else {
			return OffsetDateTime::now_utc();
		};

		if cache_control.no_store {
			return OffsetDateTime::now_utc();
		}

		let max_age = cache_control.max_age.unwrap_or_default();

		OffsetDateTime::now_utc() + max_age
	}
}

/// JWT validation: RS256 signature, expected `iss`, and `exp` with zero leeway.
/// `iat` is still checked in [`ZitadelJWTVerifier::verify`]; `nbf` and `aud`
/// are not.
fn signature_validation(domain: &Url) -> Validation {
	// Url always comes with an '/' at the end. We need to remove it before
	// for checking
	let expected_issuer = domain.as_str().strip_suffix("/").unwrap_or(domain.as_str());
	let mut validation = Validation::new(Algorithm::RS256);
	validation.leeway = 0;
	validation.validate_exp = true;
	validation.validate_aud = false;
	validation.validate_nbf = false;
	validation.set_issuer(&[expected_issuer]);
	validation.set_required_spec_claims(&["exp", "iss"]);
	validation
}

/// Enum for errors that can happen whilst verifying the token
#[derive(Debug, thiserror::Error)]
pub enum TokenValidationError {
	/// Bad token error
	#[error("Bad token: {0}")]
	BadToken(&'static str),
	/// Error renewing the jwks
	#[error("Failed to renew the jwks: {0}")]
	RenewJwksError(#[from] RenewJwksError),
	/// kid not found at the token
	#[error("Unknown JWK, kid: {0}")]
	KidNotFoundError(String),
	/// Error decoding and verifying the token
	#[error("Failed to decode the token with the verifier: {0}")]
	TokenDecodeError(JwtError),
	/// Wrong issuer error
	#[error("The token came from a different issuer than the expected. Token issuer: '{0}'")]
	TokenIssuerError(String),
	/// Token expired error
	#[error("The token has expired")]
	TokenExpiredError,
	/// Token issued in future error
	#[error("Token issued in future")]
	TokenIssuedInFutureError,
	/// Token claims are missing, the wrong type, or do not match `T`
	#[error("Malformed token claims: {0}")]
	MalformedToken(JwtError),
}

impl From<JwtError> for TokenValidationError {
	fn from(error: JwtError) -> Self {
		match error.kind() {
			ErrorKind::Json(_) => Self::MalformedToken(error),
			ErrorKind::ExpiredSignature => Self::TokenExpiredError,
			ErrorKind::InvalidIssuer => Self::TokenIssuerError(error.to_string()),
			_ => Self::TokenDecodeError(error),
		}
	}
}

/// Enum for errors that can happen whilst renewing the jwks
#[derive(Debug, thiserror::Error)]
pub enum RenewJwksError {
	/// General error from reqwest middleware request
	#[error("Failed to do the reqwest middleware request: {0}")]
	ReqwestMiddlewareError(#[from] reqwest_middleware::Error),
	/// General error from reqwest request
	#[error("Failed to do the reqwest: {0}")]
	ReqwestError(#[from] reqwest::Error),
	/// Requested returned with a bad status code error
	#[error("The request returned with a bad status code: {0}")]
	BadStatusCodeError(reqwest::StatusCode),
	/// Parsing the body as jwks error
	#[error("Failed to parse the token: {0}")]
	ParsingTokenError(#[from] serde_json::Error),
}
