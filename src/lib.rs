// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

//! Rust client for [Zitadel](https://zitadel.com/)

#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]

#[cfg(feature = "v1")]
pub mod v1;
pub mod v2;
