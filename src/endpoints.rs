//! Named REST routes for the OpenAI API.
use crate::{Method, Result, client::encode_segment};

/// An HTTP method and relative API path, ready for `Client::endpoint`.
#[derive(Clone, Debug)]
pub struct Endpoint {
    pub method: Method,
    pub path: String,
}

impl Endpoint {
    fn new(method: Method, path: impl Into<String>) -> Self {
        Self {
            method,
            path: path.into(),
        }
    }
}

/// Catalog entries for tooling and inspection.
pub const CATALOG: &[(&str, &str, &str)] = &[
    ("create_speech", "POST", "/audio/speech"),
    ("create_transcription", "POST", "/audio/transcriptions"),
    ("create_translation", "POST", "/audio/translations"),
    ("list_voice_consents", "GET", "/audio/voice_consents"),
    ("create_voice_consent", "POST", "/audio/voice_consents"),
    (
        "delete_voice_consent",
        "DELETE",
        "/audio/voice_consents/{consent_id}",
    ),
    (
        "get_voice_consent",
        "GET",
        "/audio/voice_consents/{consent_id}",
    ),
    (
        "update_voice_consent",
        "POST",
        "/audio/voice_consents/{consent_id}",
    ),
    ("create_voice", "POST", "/audio/voices"),
    ("list_batches", "GET", "/batches"),
    ("create_batch", "POST", "/batches"),
    ("retrieve_batch", "GET", "/batches/{batch_id}"),
    ("cancel_batch", "POST", "/batches/{batch_id}/cancel"),
    ("list_chat_completions", "GET", "/chat/completions"),
    ("create_chat_completion", "POST", "/chat/completions"),
    (
        "delete_chat_completion",
        "DELETE",
        "/chat/completions/{completion_id}",
    ),
    (
        "get_chat_completion",
        "GET",
        "/chat/completions/{completion_id}",
    ),
    (
        "update_chat_completion",
        "POST",
        "/chat/completions/{completion_id}",
    ),
    (
        "get_chat_completion_messages",
        "GET",
        "/chat/completions/{completion_id}/messages",
    ),
    ("create_chat_session_method", "POST", "/chatkit/sessions"),
    (
        "cancel_chat_session_method",
        "POST",
        "/chatkit/sessions/{session_id}/cancel",
    ),
    ("list_threads_method", "GET", "/chatkit/threads"),
    (
        "delete_thread_method",
        "DELETE",
        "/chatkit/threads/{thread_id}",
    ),
    ("get_thread_method", "GET", "/chatkit/threads/{thread_id}"),
    (
        "list_thread_items_method",
        "GET",
        "/chatkit/threads/{thread_id}/items",
    ),
    ("list_containers", "GET", "/containers"),
    ("create_container", "POST", "/containers"),
    ("delete_container", "DELETE", "/containers/{container_id}"),
    ("retrieve_container", "GET", "/containers/{container_id}"),
    (
        "list_container_files",
        "GET",
        "/containers/{container_id}/files",
    ),
    (
        "create_container_file",
        "POST",
        "/containers/{container_id}/files",
    ),
    (
        "delete_container_file",
        "DELETE",
        "/containers/{container_id}/files/{file_id}",
    ),
    (
        "retrieve_container_file",
        "GET",
        "/containers/{container_id}/files/{file_id}",
    ),
    (
        "retrieve_container_file_content",
        "GET",
        "/containers/{container_id}/files/{file_id}/content",
    ),
    (
        "create_content_provenance_check",
        "POST",
        "/content_provenance_checks",
    ),
    ("create_conversation", "POST", "/conversations"),
    (
        "delete_conversation",
        "DELETE",
        "/conversations/{conversation_id}",
    ),
    (
        "get_conversation",
        "GET",
        "/conversations/{conversation_id}",
    ),
    (
        "update_conversation",
        "POST",
        "/conversations/{conversation_id}",
    ),
    (
        "list_conversation_items",
        "GET",
        "/conversations/{conversation_id}/items",
    ),
    (
        "create_conversation_items",
        "POST",
        "/conversations/{conversation_id}/items",
    ),
    (
        "delete_conversation_item",
        "DELETE",
        "/conversations/{conversation_id}/items/{item_id}",
    ),
    (
        "get_conversation_item",
        "GET",
        "/conversations/{conversation_id}/items/{item_id}",
    ),
    ("decisions_create", "POST", "/decisions"),
    ("create_embedding", "POST", "/embeddings"),
    ("list_evals", "GET", "/evals"),
    ("create_eval", "POST", "/evals"),
    ("delete_eval", "DELETE", "/evals/{eval_id}"),
    ("get_eval", "GET", "/evals/{eval_id}"),
    ("update_eval", "POST", "/evals/{eval_id}"),
    ("get_eval_runs", "GET", "/evals/{eval_id}/runs"),
    ("create_eval_run", "POST", "/evals/{eval_id}/runs"),
    (
        "delete_eval_run",
        "DELETE",
        "/evals/{eval_id}/runs/{run_id}",
    ),
    ("get_eval_run", "GET", "/evals/{eval_id}/runs/{run_id}"),
    ("cancel_eval_run", "POST", "/evals/{eval_id}/runs/{run_id}"),
    (
        "evals_runs_cancel",
        "POST",
        "/evals/{eval_id}/runs/{run_id}/cancel",
    ),
    (
        "get_eval_run_output_items",
        "GET",
        "/evals/{eval_id}/runs/{run_id}/output_items",
    ),
    (
        "get_eval_run_output_item",
        "GET",
        "/evals/{eval_id}/runs/{run_id}/output_items/{output_item_id}",
    ),
    ("list_files", "GET", "/files"),
    ("create_file", "POST", "/files"),
    ("delete_file", "DELETE", "/files/{file_id}"),
    ("retrieve_file", "GET", "/files/{file_id}"),
    ("download_file", "GET", "/files/{file_id}/content"),
    ("run_grader", "POST", "/fine_tuning/alpha/graders/run"),
    (
        "validate_grader",
        "POST",
        "/fine_tuning/alpha/graders/validate",
    ),
    (
        "list_fine_tuning_checkpoint_permissions",
        "GET",
        "/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions",
    ),
    (
        "create_fine_tuning_checkpoint_permission",
        "POST",
        "/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions",
    ),
    (
        "delete_fine_tuning_checkpoint_permission",
        "DELETE",
        "/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions/{permission_id}",
    ),
    (
        "list_paginated_fine_tuning_jobs",
        "GET",
        "/fine_tuning/jobs",
    ),
    ("create_fine_tuning_job", "POST", "/fine_tuning/jobs"),
    (
        "retrieve_fine_tuning_job",
        "GET",
        "/fine_tuning/jobs/{fine_tuning_job_id}",
    ),
    (
        "cancel_fine_tuning_job",
        "POST",
        "/fine_tuning/jobs/{fine_tuning_job_id}/cancel",
    ),
    (
        "list_fine_tuning_job_checkpoints",
        "GET",
        "/fine_tuning/jobs/{fine_tuning_job_id}/checkpoints",
    ),
    (
        "list_fine_tuning_events",
        "GET",
        "/fine_tuning/jobs/{fine_tuning_job_id}/events",
    ),
    (
        "pause_fine_tuning_job",
        "POST",
        "/fine_tuning/jobs/{fine_tuning_job_id}/pause",
    ),
    (
        "resume_fine_tuning_job",
        "POST",
        "/fine_tuning/jobs/{fine_tuning_job_id}/resume",
    ),
    ("create_image_edit", "POST", "/images/edits"),
    ("create_image", "POST", "/images/generations"),
    ("live_create", "POST", "/live/sessions"),
    (
        "live_sessions_accept",
        "POST",
        "/live/sessions/{session_id}/accept",
    ),
    (
        "live_sessions_download_recording",
        "GET",
        "/live/sessions/{session_id}/content",
    ),
    (
        "live_sessions_fork",
        "POST",
        "/live/sessions/{session_id}/fork",
    ),
    (
        "live_sessions_hangup",
        "POST",
        "/live/sessions/{session_id}/hangup",
    ),
    (
        "live_sessions_refer",
        "POST",
        "/live/sessions/{session_id}/refer",
    ),
    (
        "live_sessions_reject",
        "POST",
        "/live/sessions/{session_id}/reject",
    ),
    ("list_models", "GET", "/models"),
    ("delete_model", "DELETE", "/models/{model}"),
    ("retrieve_model", "GET", "/models/{model}"),
    ("create_moderation", "POST", "/moderations"),
    ("admin_api_keys_list", "GET", "/organization/admin_api_keys"),
    (
        "admin_api_keys_create",
        "POST",
        "/organization/admin_api_keys",
    ),
    (
        "admin_api_keys_delete",
        "DELETE",
        "/organization/admin_api_keys/{key_id}",
    ),
    (
        "admin_api_keys_get",
        "GET",
        "/organization/admin_api_keys/{key_id}",
    ),
    ("list_audit_logs", "GET", "/organization/audit_logs"),
    (
        "list_organization_certificates",
        "GET",
        "/organization/certificates",
    ),
    ("upload_certificate", "POST", "/organization/certificates"),
    (
        "activate_organization_certificates",
        "POST",
        "/organization/certificates/activate",
    ),
    (
        "deactivate_organization_certificates",
        "POST",
        "/organization/certificates/deactivate",
    ),
    (
        "delete_certificate",
        "DELETE",
        "/organization/certificates/{certificate_id}",
    ),
    (
        "get_certificate",
        "GET",
        "/organization/certificates/{certificate_id}",
    ),
    (
        "modify_certificate",
        "POST",
        "/organization/certificates/{certificate_id}",
    ),
    ("usage_costs", "GET", "/organization/costs"),
    (
        "retrieve_organization_data_retention",
        "GET",
        "/organization/data_retention",
    ),
    (
        "update_organization_data_retention",
        "POST",
        "/organization/data_retention",
    ),
    (
        "admin_organization_external_storage_create",
        "POST",
        "/organization/external_storage",
    ),
    (
        "admin_organization_external_storage_delete",
        "DELETE",
        "/organization/external_storage/{external_storage_id}",
    ),
    (
        "admin_organization_external_storage_retrieve",
        "GET",
        "/organization/external_storage/{external_storage_id}",
    ),
    (
        "admin_organization_external_storage_validate",
        "POST",
        "/organization/external_storage/{external_storage_id}/validate",
    ),
    ("list_groups", "GET", "/organization/groups"),
    ("create_group", "POST", "/organization/groups"),
    ("delete_group", "DELETE", "/organization/groups/{group_id}"),
    ("retrieve_group", "GET", "/organization/groups/{group_id}"),
    ("update_group", "POST", "/organization/groups/{group_id}"),
    (
        "list_group_role_assignments",
        "GET",
        "/organization/groups/{group_id}/roles",
    ),
    (
        "assign_group_role",
        "POST",
        "/organization/groups/{group_id}/roles",
    ),
    (
        "unassign_group_role",
        "DELETE",
        "/organization/groups/{group_id}/roles/{role_id}",
    ),
    (
        "retrieve_group_role",
        "GET",
        "/organization/groups/{group_id}/roles/{role_id}",
    ),
    (
        "list_group_users",
        "GET",
        "/organization/groups/{group_id}/users",
    ),
    (
        "add_group_user",
        "POST",
        "/organization/groups/{group_id}/users",
    ),
    (
        "remove_group_user",
        "DELETE",
        "/organization/groups/{group_id}/users/{user_id}",
    ),
    (
        "retrieve_group_user",
        "GET",
        "/organization/groups/{group_id}/users/{user_id}",
    ),
    ("list_invites", "GET", "/organization/invites"),
    ("invite_user", "POST", "/organization/invites"),
    (
        "delete_invite",
        "DELETE",
        "/organization/invites/{invite_id}",
    ),
    (
        "retrieve_invite",
        "GET",
        "/organization/invites/{invite_id}",
    ),
    ("list_projects", "GET", "/organization/projects"),
    ("create_project", "POST", "/organization/projects"),
    (
        "retrieve_project",
        "GET",
        "/organization/projects/{project_id}",
    ),
    (
        "modify_project",
        "POST",
        "/organization/projects/{project_id}",
    ),
    (
        "list_project_api_keys",
        "GET",
        "/organization/projects/{project_id}/api_keys",
    ),
    (
        "delete_project_api_key",
        "DELETE",
        "/organization/projects/{project_id}/api_keys/{api_key_id}",
    ),
    (
        "retrieve_project_api_key",
        "GET",
        "/organization/projects/{project_id}/api_keys/{api_key_id}",
    ),
    (
        "admin_organization_projects_api_keys_update",
        "POST",
        "/organization/projects/{project_id}/api_keys/{api_key_id}",
    ),
    (
        "archive_project",
        "POST",
        "/organization/projects/{project_id}/archive",
    ),
    (
        "list_project_certificates",
        "GET",
        "/organization/projects/{project_id}/certificates",
    ),
    (
        "activate_project_certificates",
        "POST",
        "/organization/projects/{project_id}/certificates/activate",
    ),
    (
        "deactivate_project_certificates",
        "POST",
        "/organization/projects/{project_id}/certificates/deactivate",
    ),
    (
        "retrieve_project_data_retention",
        "GET",
        "/organization/projects/{project_id}/data_retention",
    ),
    (
        "update_project_data_retention",
        "POST",
        "/organization/projects/{project_id}/data_retention",
    ),
    (
        "list_project_groups",
        "GET",
        "/organization/projects/{project_id}/groups",
    ),
    (
        "add_project_group",
        "POST",
        "/organization/projects/{project_id}/groups",
    ),
    (
        "remove_project_group",
        "DELETE",
        "/organization/projects/{project_id}/groups/{group_id}",
    ),
    (
        "retrieve_project_group",
        "GET",
        "/organization/projects/{project_id}/groups/{group_id}",
    ),
    (
        "retrieve_project_hosted_tool_permissions",
        "GET",
        "/organization/projects/{project_id}/hosted_tool_permissions",
    ),
    (
        "update_project_hosted_tool_permissions",
        "POST",
        "/organization/projects/{project_id}/hosted_tool_permissions",
    ),
    (
        "delete_project_model_permissions",
        "DELETE",
        "/organization/projects/{project_id}/model_permissions",
    ),
    (
        "retrieve_project_model_permissions",
        "GET",
        "/organization/projects/{project_id}/model_permissions",
    ),
    (
        "update_project_model_permissions",
        "POST",
        "/organization/projects/{project_id}/model_permissions",
    ),
    (
        "list_project_rate_limits",
        "GET",
        "/organization/projects/{project_id}/rate_limits",
    ),
    (
        "update_project_rate_limits",
        "POST",
        "/organization/projects/{project_id}/rate_limits/{rate_limit_id}",
    ),
    (
        "list_project_service_accounts",
        "GET",
        "/organization/projects/{project_id}/service_accounts",
    ),
    (
        "create_project_service_account",
        "POST",
        "/organization/projects/{project_id}/service_accounts",
    ),
    (
        "delete_project_service_account",
        "DELETE",
        "/organization/projects/{project_id}/service_accounts/{service_account_id}",
    ),
    (
        "retrieve_project_service_account",
        "GET",
        "/organization/projects/{project_id}/service_accounts/{service_account_id}",
    ),
    (
        "update_project_service_account",
        "POST",
        "/organization/projects/{project_id}/service_accounts/{service_account_id}",
    ),
    (
        "create_service_account_api_key",
        "POST",
        "/organization/projects/{project_id}/service_accounts/{service_account_id}/api_keys",
    ),
    (
        "list_project_spend_alerts",
        "GET",
        "/organization/projects/{project_id}/spend_alerts",
    ),
    (
        "create_project_spend_alert",
        "POST",
        "/organization/projects/{project_id}/spend_alerts",
    ),
    (
        "delete_project_spend_alert",
        "DELETE",
        "/organization/projects/{project_id}/spend_alerts/{alert_id}",
    ),
    (
        "retrieve_project_spend_alert",
        "GET",
        "/organization/projects/{project_id}/spend_alerts/{alert_id}",
    ),
    (
        "update_project_spend_alert",
        "POST",
        "/organization/projects/{project_id}/spend_alerts/{alert_id}",
    ),
    (
        "delete_project_spend_limit",
        "DELETE",
        "/organization/projects/{project_id}/spend_limit",
    ),
    (
        "get_project_spend_limit",
        "GET",
        "/organization/projects/{project_id}/spend_limit",
    ),
    (
        "update_project_spend_limit",
        "POST",
        "/organization/projects/{project_id}/spend_limit",
    ),
    (
        "list_project_users",
        "GET",
        "/organization/projects/{project_id}/users",
    ),
    (
        "create_project_user",
        "POST",
        "/organization/projects/{project_id}/users",
    ),
    (
        "delete_project_user",
        "DELETE",
        "/organization/projects/{project_id}/users/{user_id}",
    ),
    (
        "retrieve_project_user",
        "GET",
        "/organization/projects/{project_id}/users/{user_id}",
    ),
    (
        "modify_project_user",
        "POST",
        "/organization/projects/{project_id}/users/{user_id}",
    ),
    ("list_roles", "GET", "/organization/roles"),
    ("create_role", "POST", "/organization/roles"),
    ("delete_role", "DELETE", "/organization/roles/{role_id}"),
    ("retrieve_role", "GET", "/organization/roles/{role_id}"),
    ("update_role", "POST", "/organization/roles/{role_id}"),
    (
        "list_organization_spend_alerts",
        "GET",
        "/organization/spend_alerts",
    ),
    (
        "create_organization_spend_alert",
        "POST",
        "/organization/spend_alerts",
    ),
    (
        "delete_organization_spend_alert",
        "DELETE",
        "/organization/spend_alerts/{alert_id}",
    ),
    (
        "retrieve_organization_spend_alert",
        "GET",
        "/organization/spend_alerts/{alert_id}",
    ),
    (
        "update_organization_spend_alert",
        "POST",
        "/organization/spend_alerts/{alert_id}",
    ),
    (
        "delete_organization_spend_limit",
        "DELETE",
        "/organization/spend_limit",
    ),
    (
        "get_organization_spend_limit",
        "GET",
        "/organization/spend_limit",
    ),
    (
        "update_organization_spend_limit",
        "POST",
        "/organization/spend_limit",
    ),
    (
        "usage_audio_speeches",
        "GET",
        "/organization/usage/audio_speeches",
    ),
    (
        "usage_audio_transcriptions",
        "GET",
        "/organization/usage/audio_transcriptions",
    ),
    (
        "usage_code_interpreter_sessions",
        "GET",
        "/organization/usage/code_interpreter_sessions",
    ),
    (
        "usage_completions",
        "GET",
        "/organization/usage/completions",
    ),
    (
        "admin_organization_usage_decisions",
        "GET",
        "/organization/usage/decisions",
    ),
    ("usage_embeddings", "GET", "/organization/usage/embeddings"),
    (
        "usage_file_search_calls",
        "GET",
        "/organization/usage/file_search_calls",
    ),
    ("usage_images", "GET", "/organization/usage/images"),
    (
        "usage_moderations",
        "GET",
        "/organization/usage/moderations",
    ),
    (
        "usage_vector_stores",
        "GET",
        "/organization/usage/vector_stores",
    ),
    (
        "usage_web_search_calls",
        "GET",
        "/organization/usage/web_search_calls",
    ),
    ("list_users", "GET", "/organization/users"),
    ("delete_user", "DELETE", "/organization/users/{user_id}"),
    ("retrieve_user", "GET", "/organization/users/{user_id}"),
    ("modify_user", "POST", "/organization/users/{user_id}"),
    (
        "list_user_role_assignments",
        "GET",
        "/organization/users/{user_id}/roles",
    ),
    (
        "assign_user_role",
        "POST",
        "/organization/users/{user_id}/roles",
    ),
    (
        "unassign_user_role",
        "DELETE",
        "/organization/users/{user_id}/roles/{role_id}",
    ),
    (
        "retrieve_user_role",
        "GET",
        "/organization/users/{user_id}/roles/{role_id}",
    ),
    (
        "list_project_group_role_assignments",
        "GET",
        "/projects/{project_id}/groups/{group_id}/roles",
    ),
    (
        "assign_project_group_role",
        "POST",
        "/projects/{project_id}/groups/{group_id}/roles",
    ),
    (
        "unassign_project_group_role",
        "DELETE",
        "/projects/{project_id}/groups/{group_id}/roles/{role_id}",
    ),
    (
        "retrieve_project_group_role",
        "GET",
        "/projects/{project_id}/groups/{group_id}/roles/{role_id}",
    ),
    ("list_project_roles", "GET", "/projects/{project_id}/roles"),
    (
        "create_project_role",
        "POST",
        "/projects/{project_id}/roles",
    ),
    (
        "delete_project_role",
        "DELETE",
        "/projects/{project_id}/roles/{role_id}",
    ),
    (
        "retrieve_project_role",
        "GET",
        "/projects/{project_id}/roles/{role_id}",
    ),
    (
        "update_project_role",
        "POST",
        "/projects/{project_id}/roles/{role_id}",
    ),
    (
        "list_project_user_role_assignments",
        "GET",
        "/projects/{project_id}/users/{user_id}/roles",
    ),
    (
        "assign_project_user_role",
        "POST",
        "/projects/{project_id}/users/{user_id}/roles",
    ),
    (
        "unassign_project_user_role",
        "DELETE",
        "/projects/{project_id}/users/{user_id}/roles/{role_id}",
    ),
    (
        "retrieve_project_user_role",
        "GET",
        "/projects/{project_id}/users/{user_id}/roles/{role_id}",
    ),
    ("create_realtime_call", "POST", "/realtime/calls"),
    (
        "accept_realtime_call",
        "POST",
        "/realtime/calls/{call_id}/accept",
    ),
    (
        "hangup_realtime_call",
        "POST",
        "/realtime/calls/{call_id}/hangup",
    ),
    (
        "refer_realtime_call",
        "POST",
        "/realtime/calls/{call_id}/refer",
    ),
    (
        "reject_realtime_call",
        "POST",
        "/realtime/calls/{call_id}/reject",
    ),
    (
        "create_realtime_client_secret",
        "POST",
        "/realtime/client_secrets",
    ),
    (
        "create_realtime_translation_client_secret",
        "POST",
        "/realtime/translations/client_secrets",
    ),
    ("create_response", "POST", "/responses"),
    ("compact_response", "POST", "/responses/compact"),
    (
        "count_response_input_tokens",
        "POST",
        "/responses/input_tokens",
    ),
    ("delete_response", "DELETE", "/responses/{response_id}"),
    ("get_response", "GET", "/responses/{response_id}"),
    ("cancel_response", "POST", "/responses/{response_id}/cancel"),
    (
        "list_input_items",
        "GET",
        "/responses/{response_id}/input_items",
    ),
    ("get_project_safety_alert", "GET", "/safety/alerts/{id}"),
    ("safety_cases_retrieve", "GET", "/safety/cases/{id}"),
    ("list_skills", "GET", "/skills"),
    ("create_skill", "POST", "/skills"),
    ("delete_skill", "DELETE", "/skills/{skill_id}"),
    ("get_skill", "GET", "/skills/{skill_id}"),
    ("update_skill_default_version", "POST", "/skills/{skill_id}"),
    ("get_skill_content", "GET", "/skills/{skill_id}/content"),
    ("list_skill_versions", "GET", "/skills/{skill_id}/versions"),
    (
        "create_skill_version",
        "POST",
        "/skills/{skill_id}/versions",
    ),
    (
        "delete_skill_version",
        "DELETE",
        "/skills/{skill_id}/versions/{version}",
    ),
    (
        "get_skill_version",
        "GET",
        "/skills/{skill_id}/versions/{version}",
    ),
    (
        "get_skill_version_content",
        "GET",
        "/skills/{skill_id}/versions/{version}/content",
    ),
    ("create_upload", "POST", "/uploads"),
    ("cancel_upload", "POST", "/uploads/{upload_id}/cancel"),
    ("complete_upload", "POST", "/uploads/{upload_id}/complete"),
    ("add_upload_part", "POST", "/uploads/{upload_id}/parts"),
    ("list_vector_stores", "GET", "/vector_stores"),
    ("create_vector_store", "POST", "/vector_stores"),
    (
        "delete_vector_store",
        "DELETE",
        "/vector_stores/{vector_store_id}",
    ),
    (
        "get_vector_store",
        "GET",
        "/vector_stores/{vector_store_id}",
    ),
    (
        "modify_vector_store",
        "POST",
        "/vector_stores/{vector_store_id}",
    ),
    (
        "create_vector_store_file_batch",
        "POST",
        "/vector_stores/{vector_store_id}/file_batches",
    ),
    (
        "get_vector_store_file_batch",
        "GET",
        "/vector_stores/{vector_store_id}/file_batches/{batch_id}",
    ),
    (
        "cancel_vector_store_file_batch",
        "POST",
        "/vector_stores/{vector_store_id}/file_batches/{batch_id}/cancel",
    ),
    (
        "list_files_in_vector_store_batch",
        "GET",
        "/vector_stores/{vector_store_id}/file_batches/{batch_id}/files",
    ),
    (
        "list_vector_store_files",
        "GET",
        "/vector_stores/{vector_store_id}/files",
    ),
    (
        "create_vector_store_file",
        "POST",
        "/vector_stores/{vector_store_id}/files",
    ),
    (
        "delete_vector_store_file",
        "DELETE",
        "/vector_stores/{vector_store_id}/files/{file_id}",
    ),
    (
        "get_vector_store_file",
        "GET",
        "/vector_stores/{vector_store_id}/files/{file_id}",
    ),
    (
        "update_vector_store_file_attributes",
        "POST",
        "/vector_stores/{vector_store_id}/files/{file_id}",
    ),
    (
        "retrieve_vector_store_file_content",
        "GET",
        "/vector_stores/{vector_store_id}/files/{file_id}/content",
    ),
    (
        "search_vector_store",
        "POST",
        "/vector_stores/{vector_store_id}/search",
    ),
    ("webhooks_create", "POST", "/webhook_endpoints"),
    (
        "webhooks_delete",
        "DELETE",
        "/webhook_endpoints/{webhook_endpoint_id}",
    ),
    (
        "webhooks_retrieve",
        "GET",
        "/webhook_endpoints/{webhook_endpoint_id}",
    ),
    (
        "webhooks_update",
        "POST",
        "/webhook_endpoints/{webhook_endpoint_id}",
    ),
    (
        "webhooks_rotate_secret",
        "POST",
        "/webhook_endpoints/{webhook_endpoint_id}/rotate_secret",
    ),
    (
        "webhooks_test",
        "POST",
        "/webhook_endpoints/{webhook_endpoint_id}/test",
    ),
    ("webhooks_event_types_list", "GET", "/webhook_event_types"),
];

/// Generates audio from the input text. Returns the audio file content, or a stream of audio events.
pub fn create_speech() -> Endpoint {
    Endpoint::new(Method::POST, "/audio/speech")
}

/// Transcribes audio into the input language. Returns a transcription object in `json`, `diarized_json`, or `verbose_json` format, or a stream of transcript events.
pub fn create_transcription() -> Endpoint {
    Endpoint::new(Method::POST, "/audio/transcriptions")
}

/// Translates audio into English.
pub fn create_translation() -> Endpoint {
    Endpoint::new(Method::POST, "/audio/translations")
}

/// Returns a list of voice consent recordings.
pub fn list_voice_consents() -> Endpoint {
    Endpoint::new(Method::GET, "/audio/voice_consents")
}

/// Upload a voice consent recording.
pub fn create_voice_consent() -> Endpoint {
    Endpoint::new(Method::POST, "/audio/voice_consents")
}

/// Deletes a voice consent recording.
pub fn delete_voice_consent(consent_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/audio/voice_consents/{}", encode_segment(consent_id)?),
    ))
}

/// Retrieves a voice consent recording.
pub fn get_voice_consent(consent_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/audio/voice_consents/{}", encode_segment(consent_id)?),
    ))
}

/// Updates a voice consent recording (metadata only).
pub fn update_voice_consent(consent_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/audio/voice_consents/{}", encode_segment(consent_id)?),
    ))
}

/// Creates a custom voice.
pub fn create_voice() -> Endpoint {
    Endpoint::new(Method::POST, "/audio/voices")
}

/// List your organization's batches.
pub fn list_batches() -> Endpoint {
    Endpoint::new(Method::GET, "/batches")
}

/// Creates and executes a batch from an uploaded file of requests
pub fn create_batch() -> Endpoint {
    Endpoint::new(Method::POST, "/batches")
}

/// Retrieves a batch.
pub fn retrieve_batch(batch_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/batches/{}", encode_segment(batch_id)?),
    ))
}

/// Cancel a batch; partial results become available when cancellation finishes.
pub fn cancel_batch(batch_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/batches/{}/cancel", encode_segment(batch_id)?),
    ))
}

/// List stored Chat Completions. Only Chat Completions that have been stored with the `store` parameter set to `true` will be returned.
pub fn list_chat_completions() -> Endpoint {
    Endpoint::new(Method::GET, "/chat/completions")
}

/// Create a chat completion, optionally streamed.
pub fn create_chat_completion() -> Endpoint {
    Endpoint::new(Method::POST, "/chat/completions")
}

/// Delete a stored chat completion. Only Chat Completions that have been created with the `store` parameter set to `true` can be deleted.
pub fn delete_chat_completion(completion_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/chat/completions/{}", encode_segment(completion_id)?),
    ))
}

/// Get a stored chat completion. Only Chat Completions that have been created with the `store` parameter set to `true` will be returned.
pub fn get_chat_completion(completion_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/chat/completions/{}", encode_segment(completion_id)?),
    ))
}

/// Modify a stored chat completion. Only Chat Completions that have been created with the `store` parameter set to `true` can be modified. Currently, the only supported modification is to update the `metadata` field.
pub fn update_chat_completion(completion_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/chat/completions/{}", encode_segment(completion_id)?),
    ))
}

/// Get the messages in a stored chat completion. Only Chat Completions that have been created with the `store` parameter set to `true` will be returned.
pub fn get_chat_completion_messages(completion_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/chat/completions/{}/messages",
            encode_segment(completion_id)?
        ),
    ))
}

/// Create a ChatKit session.
pub fn create_chat_session_method() -> Endpoint {
    Endpoint::new(Method::POST, "/chatkit/sessions")
}

/// Cancel an active ChatKit session and return its most recent metadata. Cancelling prevents new requests from using the issued client secret.
pub fn cancel_chat_session_method(session_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/chatkit/sessions/{}/cancel", encode_segment(session_id)?),
    ))
}

/// List ChatKit threads with optional pagination and user filters.
pub fn list_threads_method() -> Endpoint {
    Endpoint::new(Method::GET, "/chatkit/threads")
}

/// Delete a ChatKit thread along with its items and stored attachments.
pub fn delete_thread_method(thread_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/chatkit/threads/{}", encode_segment(thread_id)?),
    ))
}

/// Retrieve a ChatKit thread by its identifier.
pub fn get_thread_method(thread_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/chatkit/threads/{}", encode_segment(thread_id)?),
    ))
}

/// List items that belong to a ChatKit thread.
pub fn list_thread_items_method(thread_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/chatkit/threads/{}/items", encode_segment(thread_id)?),
    ))
}

/// List Containers
pub fn list_containers() -> Endpoint {
    Endpoint::new(Method::GET, "/containers")
}

/// Create Container
pub fn create_container() -> Endpoint {
    Endpoint::new(Method::POST, "/containers")
}

/// Delete Container
pub fn delete_container(container_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/containers/{}", encode_segment(container_id)?),
    ))
}

/// Retrieve Container
pub fn retrieve_container(container_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/containers/{}", encode_segment(container_id)?),
    ))
}

/// List Container files
pub fn list_container_files(container_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/containers/{}/files", encode_segment(container_id)?),
    ))
}

/// Create a Container File You can send either a multipart/form-data request with the raw file content, or a JSON request with a file ID.
pub fn create_container_file(container_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/containers/{}/files", encode_segment(container_id)?),
    ))
}

/// Delete Container File
pub fn delete_container_file(container_id: &str, file_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/containers/{}/files/{}",
            encode_segment(container_id)?,
            encode_segment(file_id)?
        ),
    ))
}

/// Retrieve Container File
pub fn retrieve_container_file(container_id: &str, file_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/containers/{}/files/{}",
            encode_segment(container_id)?,
            encode_segment(file_id)?
        ),
    ))
}

/// Retrieve Container File Content
pub fn retrieve_container_file_content(container_id: &str, file_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/containers/{}/files/{}/content",
            encode_segment(container_id)?,
            encode_segment(file_id)?
        ),
    ))
}

/// Check image or audio content for OpenAI provenance signals.
pub fn create_content_provenance_check() -> Endpoint {
    Endpoint::new(Method::POST, "/content_provenance_checks")
}

/// Create a conversation.
pub fn create_conversation() -> Endpoint {
    Endpoint::new(Method::POST, "/conversations")
}

/// Delete a conversation. Items in the conversation will not be deleted.
pub fn delete_conversation(conversation_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/conversations/{}", encode_segment(conversation_id)?),
    ))
}

/// Get a conversation
pub fn get_conversation(conversation_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/conversations/{}", encode_segment(conversation_id)?),
    ))
}

/// Update a conversation
pub fn update_conversation(conversation_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/conversations/{}", encode_segment(conversation_id)?),
    ))
}

/// List all items for a conversation with the given ID.
pub fn list_conversation_items(conversation_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/conversations/{}/items", encode_segment(conversation_id)?),
    ))
}

/// Create items in a conversation with the given ID.
pub fn create_conversation_items(conversation_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/conversations/{}/items", encode_segment(conversation_id)?),
    ))
}

/// Delete an item from a conversation with the given IDs.
pub fn delete_conversation_item(conversation_id: &str, item_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/conversations/{}/items/{}",
            encode_segment(conversation_id)?,
            encode_segment(item_id)?
        ),
    ))
}

/// Get a single item from a conversation with the given IDs.
pub fn get_conversation_item(conversation_id: &str, item_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/conversations/{}/items/{}",
            encode_segment(conversation_id)?,
            encode_segment(item_id)?
        ),
    ))
}

/// Use this endpoint to quickly answer classification or scoring questions about some input. You can ask multiple questions in one request.
pub fn decisions_create() -> Endpoint {
    Endpoint::new(Method::POST, "/decisions")
}

/// Creates an embedding vector representing the input text.
pub fn create_embedding() -> Endpoint {
    Endpoint::new(Method::POST, "/embeddings")
}

/// List evaluations for a project.
pub fn list_evals() -> Endpoint {
    Endpoint::new(Method::GET, "/evals")
}

/// Create an evaluation with criteria and a data source.
pub fn create_eval() -> Endpoint {
    Endpoint::new(Method::POST, "/evals")
}

/// Delete an evaluation.
pub fn delete_eval(eval_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/evals/{}", encode_segment(eval_id)?),
    ))
}

/// Get an evaluation by ID.
pub fn get_eval(eval_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/evals/{}", encode_segment(eval_id)?),
    ))
}

/// Update certain properties of an evaluation.
pub fn update_eval(eval_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/evals/{}", encode_segment(eval_id)?),
    ))
}

/// Get a list of runs for an evaluation.
pub fn get_eval_runs(eval_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/evals/{}/runs", encode_segment(eval_id)?),
    ))
}

/// Kicks off a new run for a given evaluation, specifying the data source, and what model configuration to use to test. The datasource will be validated against the schema specified in the config of the evaluation.
pub fn create_eval_run(eval_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/evals/{}/runs", encode_segment(eval_id)?),
    ))
}

/// Delete an eval run.
pub fn delete_eval_run(eval_id: &str, run_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/evals/{}/runs/{}",
            encode_segment(eval_id)?,
            encode_segment(run_id)?
        ),
    ))
}

/// Get an evaluation run by ID.
pub fn get_eval_run(eval_id: &str, run_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/evals/{}/runs/{}",
            encode_segment(eval_id)?,
            encode_segment(run_id)?
        ),
    ))
}

/// Cancel an ongoing evaluation run.
pub fn cancel_eval_run(eval_id: &str, run_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/evals/{}/runs/{}",
            encode_segment(eval_id)?,
            encode_segment(run_id)?
        ),
    ))
}

/// Cancel an ongoing evaluation run.
pub fn evals_runs_cancel(eval_id: &str, run_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/evals/{}/runs/{}/cancel",
            encode_segment(eval_id)?,
            encode_segment(run_id)?
        ),
    ))
}

/// Get a list of output items for an evaluation run.
pub fn get_eval_run_output_items(eval_id: &str, run_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/evals/{}/runs/{}/output_items",
            encode_segment(eval_id)?,
            encode_segment(run_id)?
        ),
    ))
}

/// Get an evaluation run output item by ID.
pub fn get_eval_run_output_item(
    eval_id: &str,
    run_id: &str,
    output_item_id: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/evals/{}/runs/{}/output_items/{}",
            encode_segment(eval_id)?,
            encode_segment(run_id)?,
            encode_segment(output_item_id)?
        ),
    ))
}

/// Returns a list of files.
pub fn list_files() -> Endpoint {
    Endpoint::new(Method::GET, "/files")
}

/// Upload a file for use with API requests.
pub fn create_file() -> Endpoint {
    Endpoint::new(Method::POST, "/files")
}

/// Delete a file and remove it from all vector stores.
pub fn delete_file(file_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/files/{}", encode_segment(file_id)?),
    ))
}

/// Returns information about a specific file.
pub fn retrieve_file(file_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/files/{}", encode_segment(file_id)?),
    ))
}

/// Returns a response containing the contents of the specified file.
pub fn download_file(file_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/files/{}/content", encode_segment(file_id)?),
    ))
}

/// Run a grader.
pub fn run_grader() -> Endpoint {
    Endpoint::new(Method::POST, "/fine_tuning/alpha/graders/run")
}

/// Validate a grader.
pub fn validate_grader() -> Endpoint {
    Endpoint::new(Method::POST, "/fine_tuning/alpha/graders/validate")
}

/// **NOTE:** This endpoint requires an [admin API key](https://developers.openai.com/api/reference/resources/admin/subresources/organization/subresources/admin_api_keys). Organization owners can use this endpoint to view all permissions for a fine-tuned model checkpoint.
pub fn list_fine_tuning_checkpoint_permissions(
    fine_tuned_model_checkpoint: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/fine_tuning/checkpoints/{}/permissions",
            encode_segment(fine_tuned_model_checkpoint)?
        ),
    ))
}

/// **NOTE:** Calling this endpoint requires an [admin API key](https://developers.openai.com/api/reference/resources/admin/subresources/organization/subresources/admin_api_keys). This enables organization owners to share fine-tuned models with other projects in their organization.
pub fn create_fine_tuning_checkpoint_permission(
    fine_tuned_model_checkpoint: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/fine_tuning/checkpoints/{}/permissions",
            encode_segment(fine_tuned_model_checkpoint)?
        ),
    ))
}

/// **NOTE:** This endpoint requires an [admin API key](https://developers.openai.com/api/reference/resources/admin/subresources/organization/subresources/admin_api_keys). Organization owners can use this endpoint to delete a permission for a fine-tuned model checkpoint.
pub fn delete_fine_tuning_checkpoint_permission(
    fine_tuned_model_checkpoint: &str,
    permission_id: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/fine_tuning/checkpoints/{}/permissions/{}",
            encode_segment(fine_tuned_model_checkpoint)?,
            encode_segment(permission_id)?
        ),
    ))
}

/// List your organization's fine-tuning jobs
pub fn list_paginated_fine_tuning_jobs() -> Endpoint {
    Endpoint::new(Method::GET, "/fine_tuning/jobs")
}

/// Create a fine-tuning job from a dataset.
pub fn create_fine_tuning_job() -> Endpoint {
    Endpoint::new(Method::POST, "/fine_tuning/jobs")
}

/// Get info about a fine-tuning job. [Learn more about fine-tuning](https://developers.openai.com/api/docs/guides/model-optimization)
pub fn retrieve_fine_tuning_job(fine_tuning_job_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/fine_tuning/jobs/{}", encode_segment(fine_tuning_job_id)?),
    ))
}

/// Immediately cancel a fine-tune job.
pub fn cancel_fine_tuning_job(fine_tuning_job_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/fine_tuning/jobs/{}/cancel",
            encode_segment(fine_tuning_job_id)?
        ),
    ))
}

/// List checkpoints for a fine-tuning job.
pub fn list_fine_tuning_job_checkpoints(fine_tuning_job_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/fine_tuning/jobs/{}/checkpoints",
            encode_segment(fine_tuning_job_id)?
        ),
    ))
}

/// Get status updates for a fine-tuning job.
pub fn list_fine_tuning_events(fine_tuning_job_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/fine_tuning/jobs/{}/events",
            encode_segment(fine_tuning_job_id)?
        ),
    ))
}

/// Pause a fine-tune job.
pub fn pause_fine_tuning_job(fine_tuning_job_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/fine_tuning/jobs/{}/pause",
            encode_segment(fine_tuning_job_id)?
        ),
    ))
}

/// Resume a fine-tune job.
pub fn resume_fine_tuning_job(fine_tuning_job_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/fine_tuning/jobs/{}/resume",
            encode_segment(fine_tuning_job_id)?
        ),
    ))
}

/// Creates an edited or extended image given one or more source images and a prompt. This endpoint supports GPT Image models and `dall-e-2`.
pub fn create_image_edit() -> Endpoint {
    Endpoint::new(Method::POST, "/images/edits")
}

/// Creates an image given a prompt. [Learn more](https://developers.openai.com/api/docs/guides/images-vision).
pub fn create_image() -> Endpoint {
    Endpoint::new(Method::POST, "/images/generations")
}

/// Create a Live WebRTC session.
pub fn live_create() -> Endpoint {
    Endpoint::new(Method::POST, "/live/sessions")
}

/// Accept an incoming SIP call.
pub fn live_sessions_accept(session_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/live/sessions/{}/accept", encode_segment(session_id)?),
    ))
}

/// Get Live session content
pub fn live_sessions_download_recording(session_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/live/sessions/{}/content", encode_segment(session_id)?),
    ))
}

/// Fork a stored Live session onto a new WebRTC connection.
pub fn live_sessions_fork(session_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/live/sessions/{}/fork", encode_segment(session_id)?),
    ))
}

/// End a SIP call identified by session_id.
pub fn live_sessions_hangup(session_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/live/sessions/{}/hangup", encode_segment(session_id)?),
    ))
}

/// Transfer a SIP call to another destination.
pub fn live_sessions_refer(session_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/live/sessions/{}/refer", encode_segment(session_id)?),
    ))
}

/// Reject an incoming SIP call.
pub fn live_sessions_reject(session_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/live/sessions/{}/reject", encode_segment(session_id)?),
    ))
}

/// Lists the currently available models, and provides basic information about each one such as the owner and availability.
pub fn list_models() -> Endpoint {
    Endpoint::new(Method::GET, "/models")
}

/// Delete a fine-tuned model. You must have the Owner role in your organization to delete a model.
pub fn delete_model(model: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/models/{}", encode_segment(model)?),
    ))
}

/// Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
pub fn retrieve_model(model: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/models/{}", encode_segment(model)?),
    ))
}

/// Classifies if text and/or image inputs are potentially harmful. Learn more in the [moderation guide](https://developers.openai.com/api/docs/guides/moderation).
pub fn create_moderation() -> Endpoint {
    Endpoint::new(Method::POST, "/moderations")
}

/// List organization API keys
pub fn admin_api_keys_list() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/admin_api_keys")
}

/// Create an organization admin API key
pub fn admin_api_keys_create() -> Endpoint {
    Endpoint::new(Method::POST, "/organization/admin_api_keys")
}

/// Delete an organization admin API key
pub fn admin_api_keys_delete(key_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/organization/admin_api_keys/{}", encode_segment(key_id)?),
    ))
}

/// Retrieve a single organization API key
pub fn admin_api_keys_get(key_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/organization/admin_api_keys/{}", encode_segment(key_id)?),
    ))
}

/// List user actions and configuration changes within this organization.
pub fn list_audit_logs() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/audit_logs")
}

/// List uploaded certificates for this organization.
pub fn list_organization_certificates() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/certificates")
}

/// Upload a certificate to the organization. This does **not** automatically activate the certificate. Organizations can upload up to 50 certificates.
pub fn upload_certificate() -> Endpoint {
    Endpoint::new(Method::POST, "/organization/certificates")
}

/// Activate certificates at the organization level. You can atomically and idempotently activate up to 10 certificates at a time.
pub fn activate_organization_certificates() -> Endpoint {
    Endpoint::new(Method::POST, "/organization/certificates/activate")
}

/// Deactivate certificates at the organization level. You can atomically and idempotently deactivate up to 10 certificates at a time.
pub fn deactivate_organization_certificates() -> Endpoint {
    Endpoint::new(Method::POST, "/organization/certificates/deactivate")
}

/// Delete a certificate from the organization. The certificate must be inactive for the organization and all projects.
pub fn delete_certificate(certificate_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/organization/certificates/{}",
            encode_segment(certificate_id)?
        ),
    ))
}

/// Get a certificate that has been uploaded to the organization. You can get a certificate regardless of whether it is active or not.
pub fn get_certificate(certificate_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/certificates/{}",
            encode_segment(certificate_id)?
        ),
    ))
}

/// Modify a certificate. Note that only the name can be modified.
pub fn modify_certificate(certificate_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/certificates/{}",
            encode_segment(certificate_id)?
        ),
    ))
}

/// Get costs details for the organization.
pub fn usage_costs() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/costs")
}

/// Retrieves organization data retention controls.
pub fn retrieve_organization_data_retention() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/data_retention")
}

/// Updates organization data retention controls.
pub fn update_organization_data_retention() -> Endpoint {
    Endpoint::new(Method::POST, "/organization/data_retention")
}

/// Register one customer-managed external storage configuration.
pub fn admin_organization_external_storage_create() -> Endpoint {
    Endpoint::new(Method::POST, "/organization/external_storage")
}

/// Disconnect a customer-managed external storage configuration.
pub fn admin_organization_external_storage_delete(external_storage_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/organization/external_storage/{}",
            encode_segment(external_storage_id)?
        ),
    ))
}

/// Get one customer-managed external storage configuration.
pub fn admin_organization_external_storage_retrieve(external_storage_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/external_storage/{}",
            encode_segment(external_storage_id)?
        ),
    ))
}

/// Validate one customer-managed external storage configuration.
pub fn admin_organization_external_storage_validate(external_storage_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/external_storage/{}/validate",
            encode_segment(external_storage_id)?
        ),
    ))
}

/// Lists all groups in the organization.
pub fn list_groups() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/groups")
}

/// Creates a new group in the organization.
pub fn create_group() -> Endpoint {
    Endpoint::new(Method::POST, "/organization/groups")
}

/// Deletes a group from the organization.
pub fn delete_group(group_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/organization/groups/{}", encode_segment(group_id)?),
    ))
}

/// Retrieves a group.
pub fn retrieve_group(group_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/organization/groups/{}", encode_segment(group_id)?),
    ))
}

/// Updates a group's information.
pub fn update_group(group_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/organization/groups/{}", encode_segment(group_id)?),
    ))
}

/// Lists the organization roles assigned to a group within the organization.
pub fn list_group_role_assignments(group_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/organization/groups/{}/roles", encode_segment(group_id)?),
    ))
}

/// Assigns an organization role to a group within the organization.
pub fn assign_group_role(group_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/organization/groups/{}/roles", encode_segment(group_id)?),
    ))
}

/// Unassigns an organization role from a group within the organization.
pub fn unassign_group_role(group_id: &str, role_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/organization/groups/{}/roles/{}",
            encode_segment(group_id)?,
            encode_segment(role_id)?
        ),
    ))
}

/// Retrieves an organization role assigned to a group.
pub fn retrieve_group_role(group_id: &str, role_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/groups/{}/roles/{}",
            encode_segment(group_id)?,
            encode_segment(role_id)?
        ),
    ))
}

/// Lists the users assigned to a group.
pub fn list_group_users(group_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/organization/groups/{}/users", encode_segment(group_id)?),
    ))
}

/// Adds a user to a group.
pub fn add_group_user(group_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/organization/groups/{}/users", encode_segment(group_id)?),
    ))
}

/// Removes a user from a group.
pub fn remove_group_user(group_id: &str, user_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/organization/groups/{}/users/{}",
            encode_segment(group_id)?,
            encode_segment(user_id)?
        ),
    ))
}

/// Retrieves a user in a group.
pub fn retrieve_group_user(group_id: &str, user_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/groups/{}/users/{}",
            encode_segment(group_id)?,
            encode_segment(user_id)?
        ),
    ))
}

/// Returns a list of invites in the organization.
pub fn list_invites() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/invites")
}

/// Create an invite for a user to the organization. The invite must be accepted by the user before they have access to the organization.
pub fn invite_user() -> Endpoint {
    Endpoint::new(Method::POST, "/organization/invites")
}

/// Delete an invite. If the invite has already been accepted, it cannot be deleted.
pub fn delete_invite(invite_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/organization/invites/{}", encode_segment(invite_id)?),
    ))
}

/// Retrieves an invite.
pub fn retrieve_invite(invite_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/organization/invites/{}", encode_segment(invite_id)?),
    ))
}

/// Returns a list of projects.
pub fn list_projects() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/projects")
}

/// Create a new project in the organization. Projects can be created and archived, but cannot be deleted.
pub fn create_project() -> Endpoint {
    Endpoint::new(Method::POST, "/organization/projects")
}

/// Retrieves a project.
pub fn retrieve_project(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/organization/projects/{}", encode_segment(project_id)?),
    ))
}

/// Modifies a project in the organization.
pub fn modify_project(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/organization/projects/{}", encode_segment(project_id)?),
    ))
}

/// Returns a list of API keys in the project.
pub fn list_project_api_keys(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/api_keys",
            encode_segment(project_id)?
        ),
    ))
}

/// Deletes an API key from the project. Returns confirmation of the key deletion, or an error if the key belonged to a service account.
pub fn delete_project_api_key(project_id: &str, api_key_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/organization/projects/{}/api_keys/{}",
            encode_segment(project_id)?,
            encode_segment(api_key_id)?
        ),
    ))
}

/// Retrieves an API key in the project.
pub fn retrieve_project_api_key(project_id: &str, api_key_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/api_keys/{}",
            encode_segment(project_id)?,
            encode_segment(api_key_id)?
        ),
    ))
}

/// Updates the scopes and/or expiration of an existing API key in the project, including a service-account-owned key. Requires permission to write project API keys.
pub fn admin_organization_projects_api_keys_update(
    project_id: &str,
    api_key_id: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/api_keys/{}",
            encode_segment(project_id)?,
            encode_segment(api_key_id)?
        ),
    ))
}

/// Archives a project in the organization. Archived projects cannot be used or updated.
pub fn archive_project(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/archive",
            encode_segment(project_id)?
        ),
    ))
}

/// List certificates for this project.
pub fn list_project_certificates(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/certificates",
            encode_segment(project_id)?
        ),
    ))
}

/// Activate certificates at the project level. You can atomically and idempotently activate up to 10 certificates at a time.
pub fn activate_project_certificates(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/certificates/activate",
            encode_segment(project_id)?
        ),
    ))
}

/// Deactivate certificates at the project level. You can atomically and idempotently deactivate up to 10 certificates at a time.
pub fn deactivate_project_certificates(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/certificates/deactivate",
            encode_segment(project_id)?
        ),
    ))
}

/// Retrieves project data retention controls.
pub fn retrieve_project_data_retention(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/data_retention",
            encode_segment(project_id)?
        ),
    ))
}

/// Updates project data retention controls.
pub fn update_project_data_retention(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/data_retention",
            encode_segment(project_id)?
        ),
    ))
}

/// Lists the groups that have access to a project.
pub fn list_project_groups(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/groups",
            encode_segment(project_id)?
        ),
    ))
}

/// Grants a group access to a project.
pub fn add_project_group(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/groups",
            encode_segment(project_id)?
        ),
    ))
}

/// Revokes a group's access to a project.
pub fn remove_project_group(project_id: &str, group_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/organization/projects/{}/groups/{}",
            encode_segment(project_id)?,
            encode_segment(group_id)?
        ),
    ))
}

/// Retrieves a project's group.
pub fn retrieve_project_group(project_id: &str, group_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/groups/{}",
            encode_segment(project_id)?,
            encode_segment(group_id)?
        ),
    ))
}

/// Returns hosted tool permissions for a project.
pub fn retrieve_project_hosted_tool_permissions(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/hosted_tool_permissions",
            encode_segment(project_id)?
        ),
    ))
}

/// Updates hosted tool permissions for a project.
pub fn update_project_hosted_tool_permissions(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/hosted_tool_permissions",
            encode_segment(project_id)?
        ),
    ))
}

/// Deletes model permissions for a project.
pub fn delete_project_model_permissions(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/organization/projects/{}/model_permissions",
            encode_segment(project_id)?
        ),
    ))
}

/// Returns model permissions for a project.
pub fn retrieve_project_model_permissions(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/model_permissions",
            encode_segment(project_id)?
        ),
    ))
}

/// Updates model permissions for a project.
pub fn update_project_model_permissions(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/model_permissions",
            encode_segment(project_id)?
        ),
    ))
}

/// Returns the rate limits per model for a project.
pub fn list_project_rate_limits(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/rate_limits",
            encode_segment(project_id)?
        ),
    ))
}

/// Updates a project rate limit.
pub fn update_project_rate_limits(project_id: &str, rate_limit_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/rate_limits/{}",
            encode_segment(project_id)?,
            encode_segment(rate_limit_id)?
        ),
    ))
}

/// Returns a list of service accounts in the project.
pub fn list_project_service_accounts(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/service_accounts",
            encode_segment(project_id)?
        ),
    ))
}

/// Creates a new service account in the project. By default, this also returns an unredacted API key for the service account.
pub fn create_project_service_account(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/service_accounts",
            encode_segment(project_id)?
        ),
    ))
}

/// Deletes a service account from the project. Returns confirmation of service account deletion, or an error if the project is archived (archived projects have no service accounts).
pub fn delete_project_service_account(
    project_id: &str,
    service_account_id: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/organization/projects/{}/service_accounts/{}",
            encode_segment(project_id)?,
            encode_segment(service_account_id)?
        ),
    ))
}

/// Retrieves a service account in the project.
pub fn retrieve_project_service_account(
    project_id: &str,
    service_account_id: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/service_accounts/{}",
            encode_segment(project_id)?,
            encode_segment(service_account_id)?
        ),
    ))
}

/// Updates a service account in the project.
pub fn update_project_service_account(
    project_id: &str,
    service_account_id: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/service_accounts/{}",
            encode_segment(project_id)?,
            encode_segment(service_account_id)?
        ),
    ))
}

/// Creates an API key for a service account in the project.
pub fn create_service_account_api_key(
    project_id: &str,
    service_account_id: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/service_accounts/{}/api_keys",
            encode_segment(project_id)?,
            encode_segment(service_account_id)?
        ),
    ))
}

/// Lists project spend alerts.
pub fn list_project_spend_alerts(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/spend_alerts",
            encode_segment(project_id)?
        ),
    ))
}

/// Creates a project spend alert.
pub fn create_project_spend_alert(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/spend_alerts",
            encode_segment(project_id)?
        ),
    ))
}

/// Deletes a project spend alert.
pub fn delete_project_spend_alert(project_id: &str, alert_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/organization/projects/{}/spend_alerts/{}",
            encode_segment(project_id)?,
            encode_segment(alert_id)?
        ),
    ))
}

/// Retrieves a project spend alert.
pub fn retrieve_project_spend_alert(project_id: &str, alert_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/spend_alerts/{}",
            encode_segment(project_id)?,
            encode_segment(alert_id)?
        ),
    ))
}

/// Updates a project spend alert.
pub fn update_project_spend_alert(project_id: &str, alert_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/spend_alerts/{}",
            encode_segment(project_id)?,
            encode_segment(alert_id)?
        ),
    ))
}

/// Delete a project's hard spend limit.
pub fn delete_project_spend_limit(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/organization/projects/{}/spend_limit",
            encode_segment(project_id)?
        ),
    ))
}

/// Get a project's hard spend limit.
pub fn get_project_spend_limit(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/spend_limit",
            encode_segment(project_id)?
        ),
    ))
}

/// Create or replace a project's hard spend limit.
pub fn update_project_spend_limit(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/spend_limit",
            encode_segment(project_id)?
        ),
    ))
}

/// Returns a list of users in the project.
pub fn list_project_users(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/users",
            encode_segment(project_id)?
        ),
    ))
}

/// Adds a user to the project. Users must already be members of the organization to be added to a project.
pub fn create_project_user(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/users",
            encode_segment(project_id)?
        ),
    ))
}

/// Deletes a user from the project. Returns confirmation of project user deletion, or an error if the project is archived (archived projects have no users).
pub fn delete_project_user(project_id: &str, user_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/organization/projects/{}/users/{}",
            encode_segment(project_id)?,
            encode_segment(user_id)?
        ),
    ))
}

/// Retrieves a user in the project.
pub fn retrieve_project_user(project_id: &str, user_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/projects/{}/users/{}",
            encode_segment(project_id)?,
            encode_segment(user_id)?
        ),
    ))
}

/// Modifies a user's role in the project.
pub fn modify_project_user(project_id: &str, user_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/organization/projects/{}/users/{}",
            encode_segment(project_id)?,
            encode_segment(user_id)?
        ),
    ))
}

/// Lists the roles configured for the organization.
pub fn list_roles() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/roles")
}

/// Creates a custom role for the organization.
pub fn create_role() -> Endpoint {
    Endpoint::new(Method::POST, "/organization/roles")
}

/// Deletes a custom role from the organization.
pub fn delete_role(role_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/organization/roles/{}", encode_segment(role_id)?),
    ))
}

/// Retrieves an organization role.
pub fn retrieve_role(role_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/organization/roles/{}", encode_segment(role_id)?),
    ))
}

/// Updates an existing organization role.
pub fn update_role(role_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/organization/roles/{}", encode_segment(role_id)?),
    ))
}

/// Lists organization spend alerts.
pub fn list_organization_spend_alerts() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/spend_alerts")
}

/// Creates an organization spend alert.
pub fn create_organization_spend_alert() -> Endpoint {
    Endpoint::new(Method::POST, "/organization/spend_alerts")
}

/// Deletes an organization spend alert.
pub fn delete_organization_spend_alert(alert_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/organization/spend_alerts/{}", encode_segment(alert_id)?),
    ))
}

/// Retrieves an organization spend alert.
pub fn retrieve_organization_spend_alert(alert_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/organization/spend_alerts/{}", encode_segment(alert_id)?),
    ))
}

/// Updates an organization spend alert.
pub fn update_organization_spend_alert(alert_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/organization/spend_alerts/{}", encode_segment(alert_id)?),
    ))
}

/// Delete the organization's hard spend limit.
pub fn delete_organization_spend_limit() -> Endpoint {
    Endpoint::new(Method::DELETE, "/organization/spend_limit")
}

/// Get the organization's hard spend limit.
pub fn get_organization_spend_limit() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/spend_limit")
}

/// Create or replace the organization's hard spend limit.
pub fn update_organization_spend_limit() -> Endpoint {
    Endpoint::new(Method::POST, "/organization/spend_limit")
}

/// Get audio speeches usage details for the organization.
pub fn usage_audio_speeches() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/usage/audio_speeches")
}

/// Get audio transcriptions usage details for the organization.
pub fn usage_audio_transcriptions() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/usage/audio_transcriptions")
}

/// Get code interpreter sessions usage details for the organization.
pub fn usage_code_interpreter_sessions() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/usage/code_interpreter_sessions")
}

/// Get completions usage details for the organization.
pub fn usage_completions() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/usage/completions")
}

/// Get Decisions API usage details for the organization.
pub fn admin_organization_usage_decisions() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/usage/decisions")
}

/// Get embeddings usage details for the organization.
pub fn usage_embeddings() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/usage/embeddings")
}

/// Get file search calls usage details for the organization.
pub fn usage_file_search_calls() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/usage/file_search_calls")
}

/// Get images usage details for the organization.
pub fn usage_images() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/usage/images")
}

/// Get moderations usage details for the organization.
pub fn usage_moderations() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/usage/moderations")
}

/// Get vector stores usage details for the organization.
pub fn usage_vector_stores() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/usage/vector_stores")
}

/// Get web search calls usage details for the organization.
pub fn usage_web_search_calls() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/usage/web_search_calls")
}

/// Lists all of the users in the organization.
pub fn list_users() -> Endpoint {
    Endpoint::new(Method::GET, "/organization/users")
}

/// Deletes a user from the organization.
pub fn delete_user(user_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/organization/users/{}", encode_segment(user_id)?),
    ))
}

/// Retrieves a user by their identifier.
pub fn retrieve_user(user_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/organization/users/{}", encode_segment(user_id)?),
    ))
}

/// Modifies a user's role in the organization.
pub fn modify_user(user_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/organization/users/{}", encode_segment(user_id)?),
    ))
}

/// Lists the organization roles assigned to a user within the organization.
pub fn list_user_role_assignments(user_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/organization/users/{}/roles", encode_segment(user_id)?),
    ))
}

/// Assigns an organization role to a user within the organization.
pub fn assign_user_role(user_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/organization/users/{}/roles", encode_segment(user_id)?),
    ))
}

/// Unassigns an organization role from a user within the organization.
pub fn unassign_user_role(user_id: &str, role_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/organization/users/{}/roles/{}",
            encode_segment(user_id)?,
            encode_segment(role_id)?
        ),
    ))
}

/// Retrieves an organization role assigned to a user.
pub fn retrieve_user_role(user_id: &str, role_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/organization/users/{}/roles/{}",
            encode_segment(user_id)?,
            encode_segment(role_id)?
        ),
    ))
}

/// Lists the project roles assigned to a group within a project.
pub fn list_project_group_role_assignments(project_id: &str, group_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/projects/{}/groups/{}/roles",
            encode_segment(project_id)?,
            encode_segment(group_id)?
        ),
    ))
}

/// Assigns a project role to a group within a project.
pub fn assign_project_group_role(project_id: &str, group_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/projects/{}/groups/{}/roles",
            encode_segment(project_id)?,
            encode_segment(group_id)?
        ),
    ))
}

/// Unassigns a project role from a group within a project.
pub fn unassign_project_group_role(
    project_id: &str,
    group_id: &str,
    role_id: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/projects/{}/groups/{}/roles/{}",
            encode_segment(project_id)?,
            encode_segment(group_id)?,
            encode_segment(role_id)?
        ),
    ))
}

/// Retrieves a project role assigned to a group.
pub fn retrieve_project_group_role(
    project_id: &str,
    group_id: &str,
    role_id: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/projects/{}/groups/{}/roles/{}",
            encode_segment(project_id)?,
            encode_segment(group_id)?,
            encode_segment(role_id)?
        ),
    ))
}

/// Lists the roles configured for a project.
pub fn list_project_roles(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/projects/{}/roles", encode_segment(project_id)?),
    ))
}

/// Creates a custom role for a project.
pub fn create_project_role(project_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/projects/{}/roles", encode_segment(project_id)?),
    ))
}

/// Deletes a custom role from a project.
pub fn delete_project_role(project_id: &str, role_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/projects/{}/roles/{}",
            encode_segment(project_id)?,
            encode_segment(role_id)?
        ),
    ))
}

/// Retrieves a project role.
pub fn retrieve_project_role(project_id: &str, role_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/projects/{}/roles/{}",
            encode_segment(project_id)?,
            encode_segment(role_id)?
        ),
    ))
}

/// Updates an existing project role.
pub fn update_project_role(project_id: &str, role_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/projects/{}/roles/{}",
            encode_segment(project_id)?,
            encode_segment(role_id)?
        ),
    ))
}

/// Lists the project roles assigned to a user within a project.
pub fn list_project_user_role_assignments(project_id: &str, user_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/projects/{}/users/{}/roles",
            encode_segment(project_id)?,
            encode_segment(user_id)?
        ),
    ))
}

/// Assigns a project role to a user within a project.
pub fn assign_project_user_role(project_id: &str, user_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/projects/{}/users/{}/roles",
            encode_segment(project_id)?,
            encode_segment(user_id)?
        ),
    ))
}

/// Unassigns a project role from a user within a project.
pub fn unassign_project_user_role(
    project_id: &str,
    user_id: &str,
    role_id: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/projects/{}/users/{}/roles/{}",
            encode_segment(project_id)?,
            encode_segment(user_id)?,
            encode_segment(role_id)?
        ),
    ))
}

/// Retrieves a project role assigned to a user.
pub fn retrieve_project_user_role(
    project_id: &str,
    user_id: &str,
    role_id: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/projects/{}/users/{}/roles/{}",
            encode_segment(project_id)?,
            encode_segment(user_id)?,
            encode_segment(role_id)?
        ),
    ))
}

/// Create a new Realtime API call over WebRTC and receive the SDP answer needed to complete the peer connection.
pub fn create_realtime_call() -> Endpoint {
    Endpoint::new(Method::POST, "/realtime/calls")
}

/// Accept an incoming SIP call and configure the realtime session that will handle it.
pub fn accept_realtime_call(call_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/realtime/calls/{}/accept", encode_segment(call_id)?),
    ))
}

/// End an active Realtime API call, whether it was initiated over SIP or WebRTC.
pub fn hangup_realtime_call(call_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/realtime/calls/{}/hangup", encode_segment(call_id)?),
    ))
}

/// Transfer an active SIP call to a new destination using the SIP REFER verb.
pub fn refer_realtime_call(call_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/realtime/calls/{}/refer", encode_segment(call_id)?),
    ))
}

/// Decline an incoming SIP call by returning a SIP status code to the caller.
pub fn reject_realtime_call(call_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/realtime/calls/{}/reject", encode_segment(call_id)?),
    ))
}

/// Create a short-lived client secret for a Realtime session.
pub fn create_realtime_client_secret() -> Endpoint {
    Endpoint::new(Method::POST, "/realtime/client_secrets")
}

/// Create a short-lived client secret for a Realtime translation session.
pub fn create_realtime_translation_client_secret() -> Endpoint {
    Endpoint::new(Method::POST, "/realtime/translations/client_secrets")
}

/// Create a response from text or multimodal input, optionally streamed.
pub fn create_response() -> Endpoint {
    Endpoint::new(Method::POST, "/responses")
}

/// Compact a conversation into a response for subsequent requests.
pub fn compact_response() -> Endpoint {
    Endpoint::new(Method::POST, "/responses/compact")
}

/// Returns input token counts of the request. Returns an object with `object` set to `response.input_tokens` and an `input_tokens` count.
pub fn count_response_input_tokens() -> Endpoint {
    Endpoint::new(Method::POST, "/responses/input_tokens")
}

/// Deletes a model response with the given ID.
pub fn delete_response(response_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/responses/{}", encode_segment(response_id)?),
    ))
}

/// Retrieves a model response with the given ID.
pub fn get_response(response_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/responses/{}", encode_segment(response_id)?),
    ))
}

/// Cancels a model response with the given ID. Only responses created with the `background` parameter set to `true` can be cancelled. [Learn more](https://developers.openai.com/api/docs/guides/background).
pub fn cancel_response(response_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/responses/{}/cancel", encode_segment(response_id)?),
    ))
}

/// Returns a list of input items for a given response.
pub fn list_input_items(response_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/responses/{}/input_items", encode_segment(response_id)?),
    ))
}

/// Get a safety alert belonging to the authenticated API project.
pub fn get_project_safety_alert(id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/safety/alerts/{}", encode_segment(id)?),
    ))
}

/// Get a safety case by ID.
pub fn safety_cases_retrieve(id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/safety/cases/{}", encode_segment(id)?),
    ))
}

/// List all skills for the current project.
pub fn list_skills() -> Endpoint {
    Endpoint::new(Method::GET, "/skills")
}

/// Create a new skill.
pub fn create_skill() -> Endpoint {
    Endpoint::new(Method::POST, "/skills")
}

/// Delete a skill by its ID.
pub fn delete_skill(skill_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/skills/{}", encode_segment(skill_id)?),
    ))
}

/// Get a skill by its ID.
pub fn get_skill(skill_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/skills/{}", encode_segment(skill_id)?),
    ))
}

/// Update the default version pointer for a skill.
pub fn update_skill_default_version(skill_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/skills/{}", encode_segment(skill_id)?),
    ))
}

/// Download a skill zip bundle by its ID.
pub fn get_skill_content(skill_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/skills/{}/content", encode_segment(skill_id)?),
    ))
}

/// List skill versions for a skill.
pub fn list_skill_versions(skill_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/skills/{}/versions", encode_segment(skill_id)?),
    ))
}

/// Create a new immutable skill version.
pub fn create_skill_version(skill_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/skills/{}/versions", encode_segment(skill_id)?),
    ))
}

/// Delete a skill version.
pub fn delete_skill_version(skill_id: &str, version: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/skills/{}/versions/{}",
            encode_segment(skill_id)?,
            encode_segment(version)?
        ),
    ))
}

/// Get a specific skill version.
pub fn get_skill_version(skill_id: &str, version: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/skills/{}/versions/{}",
            encode_segment(skill_id)?,
            encode_segment(version)?
        ),
    ))
}

/// Download a skill version zip bundle.
pub fn get_skill_version_content(skill_id: &str, version: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/skills/{}/versions/{}/content",
            encode_segment(skill_id)?,
            encode_segment(version)?
        ),
    ))
}

/// Start a multipart upload and add parts before completing it.
pub fn create_upload() -> Endpoint {
    Endpoint::new(Method::POST, "/uploads")
}

/// Cancels the Upload. No Parts may be added after an Upload is cancelled. Returns the Upload object with status `cancelled`.
pub fn cancel_upload(upload_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/uploads/{}/cancel", encode_segment(upload_id)?),
    ))
}

/// Complete an upload and create its associated file.
pub fn complete_upload(upload_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/uploads/{}/complete", encode_segment(upload_id)?),
    ))
}

/// Add a binary part to an upload.
pub fn add_upload_part(upload_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/uploads/{}/parts", encode_segment(upload_id)?),
    ))
}

/// Returns a list of vector stores.
pub fn list_vector_stores() -> Endpoint {
    Endpoint::new(Method::GET, "/vector_stores")
}

/// Create a vector store.
pub fn create_vector_store() -> Endpoint {
    Endpoint::new(Method::POST, "/vector_stores")
}

/// Delete a vector store.
pub fn delete_vector_store(vector_store_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!("/vector_stores/{}", encode_segment(vector_store_id)?),
    ))
}

/// Retrieves a vector store.
pub fn get_vector_store(vector_store_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/vector_stores/{}", encode_segment(vector_store_id)?),
    ))
}

/// Modifies a vector store.
pub fn modify_vector_store(vector_store_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/vector_stores/{}", encode_segment(vector_store_id)?),
    ))
}

/// Create a vector store file batch.
pub fn create_vector_store_file_batch(vector_store_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/vector_stores/{}/file_batches",
            encode_segment(vector_store_id)?
        ),
    ))
}

/// Retrieves a vector store file batch.
pub fn get_vector_store_file_batch(vector_store_id: &str, batch_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/vector_stores/{}/file_batches/{}",
            encode_segment(vector_store_id)?,
            encode_segment(batch_id)?
        ),
    ))
}

/// Cancel a vector store file batch. This attempts to cancel the processing of files in this batch as soon as possible.
pub fn cancel_vector_store_file_batch(vector_store_id: &str, batch_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/vector_stores/{}/file_batches/{}/cancel",
            encode_segment(vector_store_id)?,
            encode_segment(batch_id)?
        ),
    ))
}

/// Returns a list of vector store files in a batch.
pub fn list_files_in_vector_store_batch(vector_store_id: &str, batch_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/vector_stores/{}/file_batches/{}/files",
            encode_segment(vector_store_id)?,
            encode_segment(batch_id)?
        ),
    ))
}

/// Returns a list of vector store files.
pub fn list_vector_store_files(vector_store_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!("/vector_stores/{}/files", encode_segment(vector_store_id)?),
    ))
}

/// Create a vector store file by attaching a [File](https://developers.openai.com/api/reference/resources/files) to a [vector store](https://developers.openai.com/api/reference/resources/vector_stores).
pub fn create_vector_store_file(vector_store_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/vector_stores/{}/files", encode_segment(vector_store_id)?),
    ))
}

/// Delete a vector store file. This will remove the file from the vector store but the file itself will not be deleted. To delete the file, use the [delete file](https://developers.openai.com/api/reference/resources/files/methods/delete) endpoint.
pub fn delete_vector_store_file(vector_store_id: &str, file_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/vector_stores/{}/files/{}",
            encode_segment(vector_store_id)?,
            encode_segment(file_id)?
        ),
    ))
}

/// Retrieves a vector store file.
pub fn get_vector_store_file(vector_store_id: &str, file_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/vector_stores/{}/files/{}",
            encode_segment(vector_store_id)?,
            encode_segment(file_id)?
        ),
    ))
}

/// Update attributes on a vector store file.
pub fn update_vector_store_file_attributes(
    vector_store_id: &str,
    file_id: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/vector_stores/{}/files/{}",
            encode_segment(vector_store_id)?,
            encode_segment(file_id)?
        ),
    ))
}

/// Retrieve the parsed contents of a vector store file.
pub fn retrieve_vector_store_file_content(
    vector_store_id: &str,
    file_id: &str,
) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/vector_stores/{}/files/{}/content",
            encode_segment(vector_store_id)?,
            encode_segment(file_id)?
        ),
    ))
}

/// Search a vector store for relevant chunks based on a query and file attributes filter.
pub fn search_vector_store(vector_store_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!("/vector_stores/{}/search", encode_segment(vector_store_id)?),
    ))
}

/// Creates a webhook endpoint for the authenticated project.
pub fn webhooks_create() -> Endpoint {
    Endpoint::new(Method::POST, "/webhook_endpoints")
}

/// Deletes a webhook endpoint for the authenticated project.
pub fn webhooks_delete(webhook_endpoint_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::DELETE,
        format!(
            "/webhook_endpoints/{}",
            encode_segment(webhook_endpoint_id)?
        ),
    ))
}

/// Retrieves a webhook endpoint for the authenticated project.
pub fn webhooks_retrieve(webhook_endpoint_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::GET,
        format!(
            "/webhook_endpoints/{}",
            encode_segment(webhook_endpoint_id)?
        ),
    ))
}

/// Updates a webhook endpoint for the authenticated project.
pub fn webhooks_update(webhook_endpoint_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/webhook_endpoints/{}",
            encode_segment(webhook_endpoint_id)?
        ),
    ))
}

/// Rotates the signing secret for a webhook endpoint in the authenticated project.
pub fn webhooks_rotate_secret(webhook_endpoint_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/webhook_endpoints/{}/rotate_secret",
            encode_segment(webhook_endpoint_id)?
        ),
    ))
}

/// Sends a sample event to a webhook endpoint for the authenticated project.
pub fn webhooks_test(webhook_endpoint_id: &str) -> Result<Endpoint> {
    Ok(Endpoint::new(
        Method::POST,
        format!(
            "/webhook_endpoints/{}/test",
            encode_segment(webhook_endpoint_id)?
        ),
    ))
}

/// Returns webhook event types visible to the authenticated project.
pub fn webhooks_event_types_list() -> Endpoint {
    Endpoint::new(Method::GET, "/webhook_event_types")
}
