use anyhow::{bail, Result};
use datadog_api_client::datadogV2::api_rum::{RUMAPI, ListRUMEventsOptionalParams};

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
    formatter::print_json(&resp)
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
    formatter::print_json(&resp)
}

pub async fn events_list(
    cfg: &Config,
    from: String,
    to: String,
    limit: i32,
) -> Result<()> {
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
    formatter::print_json(&resp)
}
