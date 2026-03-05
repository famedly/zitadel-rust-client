// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

//! Private key jwt for the service account or app
//! This is used to authenticate the service account or app to the Zitadel API

use std::path::PathBuf;

use anyhow_ext::Result;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Service account for the private key jwt
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct PrivateKeyJWTFile {
	pub(crate) r#type: KeyType,
	#[serde(rename = "keyId")]
	pub(crate) key_id: String,
	pub(crate) key: String,
	/// Service account user id or app client id, depending on [`KeyType`].
	#[serde(alias = "userId", alias = "clientId")]
	pub(crate) id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub(crate) enum KeyType {
	#[serde(rename = "serviceaccount")]
	ServiceAccount,
	#[serde(rename = "application")]
	App,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
	iss: String,
	sub: String,
	aud: String,
	iat: i64,
	exp: i64,
}

/// Jwt made from the service account private key
#[derive(Clone)]
pub(crate) struct PrivateKeyJWT {
	/// Type of the private key jwt
	pub key_type: KeyType,
	/// Token for authenticating the requests
	pub token: String,
	/// Time when the token will expiry
	expiry: OffsetDateTime,

	header: Header,
	claims: Claims,
	key: EncodingKey,
}

impl PrivateKeyJWT {
	/// Create a new private key jwt
	pub async fn new(aud: String, private_key_jwt_file: &PathBuf) -> Result<Self> {
		let private_key_jwt_file: PrivateKeyJWTFile =
			serde_json::from_str(tokio::fs::read_to_string(private_key_jwt_file).await?.as_ref())?;

		let mut header = Header::new(Algorithm::RS256);
		header.kid = Some(private_key_jwt_file.key_id.clone());

		let expiry = OffsetDateTime::now_utc() + time::Duration::minutes(59);

		// The renew will fix it
		let claims = Claims {
			iss: private_key_jwt_file.id.clone(),
			sub: private_key_jwt_file.id.clone(),
			aud,
			iat: OffsetDateTime::now_utc().unix_timestamp(),
			exp: expiry.unix_timestamp(),
		};

		let key = EncodingKey::from_rsa_pem(private_key_jwt_file.key.as_bytes())?;

		let token = jsonwebtoken::encode(&header, &claims, &key)?;
		Ok(Self { token, expiry, header, claims, key, key_type: private_key_jwt_file.r#type })
	}

	/// Get the token
	/// If the token is expired, a new token will be generated
	pub fn get_token(&mut self) -> Result<String> {
		if self.expiry < OffsetDateTime::now_utc() {
			self.renew()?;
		}
		Ok(self.token.clone())
	}

	/// Check if the token is expired
	#[must_use]
	pub fn is_expired(&self) -> bool {
		self.expiry < OffsetDateTime::now_utc()
	}

	pub fn renew(&mut self) -> Result<()> {
		self.claims.iat = OffsetDateTime::now_utc().unix_timestamp();
		self.expiry = OffsetDateTime::now_utc() + time::Duration::minutes(59);
		self.claims.exp = self.expiry.unix_timestamp();

		self.token = jsonwebtoken::encode(&self.header, &self.claims, &self.key)?;
		Ok(())
	}
}

impl std::fmt::Debug for PrivateKeyJWT {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{{expiry: {:?}, aud: {}}}", self.token, self.claims.aud)
	}
}
