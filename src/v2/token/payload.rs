// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

//! Claims from a verified JWT or an introspected opaque token

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use time::OffsetDateTime;

/// Claims from a verified JWT or an introspected opaque token
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct JwtPayload {
	/// Raw claim map
	claims: Map<String, Value>,
}

impl JwtPayload {
	/// Builds a payload from a JSON claim map
	#[must_use]
	pub fn from_map(claims: Map<String, Value>) -> Self {
		Self { claims }
	}

	/// Returns the raw claims
	#[must_use]
	pub fn claims(&self) -> &Map<String, Value> {
		&self.claims
	}

	/// Consumes the payload and returns the raw claim map
	#[must_use]
	pub fn into_map(self) -> Map<String, Value> {
		self.claims
	}

	/// Returns the named claim, if present
	#[must_use]
	pub fn claim(&self, name: &str) -> Option<&Value> {
		self.claims.get(name)
	}

	/// Returns the `iss` claim
	#[must_use]
	pub fn issuer(&self) -> Option<&str> {
		self.claim("iss").and_then(Value::as_str)
	}

	/// Returns the `exp` claim as a UTC timestamp
	#[must_use]
	pub fn expires_at(&self) -> Option<OffsetDateTime> {
		self.numeric_date("exp")
	}

	/// Returns the `iat` claim as a UTC timestamp
	#[must_use]
	pub fn issued_at(&self) -> Option<OffsetDateTime> {
		self.numeric_date("iat")
	}

	/// Parses a JWT numeric date claim
	fn numeric_date(&self, name: &str) -> Option<OffsetDateTime> {
		let value = self.claim(name)?;
		let timestamp =
			value.as_i64().or_else(|| value.as_u64().map(|n| i64::try_from(n).ok())?)?;
		OffsetDateTime::from_unix_timestamp(timestamp).ok()
	}
}
