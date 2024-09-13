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
#[allow(unused_imports)]
use serde_json::Value;

use crate::users::models;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserServiceRetrieveIdentityProviderIntentBody {
	/// token of the idp intent, previously returned on the success response of
	/// the IDP callback
	#[serde(rename = "idpIntentToken")]
	idp_intent_token: Option<String>,
}

impl UserServiceRetrieveIdentityProviderIntentBody {
	pub fn new() -> UserServiceRetrieveIdentityProviderIntentBody {
		UserServiceRetrieveIdentityProviderIntentBody { idp_intent_token: None }
	}

	pub fn set_idp_intent_token(&mut self, idp_intent_token: String) {
		self.idp_intent_token = Some(idp_intent_token);
	}

	pub fn with_idp_intent_token(
		mut self,
		idp_intent_token: String,
	) -> UserServiceRetrieveIdentityProviderIntentBody {
		self.idp_intent_token = Some(idp_intent_token);
		self
	}

	pub fn idp_intent_token(&self) -> Option<&String> {
		self.idp_intent_token.as_ref()
	}

	pub fn reset_idp_intent_token(&mut self) {
		self.idp_intent_token = None;
	}
}
