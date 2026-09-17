// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

//! Claims from a verified JWT or an introspected opaque token

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;

/// Registered claims used to validate a token, plus caller-defined claims
///
/// `T` is the extra claim set. Use [`HashMap<String, Value>`] (the default) to
/// keep every remaining claim. Do not put `iss`, `exp`, or `iat` on `T`; those
/// are deserialized into this wrapper.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JwtPayload<T = HashMap<String, Value>> {
	/// Token issuer
	#[serde(rename = "iss")]
	issuer: String,
	/// Expiration time as a UNIX timestamp
	#[serde(rename = "exp")]
	expires_at: i64,
	/// Issued-at time as a UNIX timestamp
	#[serde(rename = "iat")]
	issued_at: i64,
	/// Caller-defined claims
	#[serde(flatten)]
	claims: T,
}

impl<T> JwtPayload<T> {
	/// Returns the `iss` claim
	#[must_use]
	pub fn issuer(&self) -> &str {
		&self.issuer
	}

	/// Returns the `exp` claim as a UTC timestamp
	#[must_use]
	pub fn expires_at(&self) -> Option<OffsetDateTime> {
		OffsetDateTime::from_unix_timestamp(self.expires_at).ok()
	}

	/// Returns the `iat` claim as a UTC timestamp
	#[must_use]
	pub fn issued_at(&self) -> Option<OffsetDateTime> {
		OffsetDateTime::from_unix_timestamp(self.issued_at).ok()
	}

	/// Returns the extra claims
	#[must_use]
	pub const fn claims(&self) -> &T {
		&self.claims
	}

	/// Consumes the payload and returns the extra claims
	#[must_use]
	pub fn into_claims(self) -> T {
		self.claims
	}
}
