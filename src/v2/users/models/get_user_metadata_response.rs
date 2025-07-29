// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::v2::users::models;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GetUserMetadataResponse {
	#[serde(rename = "metadata")]
	metadata: models::UserMetadataResponse,
}

impl GetUserMetadataResponse {
	pub fn new(metadata: models::UserMetadataResponse) -> GetUserMetadataResponse {
		GetUserMetadataResponse { metadata }
	}
	pub fn set_details(&mut self, metadata: models::UserMetadataResponse) {
		self.metadata = metadata;
	}

	pub fn with_details(
		mut self,
		metadata: models::UserMetadataResponse,
	) -> GetUserMetadataResponse {
		self.metadata = metadata;
		self
	}

	pub fn metadata(&self) -> &models::UserMetadataResponse {
		&self.metadata
	}
}
