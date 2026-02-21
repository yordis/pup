use anyhow::Result;
use datadog_api_client::datadogV2::api_jira_integration::JiraIntegrationAPI;
use datadog_api_client::datadogV2::api_service_now_integration::ServiceNowIntegrationAPI;
use uuid::Uuid;

use crate::client;
use crate::config::Config;
use crate::formatter;

// ---- Jira ----

pub async fn jira_accounts_list(cfg: &Config) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => JiraIntegrationAPI::with_client_and_config(dd_cfg, c),
        None => JiraIntegrationAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_jira_accounts()
        .await
        .map_err(|e| anyhow::anyhow!("failed to list Jira accounts: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn jira_templates_list(cfg: &Config) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => JiraIntegrationAPI::with_client_and_config(dd_cfg, c),
        None => JiraIntegrationAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_jira_issue_templates()
        .await
        .map_err(|e| anyhow::anyhow!("failed to list Jira templates: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn jira_templates_get(cfg: &Config, template_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => JiraIntegrationAPI::with_client_and_config(dd_cfg, c),
        None => JiraIntegrationAPI::with_config(dd_cfg),
    };
    let uuid = Uuid::parse_str(template_id)
        .map_err(|e| anyhow::anyhow!("invalid template UUID '{template_id}': {e}"))?;
    let resp = api
        .get_jira_issue_template(uuid)
        .await
        .map_err(|e| anyhow::anyhow!("failed to get Jira template: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn jira_accounts_delete(cfg: &Config, account_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => JiraIntegrationAPI::with_client_and_config(dd_cfg, c),
        None => JiraIntegrationAPI::with_config(dd_cfg),
    };
    let uuid = Uuid::parse_str(account_id)
        .map_err(|e| anyhow::anyhow!("invalid account UUID '{account_id}': {e}"))?;
    api.delete_jira_account(uuid)
        .await
        .map_err(|e| anyhow::anyhow!("failed to delete Jira account: {e:?}"))?;
    eprintln!("Jira account {account_id} deleted.");
    Ok(())
}

// ---- ServiceNow ----

pub async fn servicenow_instances_list(cfg: &Config) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => ServiceNowIntegrationAPI::with_client_and_config(dd_cfg, c),
        None => ServiceNowIntegrationAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_service_now_instances()
        .await
        .map_err(|e| anyhow::anyhow!("failed to list ServiceNow instances: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn servicenow_templates_list(cfg: &Config) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => ServiceNowIntegrationAPI::with_client_and_config(dd_cfg, c),
        None => ServiceNowIntegrationAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_service_now_templates()
        .await
        .map_err(|e| anyhow::anyhow!("failed to list ServiceNow templates: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn servicenow_templates_get(cfg: &Config, template_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => ServiceNowIntegrationAPI::with_client_and_config(dd_cfg, c),
        None => ServiceNowIntegrationAPI::with_config(dd_cfg),
    };
    let uuid = Uuid::parse_str(template_id)
        .map_err(|e| anyhow::anyhow!("invalid template UUID '{template_id}': {e}"))?;
    let resp = api
        .get_service_now_template(uuid)
        .await
        .map_err(|e| anyhow::anyhow!("failed to get ServiceNow template: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn servicenow_templates_delete(cfg: &Config, template_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => ServiceNowIntegrationAPI::with_client_and_config(dd_cfg, c),
        None => ServiceNowIntegrationAPI::with_config(dd_cfg),
    };
    let uuid = Uuid::parse_str(template_id)
        .map_err(|e| anyhow::anyhow!("invalid template UUID '{template_id}': {e}"))?;
    api.delete_service_now_template(uuid)
        .await
        .map_err(|e| anyhow::anyhow!("failed to delete ServiceNow template: {e:?}"))?;
    eprintln!("ServiceNow template {template_id} deleted.");
    Ok(())
}

pub async fn servicenow_users_list(cfg: &Config, instance_name: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => ServiceNowIntegrationAPI::with_client_and_config(dd_cfg, c),
        None => ServiceNowIntegrationAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_service_now_users(uuid::Uuid::parse_str(instance_name)
            .map_err(|e| anyhow::anyhow!("invalid instance UUID '{instance_name}': {e}"))?)
        .await
        .map_err(|e| anyhow::anyhow!("failed to list ServiceNow users: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn servicenow_assignment_groups_list(cfg: &Config, instance_name: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => ServiceNowIntegrationAPI::with_client_and_config(dd_cfg, c),
        None => ServiceNowIntegrationAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_service_now_assignment_groups(uuid::Uuid::parse_str(instance_name)
            .map_err(|e| anyhow::anyhow!("invalid instance UUID '{instance_name}': {e}"))?)
        .await
        .map_err(|e| anyhow::anyhow!("failed to list ServiceNow assignment groups: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn servicenow_business_services_list(cfg: &Config, instance_name: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => ServiceNowIntegrationAPI::with_client_and_config(dd_cfg, c),
        None => ServiceNowIntegrationAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_service_now_business_services(uuid::Uuid::parse_str(instance_name)
            .map_err(|e| anyhow::anyhow!("invalid instance UUID '{instance_name}': {e}"))?)
        .await
        .map_err(|e| anyhow::anyhow!("failed to list ServiceNow business services: {e:?}"))?;
    formatter::output(cfg, &resp)
}
