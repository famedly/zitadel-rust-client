//! Implementation of the client for the Zitadel Management api
mod models;

use anyhow::{Context, Result};
use famedly_rust_utils::GenericCombinators;
use futures::Stream;
pub use models::*;
use reqwest::header::HeaderValue;

use super::{
	pagination::{PaginationHandler, PaginationParams},
	Zitadel, HEADER_ZITADEL_ORGANIZATION_ID,
};

impl Zitadel {
	/// Create actions. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-create-action)
	pub async fn create_action(
		&self,
		body: V1CreateActionRequest,
		org_id: Option<String>,
	) -> Result<V1CreateActionResponse> {
		let request = self
			.client
			.post(self.make_url("management/v1/actions")?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(&body)
			.build()
			.context("Error building create_action request")?;

		self.send_request(request).await
	}

	/// Update action. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-update-action)
	pub async fn update_action(
		&self,
		action_id: String,
		body: ManagementServiceUpdateActionBody,
		org_id: Option<String>,
	) -> Result<V1UpdateActionResponse> {
		let request = self
			.client
			.put(self.make_url(&format!("management/v1/actions/{action_id}"))?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(&body)
			.build()
			.context("Error building update_action request")?;

		self.send_request(request).await
	}

	/// Delete action. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-delete-action)
	pub async fn delete_action(
		&self,
		action_id: String,
		org_id: Option<String>,
	) -> Result<V1DeleteActionResponse> {
		let request = self
			.client
			.delete(self.make_url(&format!("management/v1/actions/{action_id}"))?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(&ManagementServiceDeleteActionBody::new())
			.build()
			.context("Error building delete_action request")?;

		self.send_request(request).await
	}

	/// Search for actions. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-actions)
	pub fn list_actions(
		&self,
		org_id: Option<String>,
		params: Option<PaginationParams>,
		queries: Option<Vec<V1ActionQuery>>,
	) -> Result<impl Stream<Item = V1Action> + Send + Sync> {
		Ok(PaginationHandler::new(
			self.clone(),
			(params, queries),
			self.make_url("management/v1/actions/_search")?,
			org_id,
		))
	}

	/// Get a flow. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-get-flow)
	pub async fn get_flow(
		&self,
		flow_type: u32,
		org_id: Option<String>,
	) -> Result<V1GetFlowResponse> {
		let request = self
			.client
			.get(self.make_url(&format!("management/v1/flows/{flow_type}"))?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.build()
			.context("Error building get_flow request")?;

		self.send_request(request).await
	}

	/// Set trigger actions. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-set-trigger-actions)
	pub async fn set_trigger_actions(
		&self,
		// TODO: Should we provide enums for these?
		flow_type: u32,
		trigger_type: u32,
		body: ManagementServiceSetTriggerActionsBody,
		org_id: Option<String>,
	) -> Result<V1SetTriggerActionsResponse> {
		let request = self
			.client
			.post(
				self.make_url(&format!("management/v1/flows/{flow_type}/trigger/{trigger_type}"))?,
			)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(&body)
			.build()
			.context("Error building set_trigger_actions request")?;

		self.send_request(request).await
	}

	/// Create application. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-api-app)
	pub async fn create_application(
		&self,
		project_id: String,
		body: ManagementServiceAddApiAppBody,
		org_id: Option<String>,
	) -> Result<V1AddApiAppResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("management/v1/projects/{project_id}/apps/api"))?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(&body)
			.build()
			.context("Error building create_application request")?;

		self.send_request(request).await
	}

	/// Remove application. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-remove-app)
	pub async fn remove_application(
		&self,
		project_id: String,
		application_id: String,
		org_id: Option<String>,
	) -> Result<V1RemoveAppResponse> {
		let request =
			self.client
				.delete(self.make_url(&format!(
					"management/v1/projects/{project_id}/apps/{application_id}"
				))?)
				.header(
					HEADER_ZITADEL_ORGANIZATION_ID,
					HeaderValue::from_str(&org_id.unwrap_or_default())?,
				)
				.build()
				.context("Error building remove_application request")?;

		self.send_request(request).await
	}

	/// Search for applications [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-apps)
	pub fn list_applications(
		&self,
		project_id: String,
		org_id: Option<String>,
		params: Option<PaginationParams>,
		queries: Option<Vec<V1AppQuery>>,
	) -> Result<impl Stream<Item = V1App>> {
		Ok(PaginationHandler::new(
			self.clone(),
			(params, queries),
			self.make_url(&format!("management/v1/projects/{project_id}/apps/_search"))?,
			org_id,
		))
	}

	/// Create project. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-project)
	pub async fn create_project(
		&self,
		body: V1AddProjectRequest,
		org_id: Option<String>,
	) -> Result<V1AddProjectResponse> {
		let request = self
			.client
			.post(self.make_url("management/v1/projects")?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(&body)
			.build()
			.context("Error building create_project request")?;

		self.send_request(request).await
	}

	/// Remove project. [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-remove-project)
	pub async fn remove_project(
		&self,
		project_id: String,
		org_id: Option<String>,
	) -> Result<V1RemoveProjectResponse> {
		let request = self
			.client
			.delete(self.make_url(&format!("management/v1/projects/{project_id}"))?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.build()
			.context("Error building delete_project request")?;

		self.send_request(request).await
	}

	/// Search for projects [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-projects)
	pub fn list_projects(
		&self,
		org_id: Option<String>,
		params: Option<PaginationParams>,
		queries: Option<Vec<V1ProjectQuery>>,
	) -> Result<impl Stream<Item = V1Project>> {
		Ok(PaginationHandler::new(
			self.clone(),
			(params, queries),
			self.make_url("management/v1/projects/_search")?,
			org_id,
		))
	}

	/// List Project Members [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-project-members
	pub fn list_project_members(
		&self,
		org_id: Option<String>,
		project_id: &str,
		params: Option<PaginationParams>,
		queries: Option<Vec<Zitadelmemberv1SearchQuery>>,
	) -> Result<impl Stream<Item = V1Member>> {
		Ok(PaginationHandler::new(
			self.clone(),
			(params, queries),
			self.make_url(&format!("management/v1/projects/{project_id}/members/_search"))?,
			org_id,
		))
	}

	/// Add Project Grant [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-project-grant)
	pub async fn add_project_grant(
		&self,
		org_id: Option<String>,
		project_id: &str,
		granted_org_id: String,
		role_keys: Option<Vec<String>>,
	) -> Result<V1AddProjectGrantResponse> {
		let request = self
			.client
			.post(self.make_url(&format!("management/v1/projects/{project_id}/grants"))?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(
				&ManagementServiceAddProjectGrantBody::new()
					.with_granted_org_id(granted_org_id)
					.chain_opt(role_keys, ManagementServiceAddProjectGrantBody::with_role_keys),
			)
			.build()
			.context("Error building add_project_grant request")?;

		self.send_request(request).await
	}

	/// Add Project Member [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-project-member)
	pub async fn add_project_member(
		&self,
		org_id: Option<String>,
		project_id: &str,
		user_id: String,
		roles: Vec<String>,
	) -> Result<V1AddProjectMemberResponse> {
		type ReqBody = ManagementServiceAddProjectMemberBody;
		let request = self
			.client
			.post(self.make_url(&format!("management/v1/projects/{project_id}/members"))?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(&ReqBody::new().with_user_id(user_id).with_roles(roles))
			.build()
			.context("Error building add_project_member request")?;

		self.send_request(request).await
	}

	/// Add Organization Member [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-org-member)
	pub async fn add_organization_member(
		&self,
		org_id: Option<String>,
		user_id: String,
		roles: Vec<String>,
	) -> Result<V1AddOrgMemberResponse> {
		let request = self
			.client
			.post(self.make_url("management/v1/orgs/me/members")?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(&V1AddOrgMemberRequest::new().with_user_id(user_id).with_roles(roles))
			.build()
			.context("Error building add_organization_member request")?;

		self.send_request(request).await
	}

	/// List Organization Members [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-org-members)
	pub fn list_organization_members(
		&self,
		org_id: Option<String>,
		params: Option<PaginationParams>,
		queries: Option<Vec<Zitadelmemberv1SearchQuery>>,
	) -> Result<impl Stream<Item = V1Member>> {
		Ok(PaginationHandler::new(
			self.clone(),
			(params, queries),
			self.make_url("management/v1/orgs/me/members/_search")?,
			org_id,
		))
	}

	/// List Project Grant Members [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-project-grant-members)
	pub fn list_project_grant_members(
		&self,
		org_id: Option<String>,
		project_id: &str,
		grant_id: &str,
		params: Option<PaginationParams>,
		queries: Option<Vec<Zitadelmemberv1SearchQuery>>,
	) -> Result<impl Stream<Item = V1Member>> {
		Ok(PaginationHandler::new(
			self.clone(),
			(params, queries),
			self.make_url(&format!(
				"management/v1/projects/{project_id}/grants/{grant_id}/members/_search"
			))?,
			org_id,
		))
	}

	/// Add Project Grant Member [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-project-grant-member)
	pub async fn add_project_grant_member(
		&self,
		org_id: Option<String>,
		project_id: &str,
		project_grant_id: &str,
		user_id: String,
		roles: Vec<String>,
	) -> Result<V1AddProjectGrantMemberResponse> {
		//
		type ReqBody = ManagementServiceAddProjectGrantMemberBody;
		let request = self
			.client
			.post(self.make_url(&format!(
				"management/v1/projects/{project_id}/grants/{project_grant_id}/members"
			))?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(&ReqBody::new(user_id).with_roles(roles))
			.build()
			.context("Error building radd_project_grant_member equest")?;

		self.send_request(request).await
	}

	/// Add User Grant [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-user-grant)
	pub async fn add_user_grant(
		&self,
		org_id: Option<String>,
		user_id: &str,
		project_id: String,
		project_grant_id: Option<String>,
		role_keys: Option<Vec<String>>,
	) -> Result<V1AddUserGrantResponse> {
		type ReqBody = ManagementServiceAddUserGrantBody;
		let request = self
			.client
			.post(self.make_url(&format!("management/v1/users/{user_id}/grants"))?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(
				&ReqBody::new(project_id)
					.chain_opt(project_grant_id, ReqBody::with_project_grant_id)
					.chain_opt(role_keys, ReqBody::with_role_keys),
			)
			.build()
			.context("Error building add_user_grant request")?;

		self.send_request(request).await
	}

	/// Search User Grants [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-list-user-grants)
	pub fn search_user_grants(
		&self,
		org_id: Option<String>,
		params: Option<PaginationParams>,
		queries: Option<Vec<V1UserGrantQuery>>,
	) -> Result<impl Stream<Item = Zitadeluserv1UserGrant>> {
		Ok(PaginationHandler::new(
			self.clone(),
			(params, queries),
			self.make_url("management/v1/users/grants/_search")?,
			org_id,
		))
	}

	/// Add Project Role [Docs](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-project-role)
	pub async fn add_project_role(
		&self,
		org_id: Option<String>,
		project_id: &str,
		role_key: String,
		display_name: String,
		group: Option<String>,
	) -> Result<V1AddProjectRoleResponse> {
		type ReqBody = ManagementServiceAddProjectRoleBody;
		let request = self
			.client
			.post(self.make_url(&format!("management/v1/projects/{project_id}/roles"))?)
			.chain_opt(org_id, |req, org_id| req.header(HEADER_ZITADEL_ORGANIZATION_ID, org_id))
			.json(&ReqBody::new(role_key, display_name).chain_opt(group, ReqBody::with_group))
			.build()
			.context("Error building add_user_grant request")?;

		self.send_request(request).await
	}
}
