//! Rust client for [Zitadel](https://zitadel.com/)

#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]

#[cfg(feature = "v1")]
pub mod v1;
pub mod v2;
