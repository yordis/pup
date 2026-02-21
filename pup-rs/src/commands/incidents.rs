use anyhow::Result;
use datadog_api_client::datadogV2::api_incidents::{
    GetIncidentOptionalParams, IncidentsAPI, ListIncidentsOptionalParams,
};

use crate::client;
use crate::config::Config;
use crate::formatter;

pub async fn list(cfg: &Config, limit: i64) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);

    let api = if let Some(http_client) = client::make_bearer_client(cfg) {
        IncidentsAPI::with_client_and_config(dd_cfg, http_client)
    } else {
        IncidentsAPI::with_config(dd_cfg)
    };

    let params = ListIncidentsOptionalParams::default().page_size(limit);

    let resp = api
        .list_incidents(params)
        .await
        .map_err(|e| anyhow::anyhow!("failed to list incidents: {:?}", e))?;

    formatter::output(cfg, &resp)?;
    Ok(())
}

pub async fn get(cfg: &Config, incident_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = if let Some(http_client) = client::make_bearer_client(cfg) {
        IncidentsAPI::with_client_and_config(dd_cfg, http_client)
    } else {
        IncidentsAPI::with_config(dd_cfg)
    };
    let resp = api
        .get_incident(
            incident_id.to_string(),
            GetIncidentOptionalParams::default(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to get incident: {:?}", e))?;
    formatter::output(cfg, &resp)
}
