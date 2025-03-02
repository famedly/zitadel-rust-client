/*
 * Management API
 *
 * The management API is as the name states the interface where systems can
 * mutate IAM objects like organizations, projects, clients, users and so on
 * if they have the necessary access rights.
 *
 * OpenAPI spec version: 1.0
 * Contact: hi@zitadel.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

use crate::v2::management::models;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Idpv1Idp {
	#[serde(rename = "id")]
	id: Option<String>,
	#[serde(rename = "details")]
	details: Option<models::V1ObjectDetails>,
	/// the state of the identity provider
	#[serde(rename = "state")]
	state: Option<models::V1IdpState>,
	#[serde(rename = "name")]
	name: Option<String>,
	/// some identity providers specify the styling of the button to their login
	#[serde(rename = "stylingType")]
	styling_type: Option<models::V1IdpStylingType>,
	/// the administrator of this identity provider
	#[serde(rename = "owner")]
	owner: Option<models::V1IdpOwnerType>,
	#[serde(rename = "oidcConfig")]
	oidc_config: Option<models::Zitadelidpv1OidcConfig>,
	#[serde(rename = "jwtConfig")]
	jwt_config: Option<models::V1JwtConfig>,
	#[serde(rename = "autoRegister")]
	auto_register: Option<bool>,
}

impl Idpv1Idp {
	pub fn new() -> Idpv1Idp {
		Idpv1Idp {
			id: None,
			details: None,
			state: None,
			name: None,
			styling_type: None,
			owner: None,
			oidc_config: None,
			jwt_config: None,
			auto_register: None,
		}
	}

	pub fn set_id(&mut self, id: String) {
		self.id = Some(id);
	}

	pub fn with_id(mut self, id: String) -> Idpv1Idp {
		self.id = Some(id);
		self
	}

	pub fn id(&self) -> Option<&String> {
		self.id.as_ref()
	}

	pub fn reset_id(&mut self) {
		self.id = None;
	}

	pub fn set_details(&mut self, details: models::V1ObjectDetails) {
		self.details = Some(details);
	}

	pub fn with_details(mut self, details: models::V1ObjectDetails) -> Idpv1Idp {
		self.details = Some(details);
		self
	}

	pub fn details(&self) -> Option<&models::V1ObjectDetails> {
		self.details.as_ref()
	}

	pub fn reset_details(&mut self) {
		self.details = None;
	}

	pub fn set_state(&mut self, state: models::V1IdpState) {
		self.state = Some(state);
	}

	pub fn with_state(mut self, state: models::V1IdpState) -> Idpv1Idp {
		self.state = Some(state);
		self
	}

	pub fn state(&self) -> Option<&models::V1IdpState> {
		self.state.as_ref()
	}

	pub fn reset_state(&mut self) {
		self.state = None;
	}

	pub fn set_name(&mut self, name: String) {
		self.name = Some(name);
	}

	pub fn with_name(mut self, name: String) -> Idpv1Idp {
		self.name = Some(name);
		self
	}

	pub fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	pub fn reset_name(&mut self) {
		self.name = None;
	}

	pub fn set_styling_type(&mut self, styling_type: models::V1IdpStylingType) {
		self.styling_type = Some(styling_type);
	}

	pub fn with_styling_type(mut self, styling_type: models::V1IdpStylingType) -> Idpv1Idp {
		self.styling_type = Some(styling_type);
		self
	}

	pub fn styling_type(&self) -> Option<&models::V1IdpStylingType> {
		self.styling_type.as_ref()
	}

	pub fn reset_styling_type(&mut self) {
		self.styling_type = None;
	}

	pub fn set_owner(&mut self, owner: models::V1IdpOwnerType) {
		self.owner = Some(owner);
	}

	pub fn with_owner(mut self, owner: models::V1IdpOwnerType) -> Idpv1Idp {
		self.owner = Some(owner);
		self
	}

	pub fn owner(&self) -> Option<&models::V1IdpOwnerType> {
		self.owner.as_ref()
	}

	pub fn reset_owner(&mut self) {
		self.owner = None;
	}

	pub fn set_oidc_config(&mut self, oidc_config: models::Zitadelidpv1OidcConfig) {
		self.oidc_config = Some(oidc_config);
	}

	pub fn with_oidc_config(mut self, oidc_config: models::Zitadelidpv1OidcConfig) -> Idpv1Idp {
		self.oidc_config = Some(oidc_config);
		self
	}

	pub fn oidc_config(&self) -> Option<&models::Zitadelidpv1OidcConfig> {
		self.oidc_config.as_ref()
	}

	pub fn reset_oidc_config(&mut self) {
		self.oidc_config = None;
	}

	pub fn set_jwt_config(&mut self, jwt_config: models::V1JwtConfig) {
		self.jwt_config = Some(jwt_config);
	}

	pub fn with_jwt_config(mut self, jwt_config: models::V1JwtConfig) -> Idpv1Idp {
		self.jwt_config = Some(jwt_config);
		self
	}

	pub fn jwt_config(&self) -> Option<&models::V1JwtConfig> {
		self.jwt_config.as_ref()
	}

	pub fn reset_jwt_config(&mut self) {
		self.jwt_config = None;
	}

	pub fn set_auto_register(&mut self, auto_register: bool) {
		self.auto_register = Some(auto_register);
	}

	pub fn with_auto_register(mut self, auto_register: bool) -> Idpv1Idp {
		self.auto_register = Some(auto_register);
		self
	}

	pub fn auto_register(&self) -> Option<&bool> {
		self.auto_register.as_ref()
	}

	pub fn reset_auto_register(&mut self) {
		self.auto_register = None;
	}
}
