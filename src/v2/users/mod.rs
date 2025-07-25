//! Implementation of the client for the zitadel user's v2 api
mod models;

use anyhow_ext::Result;
use anyhow_trace::anyhow_trace;
use base64::prelude::{Engine, BASE64_STANDARD};
use famedly_rust_utils::GenericCombinators as _;
use futures::Stream;
pub use models::*;
use serde_json::json;

use super::{
	pagination::{PaginationHandler, PaginationParams},
	Zitadel,
};
use crate::v2::HEADER_ZITADEL_ORGANIZATION_ID;

#[anyhow_trace]
impl Zitadel {
	/// Crates a new human user. [Docs](https://zitadel.com/docs/apis/resources/user_service_v2/user-service-add-human-user)
	pub async fn create_human_user(
		&self,
		body: AddHumanUserRequest,
	) -> Result<AddHumanUserResponse> {
		let request = self.client.post(self.make_url("v2/users/human")?).json(&body).build()?;

		Ok(self.send_request(request).await?)
	}
	/// Add link to an identity provider to an user
	pub async fn add_idp_link(
		&self,
		user_id: &str,
		body: UserServiceAddIdpLinkBody,
	) -> Result<AddIdpLinkResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/links"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Add OTP Email for a user
	/// Add a new One-Time-Password (OTP) Email factor to the authenticated
	/// user. OTP Email will enable the user to verify a OTP with the latest
	/// verified email. The email has to be verified to add the second factor.
	pub async fn add_otp_email(&self, user_id: &str) -> Result<AddOtpEmailResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/otp_email"))?)
			.json(&json!({}))
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Add OTP SMS for a user
	/// Add a new One-Time-Password (OTP) SMS factor to the authenticated user.
	/// OTP SMS will enable the user to verify a OTP with the latest verified
	/// phone number. The phone number has to be verified to add the second
	/// factor.
	pub async fn add_otpsms(&self, user_id: &str) -> Result<AddOtpsmsResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/otp_sms"))?)
			.json(&json!({}))
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Create a passkey registration link for a user
	/// Create a passkey registration link which includes a code and either
	/// return it or send it to the user.
	pub async fn create_passkey_registration_link(
		&self,
		user_id: &str,
		body: UserServiceCreatePasskeyRegistrationLinkBody,
	) -> Result<CreatePasskeyRegistrationLinkResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/passkeys/registration_link"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Deactivate user
	/// The state of the user will be changed to 'deactivated'. The user will
	/// not be able to log in anymore. The endpoint returns an error if the user
	/// is already in the state 'deactivated'. Use deactivate user when the user
	/// should not be able to use the account anymore, but you still need access
	/// to the user data.
	pub async fn deactivate_user(&self, user_id: &str) -> Result<DeactivateUserResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/deactivate"))?)
			.json(&json!({}))
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Delete user
	/// The state of the user will be changed to 'deleted'. The user will not be
	/// able to log in anymore. Endpoints requesting this user will return an
	/// error 'User not found.
	pub async fn delete_user(&self, user_id: &str) -> Result<DeleteUserResponse> {
		let request = self.client.delete(self.make_url(&format!("v2/users/{user_id}"))?).build()?;

		Ok(self.send_request(request).await?)
	}
	/// User by ID
	/// Returns the full user object (human or machine) including the profile,
	/// email, etc.
	pub async fn get_user_by_id(&self, user_id: &str) -> Result<GetUserByIdResponse> {
		let request = self.client.get(self.make_url(&format!("v2/users/{user_id}"))?).build()?;

		Ok(self.send_request(request).await?)
	}
	/// List all possible authentication methods of a user like password,
	/// passwordless, (T)OTP and more.
	pub async fn list_authentication_method_types(
		&self,
		user_id: &str,
	) -> Result<ListAuthenticationMethodTypesResponse> {
		let request = self
			.client
			.get(self.make_url(&format!("v2/users/{user_id}/authentication_methods"))?)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// List links to an identity provider of an user
	/// [Docs](https://zitadel.com/docs/apis/resources/user_service_v2/user-service-list-idp-links)
	pub fn list_idp_links(
		&self,
		user_id: &str,
		params: Option<PaginationParams>,
	) -> Result<impl Stream<Item = Result<IdpLink>> + Send + Sync> {
		Ok(PaginationHandler::new(
			self.clone(),
			params,
			self.make_url(&format!("v2/users/{user_id}/links/_search"))?,
			None, // Endpoint does not support org_id
		))
	}
	/// List passkeys of an user
	pub async fn list_passkeys(
		&self,
		user_id: &str,
		body: UserServiceListPasskeysBody,
	) -> Result<ListPasskeysResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/passkeys/_search"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}

	/// Search Users
	/// Search for users. By default, we will return users of your organization.
	/// Make sure to include a limit and sorting for pagination.
	/// [Docs](https://zitadel.com/docs/apis/resources/user_service_v2/user-service-list-users)
	pub fn list_users(
		&self,
		params: Option<PaginationParams>,
		sorting: Option<UserFieldName>,
		queries: Option<Vec<SearchQuery>>,
	) -> Result<impl Stream<Item = Result<User>> + Send + Sync> {
		Ok(PaginationHandler::new(
			self.clone(),
			(params, sorting, queries),
			self.make_url("v2/users")?,
			None,
		))
	}

	/// Lock user
	/// The state of the user will be changed to 'locked'. The user will not be
	/// able to log in anymore. The endpoint returns an error if the user is
	/// already in the state 'locked'. Use this endpoint if the user should not
	/// be able to log in temporarily because of an event that happened (wrong
	/// password, etc.).
	pub async fn lock_user(&self, user_id: &str) -> Result<LockUserResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/lock"))?)
			.json(&json!({}))
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Request a code to reset a password
	pub async fn password_reset(
		&self,
		user_id: &str,
		body: UserServicePasswordResetBody,
	) -> Result<PasswordResetResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/password_reset"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Reactivate user
	/// Reactivate a user with the state 'deactivated'. The user will be able to
	/// log in again afterward. The endpoint returns an error if the user is not
	/// in the state 'deactivated'.
	pub async fn reactivate_user(&self, user_id: &str) -> Result<ReactivateUserResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/reactivate"))?)
			.json(&json!({}))
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Start the registration of passkey for a user
	/// Start the registration of a passkey for a user, as a response the public
	/// key credential creation options are returned, which are used to verify
	/// the passkey.
	pub async fn register_passkey(
		&self,
		user_id: &str,
		body: UserServiceRegisterPasskeyBody,
	) -> Result<RegisterPasskeyResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/passkeys"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Start the registration of a TOTP generator for a user
	/// Start the registration of a TOTP generator for a user, as a response a
	/// secret returned, which is used to initialize a TOTP app or device.
	pub async fn register_totp(&self, user_id: &str) -> Result<RegisterTotpResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/totp"))?)
			.json(&json!({}))
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Start the registration of a u2f token for a user
	/// Start the registration of a u2f token for a user, as a response the
	/// public key credential creation options are returned, which are used to
	/// verify the u2f token.
	pub async fn register_u2_f(
		&self,
		user_id: &str,
		body: UserServiceRegisterU2FBody,
	) -> Result<RegisterU2FResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/u2f"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Remove link of an identity provider to an user
	pub async fn remove_idp_link(
		&self,
		user_id: &str,
		idp_id: &str,
		linked_user_id: &str,
	) -> Result<RemoveIdpLinkResponse> {
		let request = self
			.client
			.delete(self.make_url(&format!("v2/users/{user_id}/links/{idp_id}/{linked_user_id}"))?)
			.json(&json!({}))
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Remove One-Time-Password (OTP) Email from a user
	/// Remove the configured One-Time-Password (OTP) Email factor of a user. As
	/// only one OTP Email per user is allowed, the user will not have OTP Email
	/// as a second-factor afterward.
	pub async fn remove_otp_email(&self, user_id: &str) -> Result<RemoveOtpEmailResponse> {
		let request =
			self.client.delete(self.make_url(&format!("v2/users/{user_id}/otp_email"))?).build()?;

		Ok(self.send_request(request).await?)
	}
	/// Remove One-Time-Password (OTP) SMS from a user
	/// Remove the configured One-Time-Password (OTP) SMS factor of a user. As
	/// only one OTP SMS per user is allowed, the user will not have OTP SMS as
	/// a second-factor afterward.
	pub async fn remove_otpsms(&self, user_id: &str) -> Result<RemoveOtpsmsResponse> {
		let request =
			self.client.delete(self.make_url(&format!("v2/users/{user_id}/otp_sms"))?).build()?;

		Ok(self.send_request(request).await?)
	}
	/// Remove passkey from a user
	pub async fn remove_passkey(
		&self,
		user_id: &str,
		passkey_id: &str,
	) -> Result<RemovePasskeyResponse> {
		let request = self
			.client
			.delete(self.make_url(&format!("v2/users/{user_id}/passkeys/{passkey_id}"))?)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Delete the user phone
	pub async fn remove_phone(&self, user_id: &str) -> Result<RemovePhoneResponse> {
		let request = self
			.client
			.delete(self.make_url(&format!("v2/users/{user_id}/phone"))?)
			.json(&json!({}))
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Remove TOTP generator from a user
	/// Remove the configured TOTP generator of a user. As only one TOTP
	/// generator per user is allowed, the user will not have TOTP as a
	/// second-factor afterward.
	pub async fn remove_totp(&self, user_id: &str) -> Result<RemoveTotpResponse> {
		let request =
			self.client.delete(self.make_url(&format!("v2/users/{user_id}/totp"))?).build()?;

		Ok(self.send_request(request).await?)
	}
	/// Remove u2f token from a user
	pub async fn remove_u2_f(&self, user_id: &str, u2f_id: &str) -> Result<RemoveU2FResponse> {
		let request = self
			.client
			.delete(self.make_url(&format!("v2/users/{user_id}/u2f/{u2f_id}"))?)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Resend code to verify user email
	pub async fn resend_email_code(
		&self,
		user_id: &str,
		body: UserServiceResendEmailCodeBody,
	) -> Result<ResendEmailCodeResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/email/resend"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Resend code to verify user phone
	pub async fn resend_phone_code(
		&self,
		user_id: &str,
		body: UserServiceResendPhoneCodeBody,
	) -> Result<ResendPhoneCodeResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/phone/resend"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Retrieve the information returned by the identity provider
	/// Retrieve the information returned by the identity provider for
	/// registration or updating an existing user with new information.
	pub async fn retrieve_identity_provider_intent(
		&self,
		idp_intent_id: &str,
		body: UserServiceRetrieveIdentityProviderIntentBody,
	) -> Result<RetrieveIdentityProviderIntentResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/idp_intents/{idp_intent_id}"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Change the user email
	/// Change the email address of a user. If the state is set to not verified,
	/// a verification code will be generated, which can be either returned or
	/// sent to the user by email.
	pub async fn set_email(
		&self,
		user_id: &str,
		body: UserServiceSetEmailBody,
	) -> Result<SetEmailResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/email"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Change password
	/// Change the password of a user with either a verification code or the
	/// current password.
	pub async fn set_password(
		&self,
		user_id: &str,
		body: UserServiceSetPasswordBody,
	) -> Result<SetPasswordResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/password"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Set the user phone
	/// Set the phone number of a user. If the state is set to not verified, a
	/// verification code will be generated, which can be either returned or
	/// sent to the user by sms.
	pub async fn set_phone(
		&self,
		user_id: &str,
		body: UserServiceSetPhoneBody,
	) -> Result<SetPhoneResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/phone"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Start flow with an identity provider
	/// Start a flow with an identity provider, for external login, registration
	/// or linking.
	pub async fn start_identity_provider_intent(
		&self,
		body: StartIdentityProviderIntentRequest,
	) -> Result<StartIdentityProviderIntentResponse> {
		let request = self.client.post(self.make_url("v2/idp_intents")?).json(&body).build()?;

		Ok(self.send_request(request).await?)
	}
	/// Unlock user
	/// The state of the user will be changed to 'locked'. The user will not be
	/// able to log in anymore. The endpoint returns an error if the user is
	/// already in the state 'locked'. Use this endpoint if the user should not
	/// be able to log in temporarily because of an event that happened (wrong
	/// password, etc.).
	pub async fn unlock_user(&self, user_id: &str) -> Result<UnlockUserResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/unlock"))?)
			.json(&json!({}))
			.build()?;

		Ok(self.send_request(request).await?)
	}

	/// Update User
	/// Update all information for a user.
	pub async fn update_human_user(
		&self,
		user_id: &str,
		body: UpdateHumanUserRequest,
	) -> Result<UpdateHumanUserResponse> {
		let request = self
			.client
			.put(self.make_url(&format!("v2/users/human/{user_id}"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}

	/// Verify the email
	/// Verify the email with the generated code.
	pub async fn verify_email(
		&self,
		user_id: &str,
		body: UserServiceVerifyEmailBody,
	) -> Result<VerifyEmailResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/email/verify"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Verify a passkey for a user
	/// Verify the passkey registration with the public key credential.
	pub async fn verify_passkey_registration(
		&self,
		user_id: &str,
		passkey_id: &str,
		body: UserServiceVerifyPasskeyRegistrationBody,
	) -> Result<VerifyPasskeyRegistrationResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/passkeys/{passkey_id}"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Verify the phone
	/// Verify the phone with the generated code.
	pub async fn verify_phone(
		&self,
		user_id: &str,
		body: UserServiceVerifyPhoneBody,
	) -> Result<VerifyPhoneResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/phone/verify"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Verify a TOTP generator for a user
	/// Verify the TOTP registration with a generated code.
	pub async fn verify_totp_registration(
		&self,
		user_id: &str,
		body: UserServiceVerifyTotpRegistrationBody,
	) -> Result<VerifyTotpRegistrationResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/totp/verify"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Verify a u2f token for a user
	/// Verify the u2f token registration with the public key credential.
	pub async fn verify_u2_f_registration(
		&self,
		user_id: &str,
		u2f_id: &str,
		body: UserServiceVerifyU2FRegistrationBody,
	) -> Result<VerifyU2FRegistrationResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("v2/users/{user_id}/u2f/{u2f_id}"))?)
			.json(&body)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Get a metadata object from a user by a specific key
	pub async fn get_user_metadata(
		&self,
		user_id: &str,
		key: &str,
		org_id: Option<String>,
	) -> Result<GetUserMetadataResponse> {
		let request = self
			.client
			.get(self.make_url(&format!("management/v1/users/{user_id}/metadata/{key}"))?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Remove a metadata object from a user with a specific key
	pub async fn delete_user_metadata(&self, user_id: &str, key: &str) -> Result<Details> {
		let request = self
			.client
			.delete(self.make_url(&format!("/management/v1/users/{user_id}/metadata/{key}"))?)
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Set a metadata object for a user
	/// This method either adds or updates a metadata value for the requested
	pub async fn set_user_metadata(
		&self,
		user_id: &str,
		key: &str,
		value: &str,
	) -> Result<SetUserMetadataResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("/management/v1/users/{user_id}/metadata/{key}"))?)
			.json(&serde_json::json!({"value": BASE64_STANDARD.encode(value)}))
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Set a metadata object for a user in a bulk
	/// Add or update multiple metadata values for a user
	pub async fn set_user_metadata_bulk(
		&self,
		user_id: &str,
		body: Vec<SetMetadataEntry>,
	) -> Result<Details> {
		let request = self
			.client
			.post(self.make_url(&format!("/management/v1/users/{user_id}/metadata/_bulk"))?)
			.json(&serde_json::json!({"metadata": body}))
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Remove a list of metadata objects from a user with a list of keys
	pub async fn delete_user_metadata_bulk(
		&self,
		user_id: &str,
		body: Vec<String>,
	) -> Result<Details> {
		let request = self
			.client
			.delete(self.make_url(&format!("/management/v1/users/{user_id}/metadata/_bulk"))?)
			.json(&serde_json::json!({"keys": body}))
			.build()?;

		Ok(self.send_request(request).await?)
	}
	/// Get the metadata of a user filtered by your query
	/// [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-user-metadata)
	pub fn list_user_metadata(
		&self,
		user_id: &str,
		params: Option<PaginationParams>,
		queries: Option<Vec<KeyQuery>>,
	) -> Result<impl Stream<Item = Result<UserMetadataResponse>> + Send + Sync> {
		Ok(PaginationHandler::new(
			self.clone(),
			(params, queries),
			self.make_url(&format!("/management/v1/users/{user_id}/metadata/_search"))?,
			None, // TODO: Breaking change -> possibility to add org_id to function args
		))
	}
}
