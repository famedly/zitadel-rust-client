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
pub struct Zitadelidpv1SamlConfig {
	/// Metadata of the SAML identity provider.
	#[serde(rename = "metadataXml")]
	metadata_xml: Option<String>,
	/// Binding which defines the type of communication with the identity
	/// provider.
	#[serde(rename = "binding")]
	binding: Option<models::V1SamlBinding>,
	/// Boolean which defines if the authentication requests are signed.
	#[serde(rename = "withSignedRequest")]
	with_signed_request: Option<bool>,
	/// `nameid-format` for the SAML Request.
	#[serde(rename = "nameIdFormat")]
	name_id_format: Option<models::V1SamlNameIdFormat>,
	/// Optional name of the attribute, which will be used to map the user in
	/// case the nameid-format returned is
	/// `urn:oasis:names:tc:SAML:2.0:nameid-format:transient`.
	#[serde(rename = "transientMappingAttributeName")]
	transient_mapping_attribute_name: Option<String>,
}

impl Zitadelidpv1SamlConfig {
	pub fn new() -> Zitadelidpv1SamlConfig {
		Zitadelidpv1SamlConfig {
			metadata_xml: None,
			binding: None,
			with_signed_request: None,
			name_id_format: None,
			transient_mapping_attribute_name: None,
		}
	}

	pub fn set_metadata_xml(&mut self, metadata_xml: String) {
		self.metadata_xml = Some(metadata_xml);
	}

	pub fn with_metadata_xml(mut self, metadata_xml: String) -> Zitadelidpv1SamlConfig {
		self.metadata_xml = Some(metadata_xml);
		self
	}

	pub fn metadata_xml(&self) -> Option<&String> {
		self.metadata_xml.as_ref()
	}

	pub fn reset_metadata_xml(&mut self) {
		self.metadata_xml = None;
	}

	pub fn set_binding(&mut self, binding: models::V1SamlBinding) {
		self.binding = Some(binding);
	}

	pub fn with_binding(mut self, binding: models::V1SamlBinding) -> Zitadelidpv1SamlConfig {
		self.binding = Some(binding);
		self
	}

	pub fn binding(&self) -> Option<&models::V1SamlBinding> {
		self.binding.as_ref()
	}

	pub fn reset_binding(&mut self) {
		self.binding = None;
	}

	pub fn set_with_signed_request(&mut self, with_signed_request: bool) {
		self.with_signed_request = Some(with_signed_request);
	}

	pub fn with_with_signed_request(mut self, with_signed_request: bool) -> Zitadelidpv1SamlConfig {
		self.with_signed_request = Some(with_signed_request);
		self
	}

	pub fn with_signed_request(&self) -> Option<&bool> {
		self.with_signed_request.as_ref()
	}

	pub fn reset_with_signed_request(&mut self) {
		self.with_signed_request = None;
	}

	pub fn set_name_id_format(&mut self, name_id_format: models::V1SamlNameIdFormat) {
		self.name_id_format = Some(name_id_format);
	}

	pub fn with_name_id_format(
		mut self,
		name_id_format: models::V1SamlNameIdFormat,
	) -> Zitadelidpv1SamlConfig {
		self.name_id_format = Some(name_id_format);
		self
	}

	pub fn name_id_format(&self) -> Option<&models::V1SamlNameIdFormat> {
		self.name_id_format.as_ref()
	}

	pub fn reset_name_id_format(&mut self) {
		self.name_id_format = None;
	}

	pub fn set_transient_mapping_attribute_name(
		&mut self,
		transient_mapping_attribute_name: String,
	) {
		self.transient_mapping_attribute_name = Some(transient_mapping_attribute_name);
	}

	pub fn with_transient_mapping_attribute_name(
		mut self,
		transient_mapping_attribute_name: String,
	) -> Zitadelidpv1SamlConfig {
		self.transient_mapping_attribute_name = Some(transient_mapping_attribute_name);
		self
	}

	pub fn transient_mapping_attribute_name(&self) -> Option<&String> {
		self.transient_mapping_attribute_name.as_ref()
	}

	pub fn reset_transient_mapping_attribute_name(&mut self) {
		self.transient_mapping_attribute_name = None;
	}
}
