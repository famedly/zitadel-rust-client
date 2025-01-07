#![allow(missing_docs, unused_imports, non_snake_case, missing_debug_implementations)]
#![allow(
	clippy::must_use_candidate,
	clippy::return_self_not_must_use,
	clippy::redundant_field_names,
	clippy::new_without_default,
	clippy::used_underscore_binding,
	clippy::empty_line_after_doc_comments
)]

mod add_organization_request_admin;
pub use self::add_organization_request_admin::AddOrganizationRequestAdmin;
mod add_organization_response_created_admin;
pub use self::add_organization_response_created_admin::AddOrganizationResponseCreatedAdmin;
mod protobuf_any;
pub use self::protobuf_any::ProtobufAny;
mod rpc_status;
pub use self::rpc_status::RpcStatus;
mod v2_add_organization_request;
pub use self::v2_add_organization_request::V2AddOrganizationRequest;
mod v2_add_organization_response;
pub use self::v2_add_organization_response::V2AddOrganizationResponse;
mod v2_default_organization_query;
pub use self::v2_default_organization_query::V2DefaultOrganizationQuery;
mod v2_details;
pub use self::v2_details::V2Details;
mod v2_gender;
pub use self::v2_gender::V2Gender;
mod v2_hashed_password;
pub use self::v2_hashed_password::V2HashedPassword;
mod v2_idp_link;
pub use self::v2_idp_link::V2IdpLink;
mod v2_list_details;
pub use self::v2_list_details::V2ListDetails;
mod v2_list_organizations_request;
pub use self::v2_list_organizations_request::V2ListOrganizationsRequest;
mod v2_list_organizations_response;
pub use self::v2_list_organizations_response::V2ListOrganizationsResponse;
mod v2_list_query;
pub use self::v2_list_query::V2ListQuery;
mod v2_organization_domain_query;
pub use self::v2_organization_domain_query::V2OrganizationDomainQuery;
mod v2_organization_field_name;
pub use self::v2_organization_field_name::V2OrganizationFieldName;
mod v2_organization_id_query;
pub use self::v2_organization_id_query::V2OrganizationIdQuery;
mod v2_organization_name_query;
pub use self::v2_organization_name_query::V2OrganizationNameQuery;
mod v2_organization_state;
pub use self::v2_organization_state::V2OrganizationState;
mod v2_organization_state_query;
pub use self::v2_organization_state_query::V2OrganizationStateQuery;
mod v2_password;
pub use self::v2_password::V2Password;
mod v2_return_email_verification_code;
pub use self::v2_return_email_verification_code::V2ReturnEmailVerificationCode;
mod v2_return_phone_verification_code;
pub use self::v2_return_phone_verification_code::V2ReturnPhoneVerificationCode;
mod v2_send_email_verification_code;
pub use self::v2_send_email_verification_code::V2SendEmailVerificationCode;
mod v2_send_phone_verification_code;
pub use self::v2_send_phone_verification_code::V2SendPhoneVerificationCode;
mod v2_set_human_email;
pub use self::v2_set_human_email::V2SetHumanEmail;
mod v2_set_human_phone;
pub use self::v2_set_human_phone::V2SetHumanPhone;
mod v2_set_human_profile;
pub use self::v2_set_human_profile::V2SetHumanProfile;
mod v2_set_metadata_entry;
pub use self::v2_set_metadata_entry::V2SetMetadataEntry;
mod v2_text_query_method;
pub use self::v2_text_query_method::V2TextQueryMethod;
mod zitadelobjectv2_organization;
pub use self::zitadelobjectv2_organization::Zitadelobjectv2Organization;
mod zitadelorgv2_organization;
pub use self::zitadelorgv2_organization::Zitadelorgv2Organization;
mod zitadelorgv2_search_query;
pub use self::zitadelorgv2_search_query::Zitadelorgv2SearchQuery;

// TODO(farcaller): sort out files
pub struct File;
