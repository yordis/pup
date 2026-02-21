use anyhow::{bail, Result};
use datadog_api_client::datadogV2::api_rum::{ListRUMEventsOptionalParams, RUMAPI};
use datadog_api_client::datadogV2::model::{
    RUMApplicationCreate, RUMApplicationCreateAttributes, RUMApplicationCreateRequest,
    RUMApplicationCreateType,
};

use crate::client;
use crate::config::Config;
use crate::formatter;
use crate::util;

pub async fn apps_list(cfg: &Config) -> Result<()> {
    // RUM apps is OAuth-excluded — require API keys
    if !cfg.has_api_keys() {
        bail!("RUM apps requires API key authentication (DD_API_KEY + DD_APP_KEY)");
    }
    let dd_cfg = client::make_dd_config(cfg);
    let api = RUMAPI::with_config(dd_cfg);
    let resp = api
        .get_rum_applications()
        .await
        .map_err(|e| anyhow::anyhow!("failed to list RUM apps: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn apps_get(cfg: &Config, app_id: &str) -> Result<()> {
    if !cfg.has_api_keys() {
        bail!("RUM apps requires API key authentication (DD_API_KEY + DD_APP_KEY)");
    }
    let dd_cfg = client::make_dd_config(cfg);
    let api = RUMAPI::with_config(dd_cfg);
    let resp = api
        .get_rum_application(app_id.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to get RUM app: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn apps_create(cfg: &Config, name: &str, app_type: Option<String>) -> Result<()> {
    if !cfg.has_api_keys() {
        bail!("RUM apps requires API key authentication (DD_API_KEY + DD_APP_KEY)");
    }
    let dd_cfg = client::make_dd_config(cfg);
    let api = RUMAPI::with_config(dd_cfg);
    let mut attrs = RUMApplicationCreateAttributes::new(name.to_string());
    if let Some(t) = app_type {
        attrs = attrs.type_(t);
    }
    let data = RUMApplicationCreate::new(attrs, RUMApplicationCreateType::RUM_APPLICATION_CREATE);
    let body = RUMApplicationCreateRequest::new(data);
    let resp = api
        .create_rum_application(body)
        .await
        .map_err(|e| anyhow::anyhow!("failed to create RUM app: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn apps_delete(cfg: &Config, app_id: &str) -> Result<()> {
    if !cfg.has_api_keys() {
        bail!("RUM apps requires API key authentication (DD_API_KEY + DD_APP_KEY)");
    }
    let dd_cfg = client::make_dd_config(cfg);
    let api = RUMAPI::with_config(dd_cfg);
    api.delete_rum_application(app_id.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to delete RUM app: {e:?}"))?;
    eprintln!("RUM application {app_id} deleted.");
    Ok(())
}

pub async fn events_list(cfg: &Config, from: String, to: String, limit: i32) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => RUMAPI::with_client_and_config(dd_cfg, c),
        None => RUMAPI::with_config(dd_cfg),
    };

    let from_dt =
        chrono::DateTime::from_timestamp_millis(util::parse_time_to_unix_millis(&from)?).unwrap();
    let to_dt =
        chrono::DateTime::from_timestamp_millis(util::parse_time_to_unix_millis(&to)?).unwrap();

    let params = ListRUMEventsOptionalParams::default()
        .filter_from(from_dt)
        .filter_to(to_dt)
        .page_limit(limit);

    let resp = api
        .list_rum_events(params)
        .await
        .map_err(|e| anyhow::anyhow!("failed to list RUM events: {e:?}"))?;
    formatter::output(cfg, &resp)
}
