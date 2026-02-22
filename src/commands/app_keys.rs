use anyhow::Result;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
use datadog_api_client::datadogV2::api_key_management::{
    GetApplicationKeyOptionalParams, KeyManagementAPI, ListApplicationKeysOptionalParams,
};

use crate::client;
use crate::config::Config;
use crate::formatter;

pub async fn list(cfg: &Config) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => KeyManagementAPI::with_client_and_config(dd_cfg, c),
        None => KeyManagementAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_application_keys(ListApplicationKeysOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to list app keys: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn get(cfg: &Config, key_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => KeyManagementAPI::with_client_and_config(dd_cfg, c),
        None => KeyManagementAPI::with_config(dd_cfg),
    };
    let resp = api
        .get_application_key(
            key_id.to_string(),
            GetApplicationKeyOptionalParams::default(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("failed to get app key: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn register(cfg: &Config, key_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => ActionConnectionAPI::with_client_and_config(dd_cfg, c),
        None => ActionConnectionAPI::with_config(dd_cfg),
    };
    let resp = api
        .register_app_key(key_id.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to register app key: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn unregister(cfg: &Config, key_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => ActionConnectionAPI::with_client_and_config(dd_cfg, c),
        None => ActionConnectionAPI::with_config(dd_cfg),
    };
    api.unregister_app_key(key_id.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to unregister app key: {e:?}"))?;
    println!("App key {key_id} unregistered.");
    Ok(())
}
