use std::sync::Arc;

use cache_control::CacheControl;
use josekit::{jwk::JwkSet, jws::RS256, jwt, jwt::JwtPayload};
use reqwest::{header, Client, Response};
use time::OffsetDateTime;
use tokio::sync::RwLock;
use url::Url;

/// Zitadel client to verify a token's validity
#[derive(Debug, Clone)]
pub struct ZitadelJWTVerifier {
	/// Zitadel domain
	domain: Url,
	/// Client for performing the requests
	client: Client,
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
		let jwks = JwkSet::new();
		Self {
			domain: url,
			client: Client::new(),
			jwks_cache: Arc::new(RwLock::new(JwkSetCache {
				jwks,
				expires_at: OffsetDateTime::now_utc(),
			})),
		}
	}

	/// Verifies if a token is valid and returns the token payload
	/// The performed verifications are:
	///     - Token signature
	///     - Token not expired
	///     - Token not used before 'not before'
	///     - Token issuer is the expected server
	pub async fn verify(&self, token: String) -> Result<JwtPayload, TokenValidationError> {
		use TokenValidationError::*;

		let header = jwt::decode_header(&token)?;
		let kid = header
			.claim("kid")
			.ok_or(BadToken("No kid"))?
			.as_str()
			.ok_or(BadToken("kid is not a string"))?;

		let (mut jwk, expires_at) = {
			let jwks_cache = self.jwks_cache.read().await;
			(jwks_cache.jwks.get(kid).first().copied().cloned(), jwks_cache.expires_at)
		};
		if expires_at < OffsetDateTime::now_utc() || jwk.is_none() {
			let mut jwks_cache = self.jwks_cache.write().await;
			*jwks_cache = self.get_jwks().await?;
			jwk = jwks_cache.jwks.get(kid).first().map(|&jwk| jwk.clone());
			tracing::debug!("Updated JWKs");
		}

		let jwk = jwk.ok_or(KidNotFoundError(kid.to_owned()))?;

		let verifier = RS256.verifier_from_jwk(&jwk).map_err(TokenDecodeError)?;
		let (payload, _) = jwt::decode_with_verifier(token, &verifier).map_err(TokenDecodeError)?;

		// Url always comes with an '/' at the end. We need to remove it before for
		// checking
		if !payload.issuer().is_some_and(|issuer| {
			issuer == self.domain.as_str().strip_suffix("/").unwrap_or(self.domain.as_str())
		}) {
			return Err(TokenIssuerError(payload.issuer().unwrap_or_default().to_owned()));
		}

		let now = OffsetDateTime::now_utc();
		(payload.expires_at().ok_or(MissingClaim("exp"))? > now)
			.then_some(())
			.ok_or(TokenExpiredError)?;

		(payload.not_before().ok_or(MissingClaim("nbf"))? < now)
			.then_some(())
			.ok_or(TokenNotBeforeError)?;

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
		let jwks = JwkSet::from_bytes(body).map_err(RenewJwksError::ParsingTokenError)?;

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
	TokenDecodeError(#[from] josekit::JoseError),
	/// Wrong issuer error
	#[error("The token came from a different issuer than the expected. Token issuer: '{0}'")]
	TokenIssuerError(String),
	/// Token expired error
	#[error("The token has expired")]
	TokenExpiredError,
	/// Token used before the 'not before'
	#[error("Token used before 'not before'")]
	TokenNotBeforeError,
	/// Missing token claim error
	#[error("Token missing the claim '{0}'")]
	MissingClaim(&'static str),
}

/// Enum for errors that can happen whilst renewing the jwks
#[derive(Debug, thiserror::Error)]
pub enum RenewJwksError {
	/// General error from reqwest request
	#[error("Failed to do the reqwest: {0}")]
	ReqwestError(#[from] reqwest::Error),
	/// Requested returned with a bad status code error
	#[error("The request returned with a bad status code: {0}")]
	BadStatusCodeError(reqwest::StatusCode),
	/// Parsing the body as jwks error
	#[error("Failed to parse the token: {0}")]
	ParsingTokenError(#[from] josekit::JoseError),
}
