// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

pub mod jwt;
pub mod opaque;
pub use jwt::{TokenValidationError, ZitadelJWTVerifier};
pub use opaque::{OpaqueTokenValidationError, ZitadelOpaqueTokenVerifier};
