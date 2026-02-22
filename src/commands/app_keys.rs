use anyhow::Result;
#[cfg(not(target_arch = "wasm32"))]
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
#[cfg(not(target_arch = "wasm32"))]
use datadog_api_client::datadogV2::api_key_management::{
    GetApplicationKeyOptionalParams, KeyManagementAPI, ListApplicationKeysOptionalParams,
};

#[cfg(not(target_arch = "wasm32"))]
use crate::client;
use crate::config::Config;
use crate::formatter;

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
pub async fn list(cfg: &Config) -> Result<()> {
    let data = crate::api::get(cfg, "/api/v2/application_keys", &[]).await?;
    crate::formatter::output(cfg, &data)
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
pub async fn get(cfg: &Config, key_id: &str) -> Result<()> {
    let data = crate::api::get(cfg, &format!("/api/v2/application_keys/{key_id}"), &[]).await?;
    crate::formatter::output(cfg, &data)
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
pub async fn register(cfg: &Config, key_id: &str) -> Result<()> {
    let body = serde_json::json!({});
    let data = crate::api::post(
        cfg,
        &format!("/api/v2/action_connections/app_keys/{key_id}/register"),
        &body,
    )
    .await?;
    crate::formatter::output(cfg, &data)
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
pub async fn unregister(cfg: &Config, key_id: &str) -> Result<()> {
    crate::api::delete(
        cfg,
        &format!("/api/v2/action_connections/app_keys/{key_id}/unregister"),
    )
    .await?;
    println!("App key {key_id} unregistered.");
    Ok(())
}
