use anyhow::Result;
use datadog_api_client::datadogV2::api_fleet_automation::{
    FleetAutomationAPI, ListFleetAgentsOptionalParams, ListFleetDeploymentsOptionalParams,
    GetFleetDeploymentOptionalParams,
};

use crate::client;
use crate::config::Config;
use crate::formatter;

pub async fn agents_list(cfg: &Config, page_size: Option<i64>) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => FleetAutomationAPI::with_client_and_config(dd_cfg, c),
        None => FleetAutomationAPI::with_config(dd_cfg),
    };
    let mut params = ListFleetAgentsOptionalParams::default();
    if let Some(ps) = page_size {
        params = params.page_size(ps);
    }
    let resp = api
        .list_fleet_agents(params)
        .await
        .map_err(|e| anyhow::anyhow!("failed to list fleet agents: {e:?}"))?;
    formatter::print_json(&resp)
}

pub async fn agents_get(cfg: &Config, agent_key: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => FleetAutomationAPI::with_client_and_config(dd_cfg, c),
        None => FleetAutomationAPI::with_config(dd_cfg),
    };
    let resp = api
        .get_fleet_agent_info(agent_key.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to get fleet agent: {e:?}"))?;
    formatter::print_json(&resp)
}

pub async fn agents_versions(cfg: &Config) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => FleetAutomationAPI::with_client_and_config(dd_cfg, c),
        None => FleetAutomationAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_fleet_agent_versions()
        .await
        .map_err(|e| anyhow::anyhow!("failed to list agent versions: {e:?}"))?;
    formatter::print_json(&resp)
}

pub async fn deployments_list(cfg: &Config, page_size: Option<i64>) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => FleetAutomationAPI::with_client_and_config(dd_cfg, c),
        None => FleetAutomationAPI::with_config(dd_cfg),
    };
    let mut params = ListFleetDeploymentsOptionalParams::default();
    if let Some(ps) = page_size {
        params = params.page_size(ps);
    }
    let resp = api
        .list_fleet_deployments(params)
        .await
        .map_err(|e| anyhow::anyhow!("failed to list deployments: {e:?}"))?;
    formatter::print_json(&resp)
}

pub async fn deployments_get(cfg: &Config, deployment_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => FleetAutomationAPI::with_client_and_config(dd_cfg, c),
        None => FleetAutomationAPI::with_config(dd_cfg),
    };
    let resp = api
        .get_fleet_deployment(deployment_id.to_string(), GetFleetDeploymentOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to get deployment: {e:?}"))?;
    formatter::print_json(&resp)
}

pub async fn schedules_list(cfg: &Config) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => FleetAutomationAPI::with_client_and_config(dd_cfg, c),
        None => FleetAutomationAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_fleet_schedules()
        .await
        .map_err(|e| anyhow::anyhow!("failed to list schedules: {e:?}"))?;
    formatter::print_json(&resp)
}

pub async fn schedules_get(cfg: &Config, schedule_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => FleetAutomationAPI::with_client_and_config(dd_cfg, c),
        None => FleetAutomationAPI::with_config(dd_cfg),
    };
    let resp = api
        .get_fleet_schedule(schedule_id.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to get schedule: {e:?}"))?;
    formatter::print_json(&resp)
}
