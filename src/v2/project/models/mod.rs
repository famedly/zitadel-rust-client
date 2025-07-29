// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

#![allow(missing_docs, unused_imports, non_snake_case, missing_debug_implementations)]
#![allow(
	clippy::must_use_candidate,
	clippy::return_self_not_must_use,
	clippy::redundant_field_names,
	clippy::new_without_default,
	clippy::used_underscore_binding,
	clippy::empty_line_after_doc_comments
)]

mod v1_project_grant;
pub use self::v1_project_grant::V1ProjectGrant;
mod v1_project_grant_response;
pub use self::v1_project_grant_response::V1ProjectGrantResponse;

// TODO(farcaller): sort out files
pub struct File;
