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

use crate::v2::users::models;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SearchQuery {
	#[serde(rename = "userNameQuery")]
	user_name_query: Option<models::UserNameQuery>,
	#[serde(rename = "firstNameQuery")]
	first_name_query: Option<models::FirstNameQuery>,
	#[serde(rename = "lastNameQuery")]
	last_name_query: Option<models::LastNameQuery>,
	#[serde(rename = "nickNameQuery")]
	nick_name_query: Option<models::NickNameQuery>,
	#[serde(rename = "displayNameQuery")]
	display_name_query: Option<models::DisplayNameQuery>,
	#[serde(rename = "emailQuery")]
	email_query: Option<models::EmailQuery>,
	#[serde(rename = "stateQuery")]
	state_query: Option<models::StateQuery>,
	#[serde(rename = "typeQuery")]
	type_query: Option<models::TypeQuery>,
	#[serde(rename = "loginNameQuery")]
	login_name_query: Option<models::LoginNameQuery>,
	#[serde(rename = "inUserIdsQuery")]
	in_user_ids_query: Option<models::InUserIdQuery>,
	#[serde(rename = "orQuery")]
	or_query: Option<models::OrQuery>,
	#[serde(rename = "andQuery")]
	and_query: Option<models::AndQuery>,
	#[serde(rename = "notQuery")]
	not_query: Option<models::NotQuery>,
	#[serde(rename = "inUserEmailsQuery")]
	in_user_emails_query: Option<models::InUserEmailsQuery>,
	#[serde(rename = "organizationIdQuery")]
	organization_id_query: Option<models::OrganizationIdQuery>,
}

impl SearchQuery {
	pub fn new() -> SearchQuery {
		SearchQuery {
			user_name_query: None,
			first_name_query: None,
			last_name_query: None,
			nick_name_query: None,
			display_name_query: None,
			email_query: None,
			state_query: None,
			type_query: None,
			login_name_query: None,
			in_user_ids_query: None,
			or_query: None,
			and_query: None,
			not_query: None,
			in_user_emails_query: None,
			organization_id_query: None,
		}
	}

	pub fn set_user_name_query(&mut self, user_name_query: models::UserNameQuery) {
		self.user_name_query = Some(user_name_query);
	}

	pub fn with_user_name_query(mut self, user_name_query: models::UserNameQuery) -> SearchQuery {
		self.user_name_query = Some(user_name_query);
		self
	}

	pub fn user_name_query(&self) -> Option<&models::UserNameQuery> {
		self.user_name_query.as_ref()
	}

	pub fn reset_user_name_query(&mut self) {
		self.user_name_query = None;
	}

	pub fn set_first_name_query(&mut self, first_name_query: models::FirstNameQuery) {
		self.first_name_query = Some(first_name_query);
	}

	pub fn with_first_name_query(
		mut self,
		first_name_query: models::FirstNameQuery,
	) -> SearchQuery {
		self.first_name_query = Some(first_name_query);
		self
	}

	pub fn first_name_query(&self) -> Option<&models::FirstNameQuery> {
		self.first_name_query.as_ref()
	}

	pub fn reset_first_name_query(&mut self) {
		self.first_name_query = None;
	}

	pub fn set_last_name_query(&mut self, last_name_query: models::LastNameQuery) {
		self.last_name_query = Some(last_name_query);
	}

	pub fn with_last_name_query(mut self, last_name_query: models::LastNameQuery) -> SearchQuery {
		self.last_name_query = Some(last_name_query);
		self
	}

	pub fn last_name_query(&self) -> Option<&models::LastNameQuery> {
		self.last_name_query.as_ref()
	}

	pub fn reset_last_name_query(&mut self) {
		self.last_name_query = None;
	}

	pub fn set_nick_name_query(&mut self, nick_name_query: models::NickNameQuery) {
		self.nick_name_query = Some(nick_name_query);
	}

	pub fn with_nick_name_query(mut self, nick_name_query: models::NickNameQuery) -> SearchQuery {
		self.nick_name_query = Some(nick_name_query);
		self
	}

	pub fn nick_name_query(&self) -> Option<&models::NickNameQuery> {
		self.nick_name_query.as_ref()
	}

	pub fn reset_nick_name_query(&mut self) {
		self.nick_name_query = None;
	}

	pub fn set_display_name_query(&mut self, display_name_query: models::DisplayNameQuery) {
		self.display_name_query = Some(display_name_query);
	}

	pub fn with_display_name_query(
		mut self,
		display_name_query: models::DisplayNameQuery,
	) -> SearchQuery {
		self.display_name_query = Some(display_name_query);
		self
	}

	pub fn display_name_query(&self) -> Option<&models::DisplayNameQuery> {
		self.display_name_query.as_ref()
	}

	pub fn reset_display_name_query(&mut self) {
		self.display_name_query = None;
	}

	pub fn set_email_query(&mut self, email_query: models::EmailQuery) {
		self.email_query = Some(email_query);
	}

	pub fn with_email_query(mut self, email_query: models::EmailQuery) -> SearchQuery {
		self.email_query = Some(email_query);
		self
	}

	pub fn email_query(&self) -> Option<&models::EmailQuery> {
		self.email_query.as_ref()
	}

	pub fn reset_email_query(&mut self) {
		self.email_query = None;
	}

	pub fn set_state_query(&mut self, state_query: models::StateQuery) {
		self.state_query = Some(state_query);
	}

	pub fn with_state_query(mut self, state_query: models::StateQuery) -> SearchQuery {
		self.state_query = Some(state_query);
		self
	}

	pub fn state_query(&self) -> Option<&models::StateQuery> {
		self.state_query.as_ref()
	}

	pub fn reset_state_query(&mut self) {
		self.state_query = None;
	}

	pub fn set_type_query(&mut self, type_query: models::TypeQuery) {
		self.type_query = Some(type_query);
	}

	pub fn with_type_query(mut self, type_query: models::TypeQuery) -> SearchQuery {
		self.type_query = Some(type_query);
		self
	}

	pub fn type_query(&self) -> Option<&models::TypeQuery> {
		self.type_query.as_ref()
	}

	pub fn reset_type_query(&mut self) {
		self.type_query = None;
	}

	pub fn set_login_name_query(&mut self, login_name_query: models::LoginNameQuery) {
		self.login_name_query = Some(login_name_query);
	}

	pub fn with_login_name_query(
		mut self,
		login_name_query: models::LoginNameQuery,
	) -> SearchQuery {
		self.login_name_query = Some(login_name_query);
		self
	}

	pub fn login_name_query(&self) -> Option<&models::LoginNameQuery> {
		self.login_name_query.as_ref()
	}

	pub fn reset_login_name_query(&mut self) {
		self.login_name_query = None;
	}

	pub fn set_in_user_ids_query(&mut self, in_user_ids_query: models::InUserIdQuery) {
		self.in_user_ids_query = Some(in_user_ids_query);
	}

	pub fn with_in_user_ids_query(
		mut self,
		in_user_ids_query: models::InUserIdQuery,
	) -> SearchQuery {
		self.in_user_ids_query = Some(in_user_ids_query);
		self
	}

	pub fn in_user_ids_query(&self) -> Option<&models::InUserIdQuery> {
		self.in_user_ids_query.as_ref()
	}

	pub fn reset_in_user_ids_query(&mut self) {
		self.in_user_ids_query = None;
	}

	pub fn set_or_query(&mut self, or_query: models::OrQuery) {
		self.or_query = Some(or_query);
	}

	pub fn with_or_query(mut self, or_query: models::OrQuery) -> SearchQuery {
		self.or_query = Some(or_query);
		self
	}

	pub fn or_query(&self) -> Option<&models::OrQuery> {
		self.or_query.as_ref()
	}

	pub fn reset_or_query(&mut self) {
		self.or_query = None;
	}

	pub fn set_and_query(&mut self, and_query: models::AndQuery) {
		self.and_query = Some(and_query);
	}

	pub fn with_and_query(mut self, and_query: models::AndQuery) -> SearchQuery {
		self.and_query = Some(and_query);
		self
	}

	pub fn and_query(&self) -> Option<&models::AndQuery> {
		self.and_query.as_ref()
	}

	pub fn reset_and_query(&mut self) {
		self.and_query = None;
	}

	pub fn set_not_query(&mut self, not_query: models::NotQuery) {
		self.not_query = Some(not_query);
	}

	pub fn with_not_query(mut self, not_query: models::NotQuery) -> SearchQuery {
		self.not_query = Some(not_query);
		self
	}

	pub fn not_query(&self) -> Option<&models::NotQuery> {
		self.not_query.as_ref()
	}

	pub fn reset_not_query(&mut self) {
		self.not_query = None;
	}

	pub fn set_in_user_emails_query(&mut self, in_user_emails_query: models::InUserEmailsQuery) {
		self.in_user_emails_query = Some(in_user_emails_query);
	}

	pub fn with_in_user_emails_query(
		mut self,
		in_user_emails_query: models::InUserEmailsQuery,
	) -> SearchQuery {
		self.in_user_emails_query = Some(in_user_emails_query);
		self
	}

	pub fn in_user_emails_query(&self) -> Option<&models::InUserEmailsQuery> {
		self.in_user_emails_query.as_ref()
	}

	pub fn reset_in_user_emails_query(&mut self) {
		self.in_user_emails_query = None;
	}

	pub fn set_organization_id_query(
		&mut self,
		organization_id_query: models::OrganizationIdQuery,
	) {
		self.organization_id_query = Some(organization_id_query);
	}

	pub fn with_organization_id_query(
		mut self,
		organization_id_query: models::OrganizationIdQuery,
	) -> SearchQuery {
		self.organization_id_query = Some(organization_id_query);
		self
	}

	pub fn organization_id_query(&self) -> Option<&models::OrganizationIdQuery> {
		self.organization_id_query.as_ref()
	}

	pub fn reset_organization_id_query(&mut self) {
		self.organization_id_query = None;
	}
}