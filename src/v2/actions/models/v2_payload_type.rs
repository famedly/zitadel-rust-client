// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// Defines how the payload is formatted and secured when sent to the target.
///
/// Corresponds to the `PayloadType` enum in the Zitadel Actions v2 API proto.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum V2PayloadType {
	/// Sends the payload as plain JSON. A signature is included in the
	/// `X-ZITADEL-Signature` request header for integrity verification.
	/// This is the default for backwards compatibility.
	#[serde(rename = "PAYLOAD_TYPE_JSON")]
	Json,
	/// Sends the payload as a signed JWT in the request body. The receiver
	/// can verify authenticity and integrity using the public key published
	/// at the JWKS endpoint (`/oauth/v2/keys`).
	#[serde(rename = "PAYLOAD_TYPE_JWT")]
	Jwt,
	/// Sends the payload as an encrypted JWT (JWE). Provides confidentiality
	/// in addition to authenticity. Requires uploading a public key for
	/// encryption.
	#[serde(rename = "PAYLOAD_TYPE_JWE")]
	Jwe,
}
