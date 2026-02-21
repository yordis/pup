use anyhow::{bail, Result};
use datadog_api_client::datadogV2::api_status_pages::{
    CreateComponentOptionalParams, CreateDegradationOptionalParams, CreateStatusPageOptionalParams,
    GetComponentOptionalParams, GetDegradationOptionalParams, GetStatusPageOptionalParams,
    ListComponentsOptionalParams, ListDegradationsOptionalParams, ListStatusPagesOptionalParams,
    StatusPagesAPI, UpdateComponentOptionalParams, UpdateDegradationOptionalParams,
    UpdateStatusPageOptionalParams,
};
use datadog_api_client::datadogV2::model::{
    CreateComponentRequest, CreateDegradationRequest, CreateStatusPageRequest,
    PatchComponentRequest, PatchDegradationRequest, PatchStatusPageRequest,
};
use uuid::Uuid;

use crate::client;
use crate::config::Config;
use crate::formatter;
use crate::util;

pub async fn pages_list(cfg: &Config) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_status_pages(ListStatusPagesOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to list status pages: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn pages_get(cfg: &Config, page_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let uuid = Uuid::parse_str(page_id)
        .map_err(|e| anyhow::anyhow!("invalid page UUID '{page_id}': {e}"))?;
    let resp = api
        .get_status_page(uuid, GetStatusPageOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to get status page: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn pages_delete(cfg: &Config, page_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let uuid = Uuid::parse_str(page_id)
        .map_err(|e| anyhow::anyhow!("invalid page UUID '{page_id}': {e}"))?;
    api.delete_status_page(uuid)
        .await
        .map_err(|e| anyhow::anyhow!("failed to delete status page: {e:?}"))?;
    println!("Status page {page_id} deleted.");
    Ok(())
}

pub async fn pages_update(cfg: &Config, page_id: &str, file: &str) -> Result<()> {
    let page_uuid = Uuid::parse_str(page_id)
        .map_err(|e| anyhow::anyhow!("invalid page UUID '{page_id}': {e}"))?;
    let body: PatchStatusPageRequest = util::read_json_file(file)?;
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let resp = api
        .update_status_page(page_uuid, body, UpdateStatusPageOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to update status page: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn components_list(cfg: &Config, page_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let uuid = Uuid::parse_str(page_id)
        .map_err(|e| anyhow::anyhow!("invalid page UUID '{page_id}': {e}"))?;
    let resp = api
        .list_components(uuid, ListComponentsOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to list components: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn components_get(cfg: &Config, page_id: &str, component_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let page_uuid = Uuid::parse_str(page_id)
        .map_err(|e| anyhow::anyhow!("invalid page UUID '{page_id}': {e}"))?;
    let component_uuid = Uuid::parse_str(component_id)
        .map_err(|e| anyhow::anyhow!("invalid component UUID '{component_id}': {e}"))?;
    let resp = api
        .get_component(
            page_uuid,
            component_uuid,
            GetComponentOptionalParams::default(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to get component: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn components_update(
    cfg: &Config,
    page_id: &str,
    component_id: &str,
    file: &str,
) -> Result<()> {
    let page_uuid = Uuid::parse_str(page_id)
        .map_err(|e| anyhow::anyhow!("invalid page UUID '{page_id}': {e}"))?;
    let component_uuid = Uuid::parse_str(component_id)
        .map_err(|e| anyhow::anyhow!("invalid component UUID '{component_id}': {e}"))?;
    let body: PatchComponentRequest = util::read_json_file(file)?;
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let resp = api
        .update_component(
            page_uuid,
            component_uuid,
            body,
            UpdateComponentOptionalParams::default(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to update component: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn degradations_list(cfg: &Config) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_degradations(ListDegradationsOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to list degradations: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn degradations_get(cfg: &Config, page_id: &str, degradation_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let page_uuid = Uuid::parse_str(page_id)
        .map_err(|e| anyhow::anyhow!("invalid page UUID '{page_id}': {e}"))?;
    let degradation_uuid = Uuid::parse_str(degradation_id)
        .map_err(|e| anyhow::anyhow!("invalid degradation UUID '{degradation_id}': {e}"))?;
    let resp = api
        .get_degradation(
            page_uuid,
            degradation_uuid,
            GetDegradationOptionalParams::default(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to get degradation: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn degradations_create(cfg: &Config, page_id: &str, file: &str) -> Result<()> {
    let page_uuid = Uuid::parse_str(page_id)
        .map_err(|e| anyhow::anyhow!("invalid page UUID '{page_id}': {e}"))?;
    let body: CreateDegradationRequest = util::read_json_file(file)?;
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let resp = api
        .create_degradation(page_uuid, body, CreateDegradationOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to create degradation: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn degradations_update(
    cfg: &Config,
    page_id: &str,
    degradation_id: &str,
    file: &str,
) -> Result<()> {
    let page_uuid = Uuid::parse_str(page_id)
        .map_err(|e| anyhow::anyhow!("invalid page UUID '{page_id}': {e}"))?;
    let degradation_uuid = Uuid::parse_str(degradation_id)
        .map_err(|e| anyhow::anyhow!("invalid degradation UUID '{degradation_id}': {e}"))?;
    let body: PatchDegradationRequest = util::read_json_file(file)?;
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let resp = api
        .update_degradation(
            page_uuid,
            degradation_uuid,
            body,
            UpdateDegradationOptionalParams::default(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to update degradation: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn components_delete(cfg: &Config, page_id: &str, component_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let page_uuid = Uuid::parse_str(page_id)
        .map_err(|e| anyhow::anyhow!("invalid page UUID '{page_id}': {e}"))?;
    let component_uuid = Uuid::parse_str(component_id)
        .map_err(|e| anyhow::anyhow!("invalid component UUID '{component_id}': {e}"))?;
    api.delete_component(page_uuid, component_uuid)
        .await
        .map_err(|e| anyhow::anyhow!("failed to delete component: {e:?}"))?;
    println!("Component {component_id} deleted from page {page_id}.");
    Ok(())
}

pub async fn degradations_delete(
    cfg: &Config,
    page_id: &str,
    degradation_id: &str,
) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let page_uuid = Uuid::parse_str(page_id)
        .map_err(|e| anyhow::anyhow!("invalid page UUID '{page_id}': {e}"))?;
    let degradation_uuid = Uuid::parse_str(degradation_id)
        .map_err(|e| anyhow::anyhow!("invalid degradation UUID '{degradation_id}': {e}"))?;
    api.delete_degradation(page_uuid, degradation_uuid)
        .await
        .map_err(|e| anyhow::anyhow!("failed to delete degradation: {e:?}"))?;
    println!("Degradation {degradation_id} deleted from page {page_id}.");
    Ok(())
}

// ---------------------------------------------------------------------------
// Pages create
// ---------------------------------------------------------------------------

pub async fn pages_create(cfg: &Config, file: &str) -> Result<()> {
    let body: CreateStatusPageRequest = util::read_json_file(file)?;
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let resp = api
        .create_status_page(body, CreateStatusPageOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to create status page: {e:?}"))?;
    formatter::output(cfg, &resp)
}

// ---------------------------------------------------------------------------
// Components create
// ---------------------------------------------------------------------------

pub async fn components_create(cfg: &Config, page_id: &str, file: &str) -> Result<()> {
    let page_uuid = Uuid::parse_str(page_id)
        .map_err(|e| anyhow::anyhow!("invalid page UUID '{page_id}': {e}"))?;
    let body: CreateComponentRequest = util::read_json_file(file)?;
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => StatusPagesAPI::with_client_and_config(dd_cfg, c),
        None => StatusPagesAPI::with_config(dd_cfg),
    };
    let resp = api
        .create_component(page_uuid, body, CreateComponentOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to create component: {e:?}"))?;
    formatter::output(cfg, &resp)
}

// ---------------------------------------------------------------------------
// Third-party status pages (fetched from updog.ai, no DD auth needed)
// ---------------------------------------------------------------------------

pub async fn third_party_list(cfg: &Config) -> Result<()> {
    let url = "https://updog.ai/data/third-party-outages.json";
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("Accept", "application/json")
        .send()
        .await?;
    if !resp.status().is_success() {
        let status = resp.status();
        bail!("failed to fetch third-party outages from updog.ai (HTTP {status})");
    }
    let data: serde_json::Value = resp.json().await?;
    formatter::output(cfg, &data)
}
