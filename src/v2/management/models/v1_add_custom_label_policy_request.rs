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
pub struct V1AddCustomLabelPolicyRequest {
	/// Represents a color scheme
	#[serde(rename = "primaryColor")]
	primary_color: Option<String>,
	/// hides the org suffix on the login form if the scope
	/// \"urn:zitadel:iam:org:domain:primary:{domainname}\" is set
	#[serde(rename = "hideLoginNameSuffix")]
	hide_login_name_suffix: Option<bool>,
	/// hex value for warn color
	#[serde(rename = "warnColor")]
	warn_color: Option<String>,
	/// hex value for background color
	#[serde(rename = "backgroundColor")]
	background_color: Option<String>,
	/// hex value for font color
	#[serde(rename = "fontColor")]
	font_color: Option<String>,
	/// hex value for the primary color dark theme
	#[serde(rename = "primaryColorDark")]
	primary_color_dark: Option<String>,
	/// hex value for background color dark theme
	#[serde(rename = "backgroundColorDark")]
	background_color_dark: Option<String>,
	/// hex value for warning color dark theme
	#[serde(rename = "warnColorDark")]
	warn_color_dark: Option<String>,
	/// hex value for font color dark theme
	#[serde(rename = "fontColorDark")]
	font_color_dark: Option<String>,
	#[serde(rename = "disableWatermark")]
	disable_watermark: Option<bool>,
	/// setting if there should be a restriction on which themes are available
	#[serde(rename = "themeMode")]
	theme_mode: Option<models::V1ThemeMode>,
}

impl V1AddCustomLabelPolicyRequest {
	pub fn new() -> V1AddCustomLabelPolicyRequest {
		V1AddCustomLabelPolicyRequest {
			primary_color: None,
			hide_login_name_suffix: None,
			warn_color: None,
			background_color: None,
			font_color: None,
			primary_color_dark: None,
			background_color_dark: None,
			warn_color_dark: None,
			font_color_dark: None,
			disable_watermark: None,
			theme_mode: None,
		}
	}

	pub fn set_primary_color(&mut self, primary_color: String) {
		self.primary_color = Some(primary_color);
	}

	pub fn with_primary_color(mut self, primary_color: String) -> V1AddCustomLabelPolicyRequest {
		self.primary_color = Some(primary_color);
		self
	}

	pub fn primary_color(&self) -> Option<&String> {
		self.primary_color.as_ref()
	}

	pub fn reset_primary_color(&mut self) {
		self.primary_color = None;
	}

	pub fn set_hide_login_name_suffix(&mut self, hide_login_name_suffix: bool) {
		self.hide_login_name_suffix = Some(hide_login_name_suffix);
	}

	pub fn with_hide_login_name_suffix(
		mut self,
		hide_login_name_suffix: bool,
	) -> V1AddCustomLabelPolicyRequest {
		self.hide_login_name_suffix = Some(hide_login_name_suffix);
		self
	}

	pub fn hide_login_name_suffix(&self) -> Option<&bool> {
		self.hide_login_name_suffix.as_ref()
	}

	pub fn reset_hide_login_name_suffix(&mut self) {
		self.hide_login_name_suffix = None;
	}

	pub fn set_warn_color(&mut self, warn_color: String) {
		self.warn_color = Some(warn_color);
	}

	pub fn with_warn_color(mut self, warn_color: String) -> V1AddCustomLabelPolicyRequest {
		self.warn_color = Some(warn_color);
		self
	}

	pub fn warn_color(&self) -> Option<&String> {
		self.warn_color.as_ref()
	}

	pub fn reset_warn_color(&mut self) {
		self.warn_color = None;
	}

	pub fn set_background_color(&mut self, background_color: String) {
		self.background_color = Some(background_color);
	}

	pub fn with_background_color(
		mut self,
		background_color: String,
	) -> V1AddCustomLabelPolicyRequest {
		self.background_color = Some(background_color);
		self
	}

	pub fn background_color(&self) -> Option<&String> {
		self.background_color.as_ref()
	}

	pub fn reset_background_color(&mut self) {
		self.background_color = None;
	}

	pub fn set_font_color(&mut self, font_color: String) {
		self.font_color = Some(font_color);
	}

	pub fn with_font_color(mut self, font_color: String) -> V1AddCustomLabelPolicyRequest {
		self.font_color = Some(font_color);
		self
	}

	pub fn font_color(&self) -> Option<&String> {
		self.font_color.as_ref()
	}

	pub fn reset_font_color(&mut self) {
		self.font_color = None;
	}

	pub fn set_primary_color_dark(&mut self, primary_color_dark: String) {
		self.primary_color_dark = Some(primary_color_dark);
	}

	pub fn with_primary_color_dark(
		mut self,
		primary_color_dark: String,
	) -> V1AddCustomLabelPolicyRequest {
		self.primary_color_dark = Some(primary_color_dark);
		self
	}

	pub fn primary_color_dark(&self) -> Option<&String> {
		self.primary_color_dark.as_ref()
	}

	pub fn reset_primary_color_dark(&mut self) {
		self.primary_color_dark = None;
	}

	pub fn set_background_color_dark(&mut self, background_color_dark: String) {
		self.background_color_dark = Some(background_color_dark);
	}

	pub fn with_background_color_dark(
		mut self,
		background_color_dark: String,
	) -> V1AddCustomLabelPolicyRequest {
		self.background_color_dark = Some(background_color_dark);
		self
	}

	pub fn background_color_dark(&self) -> Option<&String> {
		self.background_color_dark.as_ref()
	}

	pub fn reset_background_color_dark(&mut self) {
		self.background_color_dark = None;
	}

	pub fn set_warn_color_dark(&mut self, warn_color_dark: String) {
		self.warn_color_dark = Some(warn_color_dark);
	}

	pub fn with_warn_color_dark(
		mut self,
		warn_color_dark: String,
	) -> V1AddCustomLabelPolicyRequest {
		self.warn_color_dark = Some(warn_color_dark);
		self
	}

	pub fn warn_color_dark(&self) -> Option<&String> {
		self.warn_color_dark.as_ref()
	}

	pub fn reset_warn_color_dark(&mut self) {
		self.warn_color_dark = None;
	}

	pub fn set_font_color_dark(&mut self, font_color_dark: String) {
		self.font_color_dark = Some(font_color_dark);
	}

	pub fn with_font_color_dark(
		mut self,
		font_color_dark: String,
	) -> V1AddCustomLabelPolicyRequest {
		self.font_color_dark = Some(font_color_dark);
		self
	}

	pub fn font_color_dark(&self) -> Option<&String> {
		self.font_color_dark.as_ref()
	}

	pub fn reset_font_color_dark(&mut self) {
		self.font_color_dark = None;
	}

	pub fn set_disable_watermark(&mut self, disable_watermark: bool) {
		self.disable_watermark = Some(disable_watermark);
	}

	pub fn with_disable_watermark(
		mut self,
		disable_watermark: bool,
	) -> V1AddCustomLabelPolicyRequest {
		self.disable_watermark = Some(disable_watermark);
		self
	}

	pub fn disable_watermark(&self) -> Option<&bool> {
		self.disable_watermark.as_ref()
	}

	pub fn reset_disable_watermark(&mut self) {
		self.disable_watermark = None;
	}

	pub fn set_theme_mode(&mut self, theme_mode: models::V1ThemeMode) {
		self.theme_mode = Some(theme_mode);
	}

	pub fn with_theme_mode(
		mut self,
		theme_mode: models::V1ThemeMode,
	) -> V1AddCustomLabelPolicyRequest {
		self.theme_mode = Some(theme_mode);
		self
	}

	pub fn theme_mode(&self) -> Option<&models::V1ThemeMode> {
		self.theme_mode.as_ref()
	}

	pub fn reset_theme_mode(&mut self) {
		self.theme_mode = None;
	}
}
