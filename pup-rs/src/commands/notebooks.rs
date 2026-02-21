use anyhow::Result;
use datadog_api_client::datadogV1::api_notebooks::{ListNotebooksOptionalParams, NotebooksAPI};
use datadog_api_client::datadogV1::model::NotebookCreateRequest;

use crate::client;
use crate::config::Config;
use crate::formatter;
use crate::util;

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

pub async fn delete(cfg: &Config, notebook_id: i64) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => NotebooksAPI::with_client_and_config(dd_cfg, c),
        None => NotebooksAPI::with_config(dd_cfg),
    };
    api.delete_notebook(notebook_id)
        .await
        .map_err(|e| anyhow::anyhow!("failed to delete notebook: {e:?}"))?;
    eprintln!("Notebook {notebook_id} deleted.");
    Ok(())
}

pub async fn create(cfg: &Config, file: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => NotebooksAPI::with_client_and_config(dd_cfg, c),
        None => NotebooksAPI::with_config(dd_cfg),
    };
    let body: NotebookCreateRequest = util::read_json_file(file)?;
    let resp = api
        .create_notebook(body)
        .await
        .map_err(|e| anyhow::anyhow!("failed to create notebook: {e:?}"))?;
    formatter::output(cfg, &resp)
}
