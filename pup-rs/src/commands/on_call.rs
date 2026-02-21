use anyhow::Result;
use datadog_api_client::datadogV2::api_teams::{
    GetTeamMembershipsOptionalParams, ListTeamsOptionalParams, TeamsAPI,
};
use datadog_api_client::datadogV2::model::{
    TeamCreate, TeamCreateAttributes, TeamCreateRequest, TeamType, TeamUpdate,
    TeamUpdateAttributes, TeamUpdateRequest,
};

use crate::client;
use crate::config::Config;
use crate::formatter;

pub async fn teams_list(cfg: &Config) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => TeamsAPI::with_client_and_config(dd_cfg, c),
        None => TeamsAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_teams(ListTeamsOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to list teams: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn teams_get(cfg: &Config, team_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => TeamsAPI::with_client_and_config(dd_cfg, c),
        None => TeamsAPI::with_config(dd_cfg),
    };
    let resp = api
        .get_team(team_id.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to get team: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn teams_delete(cfg: &Config, team_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => TeamsAPI::with_client_and_config(dd_cfg, c),
        None => TeamsAPI::with_config(dd_cfg),
    };
    api.delete_team(team_id.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to delete team: {e:?}"))?;
    eprintln!("Team {team_id} deleted.");
    Ok(())
}

pub async fn teams_create(cfg: &Config, name: &str, handle: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => TeamsAPI::with_client_and_config(dd_cfg, c),
        None => TeamsAPI::with_config(dd_cfg),
    };
    let attrs = TeamCreateAttributes::new(handle.to_string(), name.to_string());
    let data = TeamCreate::new(attrs, TeamType::TEAM);
    let body = TeamCreateRequest::new(data);
    let resp = api
        .create_team(body)
        .await
        .map_err(|e| anyhow::anyhow!("failed to create team: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn teams_update(cfg: &Config, team_id: &str, name: &str, handle: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => TeamsAPI::with_client_and_config(dd_cfg, c),
        None => TeamsAPI::with_config(dd_cfg),
    };
    let attrs = TeamUpdateAttributes::new(handle.to_string(), name.to_string());
    let data = TeamUpdate::new(attrs, TeamType::TEAM);
    let body = TeamUpdateRequest::new(data);
    let resp = api
        .update_team(team_id.to_string(), body)
        .await
        .map_err(|e| anyhow::anyhow!("failed to update team: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn memberships_list(cfg: &Config, team_id: &str, page_size: i64) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => TeamsAPI::with_client_and_config(dd_cfg, c),
        None => TeamsAPI::with_config(dd_cfg),
    };
    let params = GetTeamMembershipsOptionalParams::default().page_size(page_size);
    let resp = api
        .get_team_memberships(team_id.to_string(), params)
        .await
        .map_err(|e| anyhow::anyhow!("failed to list memberships: {e:?}"))?;
    formatter::output(cfg, &resp)
}
