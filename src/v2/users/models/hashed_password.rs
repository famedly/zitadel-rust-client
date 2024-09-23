/*
 * User Service
 *
 * This API is intended to manage users in a ZITADEL instance.
 *
 * OpenAPI spec version: 2.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HashedPassword {
	/// \"Encoded hash of a password in Modular Crypt Format: https://zitadel.com/docs/concepts/architecture/secrets#hashed-secrets\"
	#[serde(rename = "hash")]
	hash: String,
	#[serde(rename = "changeRequired")]
	change_required: Option<bool>,
}

impl HashedPassword {
	pub fn new(hash: String) -> HashedPassword {
		HashedPassword { hash, change_required: None }
	}

	pub fn set_hash(&mut self, hash: String) {
		self.hash = hash;
	}

	pub fn with_hash(mut self, hash: String) -> HashedPassword {
		self.hash = hash;
		self
	}

	pub fn hash(&self) -> &String {
		&self.hash
	}

	pub fn set_change_required(&mut self, change_required: bool) {
		self.change_required = Some(change_required);
	}

	pub fn with_change_required(mut self, change_required: bool) -> HashedPassword {
		self.change_required = Some(change_required);
		self
	}

	pub fn change_required(&self) -> Option<&bool> {
		self.change_required.as_ref()
	}

	pub fn reset_change_required(&mut self) {
		self.change_required = None;
	}
}
