// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

//! Token verification helpers

/// JWT access-token verifier
pub mod jwt;
/// Opaque access-token verifier
pub mod opaque;
/// Claims from a verified JWT or an introspected opaque token
pub mod payload;
pub use jwt::{TokenValidationError, ZitadelJWTVerifier};
pub use opaque::{OpaqueTokenValidationError, ZitadelOpaqueTokenVerifier};
pub use payload::JwtPayload;
