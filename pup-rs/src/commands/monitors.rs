use anyhow::Result;
use datadog_api_client::datadogV1::api_monitors::{
    DeleteMonitorOptionalParams, GetMonitorOptionalParams, ListMonitorsOptionalParams, MonitorsAPI,
};

use crate::client;
use crate::config::Config;
use crate::formatter;

pub async fn list(
    cfg: &Config,
    name: Option<String>,
    tags: Option<String>,
    limit: i32,
) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);

    let api = if let Some(http_client) = client::make_bearer_client(cfg) {
        MonitorsAPI::with_client_and_config(dd_cfg, http_client)
    } else {
        MonitorsAPI::with_config(dd_cfg)
    };

    let mut params = ListMonitorsOptionalParams::default();
    if let Some(name) = name {
        params = params.name(name);
    }
    if let Some(tags) = tags {
        params = params.monitor_tags(tags);
    }

    let limit = limit.clamp(1, 1000);
    params = params.page_size(limit).page(0);

    let monitors = api
        .list_monitors(params)
        .await
        .map_err(|e| anyhow::anyhow!("failed to list monitors: {:?}", e))?;

    if monitors.is_empty() {
        eprintln!("No monitors found matching the specified criteria.");
        return Ok(());
    }

    // Truncate to requested limit
    let monitors: Vec<_> = monitors.into_iter().take(limit as usize).collect();

    formatter::output(cfg, &monitors)?;
    Ok(())
}

pub async fn get(cfg: &Config, monitor_id: i64) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = if let Some(http_client) = client::make_bearer_client(cfg) {
        MonitorsAPI::with_client_and_config(dd_cfg, http_client)
    } else {
        MonitorsAPI::with_config(dd_cfg)
    };
    let resp = api
        .get_monitor(monitor_id, GetMonitorOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to get monitor: {:?}", e))?;
    formatter::output(cfg, &resp)
}

pub async fn delete(cfg: &Config, monitor_id: i64) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = if let Some(http_client) = client::make_bearer_client(cfg) {
        MonitorsAPI::with_client_and_config(dd_cfg, http_client)
    } else {
        MonitorsAPI::with_config(dd_cfg)
    };
    let resp = api
        .delete_monitor(monitor_id, DeleteMonitorOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to delete monitor: {:?}", e))?;
    formatter::output(cfg, &resp)
}
