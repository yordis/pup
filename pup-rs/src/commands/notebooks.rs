use anyhow::Result;
use datadog_api_client::datadogV1::api_notebooks::{
    NotebooksAPI, ListNotebooksOptionalParams,
};

use crate::client;
use crate::config::Config;
use crate::formatter;

pub async fn list(cfg: &Config) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => NotebooksAPI::with_client_and_config(dd_cfg, c),
        None => NotebooksAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_notebooks(ListNotebooksOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to list notebooks: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn get(cfg: &Config, notebook_id: i64) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => NotebooksAPI::with_client_and_config(dd_cfg, c),
        None => NotebooksAPI::with_config(dd_cfg),
    };
    let resp = api
        .get_notebook(notebook_id)
        .await
        .map_err(|e| anyhow::anyhow!("failed to get notebook: {e:?}"))?;
    formatter::output(cfg, &resp)
}
