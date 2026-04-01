// SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)
//
// SPDX-License-Identifier: Apache-2.0

//! V2 actions API types.
#![allow(
	missing_docs,
	non_snake_case,
	clippy::new_without_default,
	clippy::must_use_candidate,
	clippy::return_self_not_must_use,
	clippy::used_underscore_binding
)]

mod action_service_update_target_body;
pub use self::action_service_update_target_body::ActionServiceUpdateTargetBody;
mod protobuf_any;
pub use self::protobuf_any::ProtobufAny;
mod rpc_status;
pub use self::rpc_status::RpcStatus;
mod v2_condition;
pub use self::v2_condition::V2Condition;
mod v2_create_target_request;
pub use self::v2_create_target_request::V2CreateTargetRequest;
mod v2_create_target_response;
pub use self::v2_create_target_response::V2CreateTargetResponse;
mod v2_delete_target_response;
pub use self::v2_delete_target_response::V2DeleteTargetResponse;
mod v2_event_execution;
pub use self::v2_event_execution::V2EventExecution;
mod v2_execution;
pub use self::v2_execution::V2Execution;
mod v2_execution_field_name;
pub use self::v2_execution_field_name::V2ExecutionFieldName;
mod v2_execution_search_filter;
pub use self::v2_execution_search_filter::V2ExecutionSearchFilter;
mod v2_execution_type;
pub use self::v2_execution_type::V2ExecutionType;
mod v2_execution_type_filter;
pub use self::v2_execution_type_filter::V2ExecutionTypeFilter;
mod v2_function_execution;
pub use self::v2_function_execution::V2FunctionExecution;
mod v2_get_target_response;
pub use self::v2_get_target_response::V2GetTargetResponse;
mod v2_in_conditions_filter;
pub use self::v2_in_conditions_filter::V2InConditionsFilter;
mod v2_in_target_ids_filter;
pub use self::v2_in_target_ids_filter::V2InTargetIdsFilter;
mod v2_list_execution_functions_response;
pub use self::v2_list_execution_functions_response::V2ListExecutionFunctionsResponse;
mod v2_list_execution_methods_response;
pub use self::v2_list_execution_methods_response::V2ListExecutionMethodsResponse;
mod v2_list_execution_services_response;
pub use self::v2_list_execution_services_response::V2ListExecutionServicesResponse;
mod v2_list_executions_response;
pub use self::v2_list_executions_response::V2ListExecutionsResponse;
mod v2_list_targets_request;
pub use self::v2_list_targets_request::V2ListTargetsRequest;
mod v2_list_executions_request;
pub use self::v2_list_executions_request::V2ListExecutionsRequest;
mod v2_list_targets_response;
pub use self::v2_list_targets_response::V2ListTargetsResponse;
mod v2_pagination_request;
pub use self::v2_pagination_request::V2PaginationRequest;
mod v2_pagination_response;
pub use self::v2_pagination_response::V2PaginationResponse;
mod v2_request_execution;
pub use self::v2_request_execution::V2RequestExecution;
mod v2_response_execution;
pub use self::v2_response_execution::V2ResponseExecution;
mod v2_rest_async;
pub use self::v2_rest_async::V2RestAsync;
mod v2_rest_call;
pub use self::v2_rest_call::V2RestCall;
mod v2_rest_webhook;
pub use self::v2_rest_webhook::V2RestWebhook;
mod v2_set_execution_request;
pub use self::v2_set_execution_request::V2SetExecutionRequest;
mod v2_set_execution_response;
pub use self::v2_set_execution_response::V2SetExecutionResponse;
mod v2_target;
pub use self::v2_target::V2Target;
mod v2_target_field_name;
pub use self::v2_target_field_name::V2TargetFieldName;
mod v2_target_filter;
pub use self::v2_target_filter::V2TargetFilter;
mod v2_target_name_filter;
pub use self::v2_target_name_filter::V2TargetNameFilter;
mod v2_target_search_filter;
pub use self::v2_target_search_filter::V2TargetSearchFilter;
mod v2_text_filter_method;
pub use self::v2_text_filter_method::V2TextFilterMethod;
mod v2_update_target_response;
pub use self::v2_update_target_response::V2UpdateTargetResponse;
mod v2_payload_type;
pub use self::v2_payload_type::V2PayloadType;
