//! Communication with Zitadel
#![allow(unused_imports)] // TODO: remove
#![allow(dead_code)] // TODO: remove

pub mod error;

use std::{
	path::{Path, PathBuf},
	sync::Arc,
};

// use self::clonable_service_interceptor::ClonableServiceAccountInterceptor;
// use super::Event;
use error::Result;
use tokio::sync::RwLock;
use tonic::{
	codegen::InterceptedService,
	metadata::AsciiMetadataValue,
	transport::{Channel, Endpoint},
	Request, Status,
};
use url::Url;
pub use zitadel::api::zitadel::{
	management::v1::{
		import_human_user_request::{Email, HashedPassword, Idp, Phone, Profile},
		ImportHumanUserRequest,
	},
	user::v1::Gender,
};
use zitadel::{
	api::zitadel::{
		admin::v1::{
			admin_service_client::AdminServiceClient, AddIamMemberRequest, AddLdapProviderRequest,
			GetOrgByIdRequest, ListEventsRequest, ListOrgsRequest, ListOrgsResponse,
		},
		auth::v1::{auth_service_client::AuthServiceClient, GetMyUserRequest},
		management::v1::{
			bulk_set_org_metadata_request::Metadata,
			management_service_client::ManagementServiceClient, AddProjectMemberRequest,
			AddProjectRequest, AddProjectRoleRequest, AddUserGrantRequest,
			BulkAddProjectRolesRequest, BulkAddProjectRolesResponse, BulkSetOrgMetadataRequest,
			GetMyOrgRequest, GetOrgMetadataRequest, GetUserByIdRequest,
			GetUserByLoginNameGlobalRequest, GetUserMetadataRequest, ListOrgMetadataRequest,
			ListOrgMetadataResponse, ListProjectRolesRequest, ListProjectRolesResponse,
			ListUserGrantRequest, ListUserGrantResponse, RemoveOrgMetadataRequest,
			RemoveUserGrantRequest, RemoveUserRequest, SetOrgMetadataRequest,
			SetUserMetadataRequest, UpdateHumanEmailRequest, UpdateHumanEmailResponse,
			UpdateHumanProfileRequest, UpdateHumanProfileResponse, UpdateProjectRequest,
			UpdateUserGrantRequest, UpdateUserNameRequest, UpdateUserNameResponse,
		},
		metadata::v1::{metadata_query::Query, MetadataKeyQuery, MetadataQuery},
		org::v1::{Org, OrgFieldName, OrgQuery},
		project::v1::PrivateLabelingSetting,
		user::v1::{User, UserGrantQuery},
		v1::ListQuery,
	},
	credentials::{AuthenticationOptions, ServiceAccount},
};

/// Metadata/Header for Zitadel organization ID, used to set/get metadata for
/// organizations.
const HEADER_ZITADEL_ORGANIZATION_ID: &str = "x-zitadel-orgid";
const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

/// Zitadel clients and functionality
#[derive(Debug, Clone)]
pub struct Zitadel {
	/// The audience to use for authentication.
	audience: String,
	/// The service account to use for authentication.
	service_account: ServiceAccount,
	/// The authentication options.
	auth_options: AuthenticationOptions,

	/// Token and expiry state
	token: Arc<RwLock<Token>>,

	/// Client for the [Admin API](https://zitadel.com/docs/apis/admin)
	admin_client: AdminServiceClient<Channel>,
	/// Client for the [Management API](https://zitadel.com/docs/apis/mgmt)
	management_client: ManagementServiceClient<Channel>,
	/// Client for the [Authentication API](https://zitadel.com/docs/apis/resources/auth)
	auth_client: AuthServiceClient<Channel>,
}

/// Token and expiry state
#[derive(Debug, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
struct Token {
	token: String,
	expiry: time::OffsetDateTime,
}

/// Create a new tonic channel with specified endpoint. Uses http proxy if
/// able.
async fn get_channel(api_endpoint: &str) -> Result<Channel> {
	let proxy = system_proxy_connector()?;

	let endpoint = Endpoint::from_shared(api_endpoint.to_owned())?
		.timeout(DEFAULT_TIMEOUT)
		.connect_timeout(DEFAULT_TIMEOUT);

	match proxy {
		Some(c) => endpoint.connect_with_connector(c).await,
		None => endpoint.connect().await,
	}
	.map_err(Into::into)
}

/// Make a [`hyper_proxy::Proxy`] for the given `url` with a specified
/// [`hyper_proxy::Intercept`], optionally using a [`proxyvars::NoProxy`],
/// and set up authorization if present in the `url`.
fn make_proxy(
	proxy_url: &str,
	no_proxy: Option<&std::sync::Arc<proxyvars::NoProxy>>,
	intercept: hyper_proxy::Intercept,
) -> Result<hyper_proxy::Proxy> {
	let uri = proxy_url.parse::<hyper::Uri>()?;
	let intercept = no_proxy
		.map_or(intercept.clone(), |np| wrap_intercept_with_no_proxy(intercept, np.clone()));
	let mut proxy = hyper_proxy::Proxy::new(intercept, uri);
	let url = proxy_url.parse::<url::Url>()?;
	if let Some(authorization) = get_authorization(&url) {
		proxy.set_authorization(authorization);
	}
	Ok(proxy)
}

/// Extract basic auth from a [url::Url] and return a
/// [headers::authorization::Basic] if credentials are present.
///
/// Note: Using [url::Url] because [hyper::Uri] does not support getting
/// username and password
fn get_authorization(
	url: &url::Url,
) -> Option<headers::Authorization<headers::authorization::Basic>> {
	url.password().and_then(|password| {
		(!url.username().is_empty())
			.then_some(headers::Authorization::basic(url.username(), password))
	})
}

/// Create a [hyper_proxy::ProxyConnector] configured by the env
/// variables `HTTP_PROXY,` `HTTPS_PROXY`, and `NO_PROXY`
fn system_proxy_connector(
) -> Result<Option<hyper_proxy::ProxyConnector<hyper::client::HttpConnector>>> {
	let http_proxy = proxyvars::http();
	let https_proxy = proxyvars::https();
	if let (None, None) = (&http_proxy, &https_proxy) {
		return Ok(None);
	}
	let no_proxy = proxyvars::no_proxy().map(std::sync::Arc::new);

	let http = hyper::client::HttpConnector::new();
	let mut connector = hyper_proxy::ProxyConnector::new(http)?;

	if let Some(http_proxy) = http_proxy {
		let proxy = make_proxy(&http_proxy, no_proxy.as_ref(), hyper_proxy::Intercept::Http)?;
		tracing::debug!("Using HTTP_PROXY");
		connector.add_proxy(proxy);
	}
	if let Some(https_proxy) = https_proxy {
		let proxy = make_proxy(&https_proxy, no_proxy.as_ref(), hyper_proxy::Intercept::Https)?;
		tracing::debug!("Using HTTPS_PROXY");
		connector.add_proxy(proxy);
	}

	Ok(Some(connector))
}

/// Create a `Custom` [`hyper_proxy::Intercept`] that first checks against
/// the passed [proxyvars::NoProxy], then uses the
/// [`hyper_proxy::Intercept`] as usual.
fn wrap_intercept_with_no_proxy(
	intercept: hyper_proxy::Intercept,
	no_proxy: std::sync::Arc<proxyvars::NoProxy>,
) -> hyper_proxy::Intercept {
	(move |scheme: Option<&str>, host: Option<&str>, port: Option<u16>| {
		let uri_string = format!(
			"{}{}{}",
			scheme.map(|s| format!("{}://", s)).unwrap_or_default(),
			host.unwrap_or_default(),
			port.map(|p| format!(":{}", p)).unwrap_or_default()
		);
		let uri: hyper::Uri = match uri_string.parse() {
			Ok(uri) => uri,
			Err(err) => {
				tracing::warn!("Failed to parse destination uri {uri_string}: {err}");
				return false;
			}
		};

		if no_proxy.matches_uri(&uri) {
			tracing::debug!("NO_PROXY matches URI {uri}, not proxying");
			return false;
		}

		if intercept.matches(&uri) {
			tracing::debug!("Proxying {intercept:?} connection with destination URI {uri}");
			true
		} else {
			false
		}
	})
	.into()
}

impl Zitadel {
	/// Builds a new Zitadel instance.
	/// - `url` should point to the Zitadel instance the client is for
	/// - `service_account_file` should be the Zitadel-generated
	///   private key file as documented here:
	///   https://zitadel.com/docs/guides/integrate/service-users/private-key-jwt#2-generate-a-private-key-file
	#[tracing::instrument(level = "debug", skip_all)]
	pub async fn new(url: Url, service_account_file: PathBuf) -> Result<Self> {
		// Zitadel matches this against the OIDC issuer, which is set
		// to not have a trailing slash
		let audience = url.as_str().trim_end_matches('/');

		// Wait for Zitadel instance to become ready.
		/*
			tracing::info!("Waiting for Zitadel instance to become ready.");
			wait_for_successful_response(format!("{}/debug/ready", config.zitadel.url).as_ref())
				.await
				.change_context(Error::Zitadel)?;
		*/
		let service_account = ServiceAccount::load_from_json(
			std::fs::read_to_string(&service_account_file)?.as_ref(),
		)?;
		let auth_options = AuthenticationOptions { api_access: true, ..Default::default() };

		let token = Arc::new(RwLock::new(Token {
			token: service_account.authenticate_with_options(audience, &auth_options).await?,
			expiry: time::OffsetDateTime::now_utc() + time::Duration::minutes(59),
		}));

		let channel = get_channel(url.as_str()).await?;
		let admin_client = AdminServiceClient::new(channel);

		let channel = get_channel(url.as_str()).await?;
		let management_client = ManagementServiceClient::new(channel);

		let channel = get_channel(url.as_str()).await?;
		let auth_client = AuthServiceClient::new(channel);

		Ok(Zitadel {
			auth_client,
			admin_client,
			management_client,
			audience: audience.to_owned(),
			service_account,
			auth_options,
			token,
		})
	}

	/// Add authorization token to metadata. If token expired, refresh it
	pub async fn request_with_auth<R: Send, IR: tonic::IntoRequest<R> + Send>(
		&self,
		request: IR,
	) -> Result<Request<R>> {
		let mut request = request.into_request();
		let meta = request.metadata_mut();
		{
			let token = self.token.read().await;
			if check_token_not_expired_then_set_authorization(&token.token, &token.expiry, meta)? {
				return Ok(request);
			}
		}
		let mut token = self.token.write().await;
		if check_token_not_expired_then_set_authorization(&token.token, &token.expiry, meta)? {
			return Ok(request);
		}

		token.token = tokio::time::timeout(
			DEFAULT_TIMEOUT,
			self.service_account.authenticate_with_options(&self.audience, &self.auth_options),
		)
		.await??;
		token.expiry = time::OffsetDateTime::now_utc() + time::Duration::minutes(59);

		insert_auth_token(&token.token, meta)?;
		request.set_timeout(DEFAULT_TIMEOUT);

		Ok(request)
	}

	/// Get my organization.
	/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-get-my-org)
	#[tracing::instrument(level = "debug", skip_all)]
	pub async fn get_my_organization(&self) -> Result<Option<Org>> {
		Ok(self
			.management_client
			.clone()
			.get_my_org(self.request_with_auth(GetMyOrgRequest {}).await?)
			.await?
			.into_inner()
			.org)
	}
	/*

	  /// Get the User info of the user making the request, generally a service
	  /// account.
	  /// [API Docs](https://zitadel.com/docs/apis/resources/auth/auth-service-get-my-user)
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn get_my_user(&self) -> Result<User, Report<Error>> {
		  self.auth_client
			  .clone()
			  .get_my_user(self.request_with_auth(GetMyUserRequest {}).await?)
			  .await
			  .change_context(Error::Zitadel)?
			  .into_inner()
			  .user
			  .ok_or(report!(Error::Zitadel))
	  }

	  /// Get the User ID of the user making the request, generally a service
	  /// account.
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn get_my_user_id(&self) -> Result<String, Report<Error>> {
		  self.get_my_user().await.map(|user| user.id)
	  }

	  /// List events.
	  ///[API Docs](https://zitadel.com/docs/apis/resources/admin/admin-service-list-events)
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn list_events(
		  &self,
		  request: ListEventsRequest,
	  ) -> Result<Vec<Event>, Report<Error>> {
		  let response = self
			  .admin_client
			  .clone()
			  .list_events(self.request_with_auth(request).await?)
			  .await
			  .change_context(Error::Zitadel)?;
		  tracing::trace!("{response:?}");
		  response.into_inner().events.into_iter().map(TryInto::try_into).collect()
	  }

	  /// Set org metadata.
	  /// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-set-org-metadata)
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn set_org_metadata(
		  &self,
		  organization_id: Option<&str>,
		  key: String,
		  value: &str,
	  ) -> Result<(), Report<Error>> {
		  tracing::debug!("Setting organisation metadata: ({key}, {value})");
		  let value = value.as_bytes().to_vec();

		  let mut request = tonic::Request::new(SetOrgMetadataRequest { key, value });
		  if let Some(org_id) = organization_id {
			  request.metadata_mut().insert(
				  HEADER_ZITADEL_ORGANIZATION_ID,
				  org_id.parse::<AsciiMetadataValue>().change_context(Error::Zitadel)?,
			  );
		  }

		  let response = self
			  .management_client
			  .clone()
			  .set_org_metadata(self.request_with_auth(request).await?)
			  .await
			  .change_context(Error::Zitadel)?;
		  tracing::trace!("Response: {:#?}", response.into_inner());

		  Ok(())
	  }

	  /// Remove org metadata.
	  /// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-remove-org-metadata)
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn remove_org_metadata(
		  &self,
		  organization_id: Option<&str>,
		  key: String,
	  ) -> Result<(), Report<Error>> {
		  tracing::debug!("Deleting organisation metadata: {key}");
		  let mut request = tonic::Request::new(RemoveOrgMetadataRequest { key });
		  if let Some(org_id) = organization_id {
			  request.metadata_mut().insert(
				  HEADER_ZITADEL_ORGANIZATION_ID,
				  org_id.parse::<AsciiMetadataValue>().change_context(Error::Zitadel)?,
			  );
		  }

		  let response = self
			  .management_client
			  .clone()
			  .remove_org_metadata(self.request_with_auth(request).await?)
			  .await
			  .change_context(Error::Zitadel)?;
		  tracing::trace!("Response: {:#?}", response.into_inner());

		  Ok(())
	  }

	  /// Get organization by id.
	  /// [API Docs](https://zitadel.com/docs/apis/resources/admin/admin-service-get-org-by-id)
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn get_org_by_id(&self, org_id: String) -> Result<Option<Org>, Report<Error>> {
		  tracing::debug!("Getting organization by id '{org_id}'");
		  let request = tonic::Request::new(GetOrgByIdRequest { id: org_id });

		  let response = self
			  .admin_client
			  .clone()
			  .get_org_by_id(self.request_with_auth(request).await?)
			  .await
			  .change_context(Error::Zitadel)?;
		  tracing::trace!("Response: {:#?}", &response);

		  Ok(response.into_inner().org)
	  }

	  /// Search organization.
	  /// [API Docs](https://zitadel.com/docs/apis/resources/admin/admin-service-list-orgs#search-organization)
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn search_organization(
		  &self,
		  query_parameters: Option<ListQuery>,
		  queries: Vec<OrgQuery>,
	  ) -> Result<ListOrgsResponse, Report<Error>> {
		  tracing::debug!("Searching organisations '{queries:?}'");
		  let request = tonic::Request::new(ListOrgsRequest {
			  queries,
			  query: query_parameters,
			  sorting_column: OrgFieldName::Unspecified.into(),
		  });

		  let response = self
			  .admin_client
			  .clone()
			  .list_orgs(self.request_with_auth(request).await?)
			  .await
			  .change_context(Error::Zitadel)?;
		  tracing::trace!("Response: {:#?}", &response);

		  Ok(response.into_inner())
	  }

	  /// Search org metadata.
	  /// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-org-metadata)
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn search_org_metadata(
		  &self,
		  organization_id: Option<&str>,
		  query_parameters: Option<ListQuery>,
		  queries: Vec<MetadataKeyQuery>,
	  ) -> Result<ListOrgMetadataResponse, Report<Error>> {
		  tracing::debug!("Searching organisation metadata '{queries:?}'");
		  let queries = queries
			  .into_iter()
			  .map(|q| MetadataQuery { query: Some(Query::KeyQuery(q)) })
			  .collect();
		  let mut request =
			  tonic::Request::new(ListOrgMetadataRequest { queries, query: query_parameters });
		  if let Some(org_id) = organization_id {
			  request.metadata_mut().insert(
				  HEADER_ZITADEL_ORGANIZATION_ID,
				  org_id.parse::<AsciiMetadataValue>().change_context(Error::Zitadel)?,
			  );
		  }

		  let response = self
			  .management_client
			  .clone()
			  .list_org_metadata(self.request_with_auth(request).await?)
			  .await
			  .change_context(Error::Zitadel)?;
		  tracing::trace!("Response: {:#?}", &response);

		  Ok(response.into_inner())
	  }

	  /// Get org metadata for a key. Also decodes the value to a string.
	  /// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-get-org-metadata)
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn get_org_metadata(
		  &self,
		  organization_id: Option<&str>,
		  key: String,
	  ) -> Result<Option<String>, Report<Error>> {
		  let mut request = tonic::Request::new(GetOrgMetadataRequest { key: key.clone() });
		  if let Some(org_id) = organization_id {
			  request.metadata_mut().insert(
				  HEADER_ZITADEL_ORGANIZATION_ID,
				  org_id.parse::<AsciiMetadataValue>().change_context(Error::Zitadel)?,
			  );
		  }

		  let response = self
			  .management_client
			  .clone()
			  .get_org_metadata(self.request_with_auth(request).await?)
			  .await
			  .change_context(Error::Zitadel);
		  tracing::trace!("Got organization metadata response: {response:#?}");

		  let response = match response {
			  Ok(resp) => resp,
			  Err(e) => match e.downcast_ref::<Status>() {
				  // `QUERY-Rph32` is the grpc message returned by Zitadel when no metadata was found
				  // for the given key
				  Some(status)
					  if status.code() == tonic::Code::NotFound
						  && status.message().contains("QUERY-Rph32") =>
				  {
					  tracing::debug!("No org metadata found for key '{key}'. Organization ID: {organization_id:?}");
					  return Ok(None);
				  }
				  _ => {
					  return Err(e).attach_printable(format!(
					  "Error fetching metadata for key '{key}'. Organization ID: {organization_id:?}"
				  ))
				  }
			  },
		  };
		  let Some(metadata) = response.into_inner().metadata else {
			  return Err(Error::Zitadel).attach_printable(format!(
				  "No Metadata found in MetadataResponse for key '{key}'"
			  ));
		  };
		  let value = String::from_utf8(metadata.value)
			  .change_context(Error::Zitadel)
			  .attach_printable(format!("Unable to parse metadata for key '{key}'"))?;
		  tracing::debug!("Got organisation metadata for key '{key}': {value:?}");
		  Ok(Some(value))
	  }
	*/

	/// Set user metadata.
	/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-set-user-metadata)
	#[tracing::instrument(level = "debug", skip_all)]
	pub async fn set_user_metadata(
		&self,
		organization_id: Option<&str>,
		user_id: String,
		key: String,
		value: &str,
	) -> Result<()> {
		tracing::debug!("Setting metadata: for user ID {user_id} ({key}, {value})");
		let value = value.as_bytes().to_vec();

		let mut request = tonic::Request::new(SetUserMetadataRequest { id: user_id, key, value });
		if let Some(org_id) = organization_id {
			request
				.metadata_mut()
				.insert(HEADER_ZITADEL_ORGANIZATION_ID, org_id.parse::<AsciiMetadataValue>()?);
		}

		let response = self
			.management_client
			.clone()
			.set_user_metadata(self.request_with_auth(request).await?)
			.await?;
		tracing::trace!("Set user metadata response: {:#?}", response.into_inner());

		Ok(())
	}

	/*
	  /// Get user metadata for a key. Also decodes the value to a string.
	  /// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-get-user-metadata)
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn get_user_metadata(
		  &self,
		  organization_id: Option<String>,
		  user_id: &str,
		  key: &str,
	  ) -> Result<Option<String>> {
		  let mut request = tonic::Request::new(GetUserMetadataRequest {
			  id: user_id.to_owned(),
			  key: key.to_owned(),
		  });
		  if let Some(ref org_id) = organization_id {
			  request.metadata_mut().insert(
				  HEADER_ZITADEL_ORGANIZATION_ID,
				  org_id.parse::<AsciiMetadataValue>()?,
			  );
		  }

		  let response = self
			  .management_client
			  .clone()
			  .get_user_metadata(self.request_with_auth(request).await?)
			  .await
			  .change_context(Error::Zitadel);
		  let response = match response {
			  Ok(resp) => resp,
			  Err(e) => match e.downcast_ref::<Status>() {
				  // `QUERY-Rgh32` is the grpc message returned by Zitadel when no metadata was
				  // found for the given key
				  Some(status)
					  if status.code() == tonic::Code::NotFound
						  && status.message().contains("QUERY-Rgh32") =>
				  {
					  tracing::debug!("No user '{user_id}' metadata found for key '{key}'. Organization ID: {organization_id:?}");
					  return Ok(None);
				  }
				  _ => {
					  return Err(e.into()); // .attach_printable(format!(
						  // "Error fetching user '{user_id}' metadata for key '{key}'. Organization ID: {organization_id:?}"))
				  }
			  },
		  };

		  tracing::trace!("Got user metadata response: {response:#?}");
		  let Some(metadata) = response.into_inner().metadata else {
			  return Err(Error::Zitadel).attach_printable(format!(
				  "No Metadata found in MetadataResponse for key '{key}'"
			  ));
		  };
		  let value =
			  String::from_utf8(metadata.value)?; change_context(Error::Zitadel).attach_printable(
				  // format!("Unable to parse user '{user_id}' metadata for key '{key}'."),
			  //)?;
		  tracing::debug!("Got user metadata for user '{user_id}' key '{key}': {value:?}");
		  Ok(Some(value))
	  }
	*/

	/*
	  /// Set organization metadata in bulk.
	  /// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-bulk-set-org-metadata)
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn set_org_metadata_bulk(
		  &self,
		  organization_id: Option<&str>,
		  metadata: Vec<(String, String)>,
	  ) -> Result<(), Report<Error>> {
		  let metadata: Vec<Metadata> = metadata
			  .into_iter()
			  .map(|(key, value)| Metadata { key, value: value.into_bytes() })
			  .collect();

		  let mut request = tonic::Request::new(BulkSetOrgMetadataRequest { metadata });
		  if let Some(org_id) = organization_id {
			  request.metadata_mut().insert(
				  HEADER_ZITADEL_ORGANIZATION_ID,
				  org_id.parse::<AsciiMetadataValue>().change_context(Error::Zitadel)?,
			  );
		  }

		  let _response = self
			  .management_client
			  .clone()
			  .bulk_set_org_metadata(self.request_with_auth(request).await?)
			  .await
			  .change_context(Error::Zitadel)?;

		  Ok(())
	  }

	  /// Get `dns_domain` for a given project ID
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn get_project_dns_domain(
		  &self,
		  organization_id: Option<&str>,
		  project: &str,
	  ) -> Result<Option<String>, Report<Error>> {
		  self.get_org_metadata(organization_id, format!("{project}.dns_domain")).await
	  }

	  /// Set `dns_domain` for a given project ID
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn set_project_dns_domain(
		  &self,
		  organization_id: Option<&str>,
		  project: &str,
		  dns_domain: &str,
	  ) -> Result<(), Report<Error>> {
		  self.set_org_metadata(organization_id, format!("{project}.dns_domain"), dns_domain).await
	  }

	  /// Get `name` for a given project ID
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn get_project_name(
		  &self,
		  organization_id: Option<&str>,
		  project: &str,
	  ) -> Result<Option<String>, Report<Error>> {
		  self.get_org_metadata(organization_id, format!("{project}.name")).await
	  }

	  /// Set `name` for a given project ID
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn set_project_name(
		  &self,
		  organization_id: Option<&str>,
		  project: &str,
		  name: &str,
	  ) -> Result<(), Report<Error>> {
		  self.set_org_metadata(organization_id, format!("{project}.name"), name).await
	  }

	  /// Get my organization ID.
	  #[tracing::instrument(level = "debug", skip_all)]
	  pub async fn get_my_organization_id(&self) -> Result<String, Report<Error>> {
		  self.get_my_organization().await.map(|org| org.id)
	  }
	*/

	/// Create a human user, returning the User ID.
	/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-import-human-user)
	#[tracing::instrument(level = "debug", skip_all)]
	pub async fn create_human_user(
		&self,
		organization_id: &str,
		request: ImportHumanUserRequest,
	) -> Result<String> {
		let mut request_with_org = tonic::Request::new(request);
		request_with_org
			.metadata_mut()
			.insert(HEADER_ZITADEL_ORGANIZATION_ID, organization_id.parse::<AsciiMetadataValue>()?);

		Ok(self
			.management_client
			.clone()
			.import_human_user(self.request_with_auth(request_with_org).await?)
			.await?
			.into_inner()
			.user_id)
	}

	/*
		/// Update a human user, returning the update details.
		/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-update-human-profile?#update-user-profile-human)
		#[tracing::instrument(level = "debug", skip_all)]
		pub async fn update_human_user_profile(
			&self,
			organization_id: &str,
			request: UpdateHumanProfileRequest,
		) -> Result<UpdateHumanProfileResponse, Report<Error>> {
			let mut request_with_org = tonic::Request::new(request);
			request_with_org.metadata_mut().insert(
				HEADER_ZITADEL_ORGANIZATION_ID,
				organization_id.parse::<AsciiMetadataValue>().change_context(Error::Zitadel)?,
			);
			Ok(self
				.management_client
				.clone()
				.update_human_profile(self.request_with_auth(request_with_org).await?)
				.await
				.change_context(Error::Zitadel)?
				.into_inner())
		}

		/// Update a human user email, returning the update details.
		/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-update-human-email)
		#[tracing::instrument(level = "debug", skip_all)]
		pub async fn update_human_user_email(
			&self,
			organization_id: &str,
			request: UpdateHumanEmailRequest,
		) -> Result<UpdateHumanEmailResponse, Report<Error>> {
			let mut request_with_org = tonic::Request::new(request);
			request_with_org.metadata_mut().insert(
				HEADER_ZITADEL_ORGANIZATION_ID,
				organization_id.parse::<AsciiMetadataValue>().change_context(Error::Zitadel)?,
			);
			Ok(self
				.management_client
				.clone()
				.update_human_email(self.request_with_auth(request_with_org).await?)
				.await
				.change_context(Error::Zitadel)?
				.into_inner())
		}

		/// Update a human user username, returning the update details.
		/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-update-user-name)
		#[tracing::instrument(level = "debug", skip_all)]
		pub async fn update_human_user_name(
			&self,
			organization_id: &str,
			request: UpdateUserNameRequest,
		) -> Result<UpdateUserNameResponse, Report<Error>> {
			let mut request_with_org = tonic::Request::new(request);
			request_with_org.metadata_mut().insert(
				HEADER_ZITADEL_ORGANIZATION_ID,
				organization_id.parse::<AsciiMetadataValue>().change_context(Error::Zitadel)?,
			);
			Ok(self
				.management_client
				.clone()
				.update_user_name(self.request_with_auth(request_with_org).await?)
				.await
				.change_context(Error::Zitadel)?
				.into_inner())
		}
	*/

	/// Get user information for the given user ID.
	/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-get-user-by-id)
	#[tracing::instrument(level = "debug", skip_all)]
	pub async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>> {
		Ok(self
			.management_client
			.clone()
			.get_user_by_id(
				self.request_with_auth(GetUserByIdRequest { id: user_id.to_owned() }).await?,
			)
			.await?
			.into_inner()
			.user)
	}

	/// Remove the user with the given user ID.
	/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-remove-user)
	#[tracing::instrument(level = "debug", skip_all)]
	pub async fn remove_user(&self, user_id: String) -> Result<()> {
		self.management_client
			.clone()
			.remove_user(self.request_with_auth(RemoveUserRequest { id: user_id }).await?)
			.await?;

		Ok(())
	}

	/*
		/// Add a user to a project with one or more roles.
		/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-project-member)
		#[tracing::instrument(level = "debug", skip_all)]
		pub async fn add_project_member(
			&self,
			organization_id: &str,
			request: AddProjectMemberRequest,
		) -> Result<(), Report<Error>> {
			let mut request_with_org = tonic::Request::new(request);
			request_with_org.metadata_mut().insert(
				HEADER_ZITADEL_ORGANIZATION_ID,
				organization_id.parse::<AsciiMetadataValue>().change_context(Error::Zitadel)?,
			);
			self.management_client
				.clone()
				.add_project_member(self.request_with_auth(request_with_org).await?)
				.await
				.change_context(Error::Zitadel)?;

			Ok(())
		}

		/// This request adds a new user to the IAM members list with one or
		/// multiple roles. level with one or multiple roles. [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-org-member)
		#[tracing::instrument(level = "debug", skip_all)]
		pub async fn add_iam_member(&self, request: AddIamMemberRequest) -> Result<(), Report<Error>> {
			self.admin_client
				.clone()
				.add_iam_member(self.request_with_auth(request).await?)
				.await
				.change_context(Error::Zitadel)?;

			Ok(())
		}
	*/

	/// Get a user by login name searched over all organizations. The request
	/// only returns data if the login name matches exactly. [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-get-user-by-login-name-global)
	#[tracing::instrument(level = "debug", skip_all)]
	pub async fn get_user_by_login_name(&self, login_name: &str) -> Result<Option<User>> {
		let user = self
			.management_client
			.clone()
			.get_user_by_login_name_global(
				self.request_with_auth(GetUserByLoginNameGlobalRequest {
					login_name: login_name.to_owned(),
				})
				.await?,
			)
			.await?;

		Ok(user.into_inner().user)
	}

	/// Add a User grant, returning the grant ID.
	/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-user-grant)
	#[tracing::instrument(level = "debug", skip_all)]
	pub async fn add_user_grant(
		&self,
		organization_id: Option<String>,
		user_id: String,
		project_id: String,
		project_grant_id: Option<String>,
		role_keys: Vec<String>,
	) -> Result<String> {
		let add_user_grant_request = AddUserGrantRequest {
			user_id,
			project_id,
			project_grant_id: project_grant_id.unwrap_or_default(),
			role_keys,
		};
		let mut request_with_org = tonic::Request::new(add_user_grant_request);
		request_with_org.metadata_mut().insert(
			HEADER_ZITADEL_ORGANIZATION_ID,
			organization_id.unwrap_or_default().parse::<AsciiMetadataValue>()?,
		);
		Ok(self
			.management_client
			.clone()
			.add_user_grant(self.request_with_auth(request_with_org).await?)
			.await?
			.into_inner()
			.user_grant_id)
	}

	/*
		/// Update a User grant.
		/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-update-user-grant)
		#[tracing::instrument(level = "debug", skip_all)]
		pub async fn update_user_grant(
			&self,
			user_id: String,
			grant_id: String,
			role_keys: Vec<String>,
		) -> Result<(), Report<Error>> {
			self.management_client
				.clone()
				.update_user_grant(
					self.request_with_auth(UpdateUserGrantRequest { user_id, grant_id, role_keys })
						.await?,
				)
				.await
				.change_context(Error::Zitadel)?;
			Ok(())
		}

		/// Remove a User grant.
		/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-remove-user-grant)
		#[tracing::instrument(level = "debug", skip_all)]
		pub async fn remove_user_grant(
			&self,
			user_id: String,
			grant_id: String,
		) -> Result<(), Report<Error>> {
			self.management_client
				.clone()
				.remove_user_grant(
					self.request_with_auth(RemoveUserGrantRequest { user_id, grant_id }).await?,
				)
				.await
				.change_context(Error::Zitadel)?;
			Ok(())
		}

		/// Searches User grants. Returns a list of user grants that match the
		/// search queries. User grants are the roles users have for a specific
		/// project and organization. [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-user-grants)
		#[tracing::instrument(level = "debug", skip_all)]
		pub async fn search_user_grants(
			&self,
			organization_id: &str,
			queries: Vec<UserGrantQuery>,
		) -> Result<ListUserGrantResponse, Report<Error>> {
			let request = ListUserGrantRequest { queries, ..Default::default() };
			let mut request_with_org = tonic::Request::new(request);
			request_with_org.metadata_mut().insert(
				HEADER_ZITADEL_ORGANIZATION_ID,
				organization_id.parse::<AsciiMetadataValue>().change_context(Error::Zitadel)?,
			);
			Ok(self
				.management_client
				.clone()
				.list_user_grants(self.request_with_auth(request_with_org).await?)
				.await
				.change_context(Error::Zitadel)?
				.into_inner())
		}

		/// Searches Project Roles. Returns all roles of a project matching the
		/// search query. [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-project-roles)
		#[tracing::instrument(level = "debug", skip_all)]
		pub async fn search_project_roles(
			&self,
			organization_id: String,
			request: ListProjectRolesRequest,
		) -> Result<ListProjectRolesResponse, Report<Error>> {
			let mut request_with_org = tonic::Request::new(request);
			request_with_org.metadata_mut().insert(
				HEADER_ZITADEL_ORGANIZATION_ID,
				organization_id.parse::<AsciiMetadataValue>().change_context(Error::Zitadel)?,
			);
			let response = self
				.management_client
				.clone()
				.list_project_roles(self.request_with_auth(request_with_org).await?)
				.await
				.change_context(Error::Zitadel)?
				.into_inner();
			Ok(response)
		}
	*/

	/// Create a project, returning the project ID.
	/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-project)
	#[tracing::instrument(level = "debug", skip_all)]
	pub async fn add_project(
		&self,
		name: String,
		project_role_assertion: bool,
		project_role_check: bool,
		has_project_check: bool,
		private_labeling_setting: PrivateLabelingSetting,
	) -> Result<String> {
		Ok(self
			.management_client
			.clone()
			.add_project(
				self.request_with_auth(AddProjectRequest {
					name,
					project_role_assertion,
					project_role_check,
					has_project_check,
					private_labeling_setting: private_labeling_setting as i32,
				})
				.await?,
			)
			.await?
			.into_inner()
			.id)
	}

	/// Update a project.
	/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-update-project)
	#[tracing::instrument(level = "debug", skip_all)]
	pub async fn update_project(
		&self,
		id: String,
		name: String,
		project_role_assertion: bool,
		project_role_check: bool,
		has_project_check: bool,
		private_labeling_setting: PrivateLabelingSetting,
	) -> Result<()> {
		self.management_client
			.clone()
			.update_project(
				self.request_with_auth(UpdateProjectRequest {
					id,
					name,
					project_role_assertion,
					project_role_check,
					has_project_check,
					private_labeling_setting: private_labeling_setting as i32,
				})
				.await?,
			)
			.await?;
		Ok(())
	}

	/// Add a role to a project.
	/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-project-role)
	#[tracing::instrument(level = "debug", skip_all)]
	pub async fn add_project_role(
		&self,
		project_id: String,
		role_key: String,
		display_name: String,
		group: Option<String>,
	) -> Result<()> {
		self.management_client
			.clone()
			.add_project_role(
				self.request_with_auth(AddProjectRoleRequest {
					project_id,
					role_key,
					display_name,
					group: group.unwrap_or_default(),
				})
				.await?,
			)
			.await?;
		Ok(())
	}

	/// Add roles to a project in bulk.
	/// [API Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-bulk-add-project-roles)
	#[tracing::instrument(level = "debug", skip_all)]
	pub async fn add_project_roles_bulk(
		&self,
		organization_id: String,
		request: BulkAddProjectRolesRequest,
	) -> Result<BulkAddProjectRolesResponse> {
		let mut request_with_org = tonic::Request::new(request);
		request_with_org
			.metadata_mut()
			.insert(HEADER_ZITADEL_ORGANIZATION_ID, organization_id.parse::<AsciiMetadataValue>()?);
		let response = self
			.management_client
			.clone()
			.bulk_add_project_roles(self.request_with_auth(request_with_org).await?)
			.await?;
		Ok(response.into_inner())
	}

	/// Add a LDAP IDP
	/// [API Docs](https://zitadel.com/docs/apis/resources/admin/admin-service-add-ldap-provider)
	#[tracing::instrument(level = "debug", skip_all)]
	pub async fn add_ldap_idp(&self, request: AddLdapProviderRequest) -> Result<String> {
		let response = self
			.admin_client
			.clone()
			.add_ldap_provider(self.request_with_auth(request).await?)
			.await?;

		Ok(response.into_inner().id)
	}
}

/// Check if the token is still valid and set the authorization header if it is.
fn check_token_not_expired_then_set_authorization(
	token: &str,
	expiry: &time::OffsetDateTime,
	meta: &mut tonic::metadata::MetadataMap,
) -> Result<bool> {
	if expiry > &time::OffsetDateTime::now_utc() {
		insert_auth_token(token, meta)?;
		return Ok(true);
	}
	Ok(false)
}

#[doc(hidden)]
fn insert_auth_token(token: &str, meta: &mut tonic::metadata::MetadataMap) -> Result<()> {
	meta.insert("authorization", format!("Bearer {}", token).parse()?);
	Ok(())
}
